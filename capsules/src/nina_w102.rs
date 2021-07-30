use core::cell::Cell;

use kernel::utilities::cells::TakeCell;
use kernel::hil::spi::{SpiMasterDevice, SpiMasterClient};
use kernel::hil::gpio::Pin;
use kernel::ErrorCode;

use kernel::debug;

const START_CMD: u8 = 0xe0;
const END_CMD: u8 = 0xee;
const ERROR_CMD: u8 = 0xef;

const POS_CMD: usize = 1;
const POS_PARAM_LEN: usize = 2;

const CMD_FLAG: u8 = 0;
const REPLY_FLAG: u8 = 1<<7;
const DATA_FLAG: u8 = 1<<6; 

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
enum Command {
	GetFwVersion = 0x37,
}

#[derive(Copy, Clone, PartialEq)]
enum Status {
    Idle,
    Send(Command),
    Receive(Command)
}

pub struct NinaW102<'a, S: SpiMasterDevice, P:Pin> {
    spi: &'a S,
    write_buffer: TakeCell<'static, [u8]>,
    read_buffer: TakeCell<'static, [u8]>,
    ready: &'a P,
    reset: &'a P,
    status: Cell<Status>,
}

impl<'a, S: SpiMasterDevice, P:Pin> NinaW102<'a, S, P> {
    pub fn new(spi: &'a S, write_buffer: &'static mut[u8], read_buffer: &'static mut[u8], ready: &'a P, reset: &'a P) -> Self {
        ready.make_input ();
        reset.make_output ();
        NinaW102 {
            spi,
            write_buffer: TakeCell::new(write_buffer),
            read_buffer: TakeCell::new(read_buffer),
            ready,
            reset,
            status: Cell::new(Status::Idle)
        }
    }

    pub fn init () {
        
    }

    pub fn get_firmware_version(&self) -> Result<(), ErrorCode> {
        if self.status.get() == Status::Idle {            
            self.send_command (Command::GetFwVersion, 0)
        }
        else
        {
            Err(ErrorCode::BUSY)
        }
    }

    fn send_command(&self, command: Command, num_params: u8) -> Result<(), ErrorCode> {
        // should be async
        while self.ready.read() {}
        debug! ("is ready");
        self.write_buffer.take ().map_or(Err(ErrorCode::NOMEM), |buffer| {
            buffer[0] = START_CMD;
            buffer[POS_CMD] = (command as u8) & !REPLY_FLAG;
            buffer[POS_PARAM_LEN] = num_params;
            // send parameters
            buffer[3] = END_CMD;
            debug! ("{:?}", &buffer[0..4]);
            self.spi.read_write_bytes (buffer, None, 4);
            self.status.set (Status::Send(Command::GetFwVersion));
            Ok(())
        })
    }

    fn receive_command(&self, command: Command, num_params: u8) -> Result<(), ErrorCode> {
        // // should be async
        while self.ready.read() {}
        self.write_buffer.take ().map_or(Err(ErrorCode::NOMEM), |buffer| {
            for i in 0..(num_params as usize + 4) {
                buffer[i] = 0xff;
            }
            self.spi.read_write_bytes (buffer, self.read_buffer.take (), (num_params+4) as usize);
            self.status.set (Status::Receive(Command::GetFwVersion));
            Ok(())
        })
    }
}


impl <'a, S: SpiMasterDevice, P:Pin> SpiMasterClient for NinaW102<'a, S, P> {
    fn read_write_done (&self, write_buffer: &'static mut [u8], read_buffer: Option<&'static mut [u8]>, len: usize) {
        match self.status.get () {
            Status::Send(command) => {
                debug! ("sent command {:?}", command);
                match command {
                    Command::GetFwVersion => {
                        self.write_buffer.replace(write_buffer);
                        self.receive_command (Command::GetFwVersion, 90);
                    }
                }
            }
            Status::Receive(command) => {
                debug! ("received command {:?}", command);
                match command {
                    Command::GetFwVersion => {
                        self.write_buffer.replace(write_buffer);
                        read_buffer.map(|read_buffer| {
                            debug! ("{:?}", &read_buffer[0..90]);
                            debug! ("ready {}", self.ready.read());
                            self.read_buffer.replace (read_buffer)
                        });
                    }
                }
            }
            Status::Idle => {
                self.write_buffer.replace(write_buffer);
                read_buffer.map(|read_buffer| self.read_buffer.replace (read_buffer));
                // 
            }
        }
    }
}