use core::cell::Cell;
use kernel::debug;
use kernel::deferred_call::{DeferredCall, DeferredCallClient};
use kernel::hil::crc::{Client, Crc, CrcAlgorithm, CrcOutput};
use kernel::utilities::cells::OptionalCell;
use kernel::utilities::leasable_buffer::SubSliceMut;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{register_bitfields, register_structs, ReadWrite};
use kernel::utilities::StaticRef;
use kernel::ErrorCode;

register_structs! {
    pub CrcRegisters {
        /// Data register
        (0x00 => pub dr: ReadWrite<u32, DR::Register>),
        /// Independent data register
        (0x04 => pub idr: ReadWrite<u32, IDR::Register>),
        /// Control register
        (0x08 => pub cr: ReadWrite<u32, CR::Register>),
        /// Padding
        (0x0C => reserved),
        /// Initial value
        (0x10 => pub init: ReadWrite<u32, INIT::Register>),
        /// Polynomial
        (0x14 => pub pol: ReadWrite<u32, POL::Register>),
        (0x18 => @END),
    }
}

/// Base address for CRC in Secure Alias mode
pub const CRC_BASE: StaticRef<CrcRegisters> =
    unsafe { StaticRef::new(0x50023000 as *const CrcRegisters) };

register_bitfields![u32,
     pub DR [
        /// Data register
        DR OFFSET(0) NUMBITS(32) []
    ],
    pub IDR [
        /// Temporary 4 byte storage
        DR OFFSET(0) NUMBITS(32) []
    ],
    pub CR [
        /// Reset bit, used for initialising and resetting
        RESET OFFSET(0) NUMBITS(1) [],
        /// Polynomial size
        PSIZE OFFSET(3) NUMBITS(2) [],
        /// Reverse input data
        REVIN OFFSET(5) NUMBITS(2) [],
        /// Reverse output data
        REVOUT OFFSET(7) NUMBITS(1) []
    ],
    pub INIT [
        /// Initial CRC value
        INIT OFFSET(0) NUMBITS(32) []
    ],
    pub POL [
        /// Polynomial coefficients to be used for CRC computation
        POL OFFSET(0) NUMBITS(32) []
    ],
];

// CRC state checkers, used in all functions
#[derive(Copy, Clone, PartialEq)]
enum State {
    Idle,
    Processing,
}

/// Checker values for verifying if the algorithm has been set
#[derive(Copy, Clone, PartialEq)]
enum AlgSet {
    Uninitialised,
    Initialised,
}

/// Checker values for the DeferredCallClient
#[derive(Copy, Clone, PartialEq)]
enum Request {
    Input,
    Compute,
    None,
}

pub struct CRC<'a> {
    registers: StaticRef<CrcRegisters>,
    client: OptionalCell<&'a dyn Client>,
    deferred_call: DeferredCall,
    state: Cell<State>,
    alg_state: Cell<AlgSet>,
    buffer: OptionalCell<SubSliceMut<'static, u8>>,
    request: Cell<Request>,
    current_algorithm: OptionalCell<CrcAlgorithm>,
}

impl<'a> CRC<'a> {
    pub fn new(base_addr: StaticRef<CrcRegisters>) -> Self {
        Self {
            registers: base_addr,
            client: OptionalCell::empty(),
            deferred_call: DeferredCall::new(),
            state: Cell::new(State::Idle),
            alg_state: Cell::new(AlgSet::Uninitialised),
            buffer: OptionalCell::empty(),
            request: Cell::new(Request::None),
            current_algorithm: OptionalCell::empty(),
        }
    }
}

impl<'a> Crc<'a> for CRC<'a> {
    fn set_client(&self, client: &'a dyn Client) {
        self.client.set(client);
    }

    fn algorithm_supported(&self, algorithm: CrcAlgorithm) -> bool {
        matches!(
            algorithm,
            CrcAlgorithm::Crc32 | CrcAlgorithm::Crc32C | CrcAlgorithm::Crc16CCITT
        )
    }

    fn set_algorithm(&self, algorithm: CrcAlgorithm) -> Result<(), ErrorCode> {
        debug!("CRC: set_algorithm called!");

        if !self.algorithm_supported(algorithm) {
            return Err(ErrorCode::NOSUPPORT);
        }

        if self.state.get() == State::Processing {
            return Err(ErrorCode::BUSY);
        }

        // The STM32U5xx features programable parameters, in order to accomodate for
        // multiple CRC algorithms, enforceable by the user

        // INIT configurees the initial value of the CRC

        // PSIZE controls the size of the polynomial.
        // 00: 32 byt polynomial
        // 10: 8 bit polynomial
        // 01: 16 bit polynomial
        // 00: 32 bit polynomial

        // REVIN controls the reversal of the bit order of the input data.
        // 00: Bit order not affected
        // 01: Bit reversal done by byte
        // 10: Bit reversal done by half-word
        // 11: Bit reversal done by word
        // as per the CRC32 Ethernet algorithm, this one was set to byte by reversal

        // REVOUT controls the reversal of the bit order of the input data.
        // This bit controls the reversal of the bit order of the output data.
        // 0: Bit order not affected
        // 1: Bit-reversed output format

        // POL is used to write the coefficients of the polynomial to be used

        match algorithm {
            CrcAlgorithm::Crc32 => {
                self.registers.init.write(INIT::INIT.val(0xFFFFFFFF));
                self.registers.cr.modify(CR::PSIZE.val(0b00));
                self.registers.cr.modify(CR::REVIN.val(0b01));
                self.registers.cr.modify(CR::REVOUT.val(0b01));
                self.registers.pol.write(POL::POL.val(0x4C11DB7));
            }

            CrcAlgorithm::Crc32C => {
                self.registers.init.write(INIT::INIT.val(0xFFFFFFFF));
                self.registers.cr.modify(CR::PSIZE.val(0b00));
                self.registers.cr.modify(CR::REVIN.val(0b01));
                self.registers.cr.modify(CR::REVOUT.val(0b1));
                self.registers.pol.write(POL::POL.val(0x1EDC6F41));
            }

            CrcAlgorithm::Crc16CCITT => {
                self.registers.init.write(INIT::INIT.val(0x0000FFFF));
                self.registers.cr.modify(CR::PSIZE.val(0b01));
                self.registers.cr.modify(CR::REVIN.val(0b01));
                self.registers.cr.modify(CR::REVOUT.val(0b0));
                self.registers.pol.write(POL::POL.val(0x1021));
            }
        }

        // Initialising the CRC engine as per the manual, by setting the RESET Bit
        self.registers.cr.modify(CR::RESET::SET);
        self.state.set(State::Idle);
        self.alg_state.set(AlgSet::Initialised);
        self.current_algorithm.set(algorithm);

        Ok(())
    }

    fn input(
        &self,
        data: SubSliceMut<'static, u8>,
    ) -> Result<(), (ErrorCode, SubSliceMut<'static, u8>)> {
        debug!("CRC: input() called with {} bytes", data.len());

        if self.alg_state.get() == AlgSet::Uninitialised {
            return Err((ErrorCode::RESERVE, data));
        }

        if self.state.get() == State::Processing {
            return Err((ErrorCode::BUSY, data));
        }

        self.state.set(State::Processing);

        // wrote them as mut_slice and iter_mut first, realised that I only read them
        // so would not need it
        for &byte in data.as_slice().iter() {
            // should use set? or write
            self.registers.dr.set(byte as u32);
        }
        //reinterpret cast array de u32 uri

        debug!("CRC: Finished writing to DR, triggering deferred call");

        // pots and pans technology
        let mut consumed_data = data;
        let len = consumed_data.len();
        consumed_data.slice(len..len);

        self.buffer.set(consumed_data);
        self.request.set(Request::Input);
        self.deferred_call.set();

        Ok(())
    }

    fn compute(&self) -> Result<(), ErrorCode> {
        if self.alg_state.get() == AlgSet::Uninitialised {
            return Err(ErrorCode::RESERVE);
        }

        if self.state.get() == State::Processing {
            return Err(ErrorCode::BUSY);
        }

        self.state.set(State::Processing);
        self.request.set(Request::Compute);
        self.deferred_call.set();

        Ok(())
    }

    fn disable(&self) {
        // The STM's CRC has no bit for directly disabling it, we handle it by
        // setting the computation state to idle and the algorithm setting as
        // uninitialised.
        self.state.set(State::Idle);
        self.alg_state.set(AlgSet::Uninitialised);
    }
}

impl<'a> DeferredCallClient for CRC<'a> {
    fn handle_deferred_call(&self) {
        debug!("CRC: handle_deferred_call fired!");

        let current_request = self.request.get();
        self.request.set(Request::None);
        self.state.set(State::Idle);

        self.client.map(|client| match current_request {
            Request::Input => {
                debug!("CRC: Request::Input");
                if let Some(data) = self.buffer.take() {
                    debug!("CRC: client.input_done(Ok(()), data);");
                    client.input_done(Ok(()), data);
                }
            }

            Request::Compute => {
                let unprocessed_result = self.registers.dr.get();
                debug!("CRC: Request::Compute; client.crc_done...");
                let result = match self.current_algorithm.get() {
                    Some(CrcAlgorithm::Crc32) => CrcOutput::Crc32(unprocessed_result ^ 0xFFFFFFFF),
                    Some(CrcAlgorithm::Crc32C) => {
                        CrcOutput::Crc32C(unprocessed_result ^ 0xFFFFFFFF)
                    }
                    Some(CrcAlgorithm::Crc16CCITT) => {
                        CrcOutput::Crc16CCITT((unprocessed_result & 0xFFFF) as u16)
                    }
                    None => unreachable!("No algorithm has been set."),
                };
                client.crc_done(Ok(result));
            }

            Request::None => {
                debug!("CRC: Request::None");
            }
        });
    }

    fn register(&'static self) {
        self.deferred_call.register(self);
    }
}
