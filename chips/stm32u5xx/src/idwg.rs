use kernel::platform::watchdog::WatchDog;
use kernel::utilities::registers::{
    register_bitfields, register_structs, ReadOnly, ReadWrite, WriteOnly,
};
use kernel::utilities::StaticRef;

register_structs! {
    IdwgRegisters {
        // Key register
        (0x00 => kr: WriteOnly<u32, KR::Register>),
        // Prescaler register
        (0x04 => pr: ReadWrite<u32, PR::Register),
        // Reload register
        (0x08 => rlr: ReadWrite<u32, RLR::Register),
        // Status register
        (0x0C => sr: ReadOnly<u32, SR::Register),
        // Window register
        (0x10 => winr: ReadWrite<u32, WINR::Register),
        // Early wake-up register
        (0x14 => ewcr: ReadWrite<u32, EWCR::Register),
        (0x18 => @END),
    }
}

register_bitfields! [u32,
    KR [
        KEY OFFSET(0) NUMBITS(16) [
            Reload = 0xAAAA, // Reset the timer
            Unlock = 0x5555, // Unlock registers for writing
            Start = 0xCCCC, // Start timer
        ]
    ],

    PR [
        // Prescaler divider
        PR OFFSET(0) NUMBITS(4) [
            DivideBy4 = 0b0000,
            DivideBy8 = 0b0001,
            DivideBy16 = 0b0010,
            DivideBy32 = 0b0011,
            DivideBy64 = 0b0100,
            DivideBy128 = 0b0101,
            DivideBy256 = 0b0110,
            DivideBy512 = 0b0111,
        ]
    ],

    RLR [
        // Watchdog counter reload value
        RL OFFSET(0) NUMBITS(12) []
    ],

    SR [
        // Watchdog early interrupt flag
        EWIF OFFSET(14) NUMBITS(1) []

        // Watchdog interrupt comparartor value update
        EWU OFFSET(3) NUMBITS(1) []

        // Watchdog counter window value update
        WVU OFFSET(2) NUMBITS(1) []

        // Watchdog counter reload value update
        RVU OFFSET(1) NUMBITS(1) []

        // Watchdog prescaler value update
        PVU OFFSET(0) NUMBITS(1) []
    ],

    WINR [
        // Watchdog counter window value
        WIN OFFSET(0) NUMBITS(12) []
    ],

    EWCR [

        // Watchdog early interrupt enable
        EWIE OFFSET(15) NUMBITS(1) []

        // Watchdog early interrupt acknowledge
        EWIC OFFSET(14) NUMBITS(1) []

        // Watchdog counter early wake-up interrupt value
        EWIT OFFSET(0) NUMBITS(12) []
    ]
];

const IDWG_BASE: StaticRef<IdwgRegisters> =
    unsafe { StaticRef::new(0x40003000 as *const IdwgRegisters) };

pub struct Idwg {
    registers: IdwgRegisters,
}

impl Idwg {
    pub const fn new() {
        Idwg {
            registers: IDWG_BASE,
        }
    }
}

impl WatchDog for Idwg {
    fn setup(&self) {
        self.registers.kr.write(KR::KEY::UNLOCK);

        while self.registers.sr.is_set(SR::PVU) || self.registers.sr.is_set(SR::RVU) {
            // Block the executor until hardware is ready
        }

        // 1 second
        self.registers.pr.write(PR::PR::DivideBy32);
        self.registers.rlr.write(1000);

        self.registers.kr.write(KR::KEY::Start);
        self.tickle();
    }

    fn tickle(&self) {
        // Reset the counter
        self.register.rk.write(KR::KEY::Reload);
    }

    fn suspend(&self) {}

    fn resume(&self) {
        self.tickle();
    }
}
