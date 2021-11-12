use core::cell::Cell;
use core::ptr::read;

use kernel::debug;
use kernel::hil::gpio::Pin;
use kernel::hil::spi::{SpiMasterClient, SpiMasterDevice};
use kernel::hil::time::{Alarm, ConvertTicks};
use kernel::utilities::cells::TakeCell;
use kernel::ErrorCode;

const START_CMD: u8 = 0xe0;
const END_CMD: u8 = 0xee;
const ERROR_CMD: u8 = 0xef;

const POS_CMD: usize = 1;
const POS_PARAM_LEN: usize = 2;
const POS_LEN: usize = 2;
const POS_PARAM: usize = 3;

const CMD_FLAG: u8 = 0;
const REPLY_FLAG: u8 = 1 << 7;
const DATA_FLAG: u8 = 1 << 6;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
enum Command {
    GetFwVersion = 0x37,
    StartScanNetworksCmd = 0x36,
    ScanNetworksCmd = 0x27,
    GetConnStatusCmd = 0x20,
}
#[derive(Copy, Clone, PartialEq, Debug)]
enum InitStatus {
    Starting,
    Initialized,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Status {
    Idle,
    Init(InitStatus),
    Send(Command),
    Receive(Command),
    GetFirmware,
    StartScanNetworks,
    ScanNetworks,
    GetConnStatus,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum ConnectionStatus {
    Idle,
    NoSSIDAvailable,
    ScanCompleted,
    Connected,
    ConnectFailed,
    ConnectionLost,
    Disconnected,
    NoShield,
    Unknown,
}

pub struct NinaW102<'a, S: SpiMasterDevice, P: Pin, A: Alarm<'a>> {
    spi: &'a S,
    write_buffer: TakeCell<'static, [u8]>,
    read_buffer: TakeCell<'static, [u8]>,
    cs: &'a P,
    ready: &'a P,
    reset: &'a P,
    gpio0: &'a P,
    alarm: &'a A,
    status: Cell<Status>,
}

impl<'a, S: SpiMasterDevice, P: Pin, A: Alarm<'a>> NinaW102<'a, S, P, A> {
    pub fn new(
        spi: &'a S,
        write_buffer: &'static mut [u8],
        read_buffer: &'static mut [u8],
        cs: &'a P,
        ready: &'a P,
        reset: &'a P,
        gpio0: &'a P,
        alarm: &'a A,
    ) -> Self {
        cs.make_output();
        ready.make_input();
        reset.make_output();
        gpio0.make_output();

        NinaW102 {
            spi,
            write_buffer: TakeCell::new(write_buffer),
            read_buffer: TakeCell::new(read_buffer),
            cs,
            ready,
            reset,
            gpio0,
            alarm: alarm,
            status: Cell::new(Status::Idle),
        }
    }

    pub fn init(&self) -> Result<(), ErrorCode> {
        self.cs.set();
        self.reset.clear();
        self.gpio0.set();
        self.status.set(Status::Init(InitStatus::Starting));
        self.alarm
            .set_alarm(self.alarm.now(), self.alarm.ticks_from_ms(10));
        Ok(())
    }

    pub fn get_firmware_version(&self) -> Result<(), ErrorCode> {
        if self.status.get() == Status::Idle {
            // while self.ready.read() {}
           // debug!("Iese din while");
            self.send_command(Command::GetFwVersion, 0);
            Ok(())
        } else {
            Err(ErrorCode::BUSY)
        }
    }

    pub fn scan_networks(&self) -> Result<(), ErrorCode> {
        if self.status.get() == Status::Idle || self.status.get() == Status::ScanNetworks {
           // debug!("Iese din while");
            self.send_command(Command::ScanNetworksCmd, 0);
            Ok(())
        } else {
            Err(ErrorCode::BUSY)
        }
    }


    pub fn start_scan_networks(&self) -> Result<(), ErrorCode> {
        if self.status.get() == Status::Idle || self.status.get() == Status::StartScanNetworks {
            // while self.ready.read() {}
           // debug!("Iese din while");
            self.send_command(Command::StartScanNetworksCmd, 0);
            Ok(())
        } else {
            Err(ErrorCode::BUSY)
        }
    }

    pub fn get_connection_status(&self) -> Result<(), ErrorCode> {
        if self.status.get() == Status::Idle {
            // while self.ready.read() {}
           // debug!("Iese din while");
            self.send_command(Command::GetConnStatusCmd, 0);
            Ok(())
        } else {
            Err(ErrorCode::BUSY)
        }
    }

    fn wait_for_chip_ready(&self) -> Result<(), ErrorCode> {
        for i in 0..100000 {
            if !self.ready.read() {
                return Ok(());
            }
        }
        panic!("chip not ready");
        Err(ErrorCode::BUSY)
    }

    fn wait_for_chip_select(&self) -> Result<(), ErrorCode> {
        self.cs.clear();
        for i in 0..100000 {
            if self.ready.read() {
                return Ok(());
            }
        }
        panic!("chip not select");
        self.cs.set();
        Err(ErrorCode::NOACK)
    }

    fn send_command(&self, command: Command, num_params: u8) -> Result<(), ErrorCode> {
        // should be async
        self.wait_for_chip_ready()?;
        self.wait_for_chip_select()?;
        /*
        if let Err(err) = self.wait_for_chip_ready() {
            return Err(err);
        }
         */
        //debug!("is ready");
        self.write_buffer
            .take()
            .map_or(Err(ErrorCode::NOMEM), |buffer| {
                buffer[0] = START_CMD;
                buffer[POS_CMD] = (command as u8) & !REPLY_FLAG;
                buffer[POS_PARAM_LEN] = num_params;
                // send parameters
                buffer[3] = END_CMD;
                debug!("{:?}", &buffer[0..4]);

                // while !self.ready.read() {}

                self.spi
                    .read_write_bytes(buffer, None, 4)
                    .map_err(|(err, write_buffer, _)| {
                        self.write_buffer.replace(write_buffer);
                        err
                    })?;
                //self.status.set(Status::Send(Command::GetFwVersion));
                self.status.set(Status::Send(command));
                debug!("send command {:?}", command);

                Ok(())
            })
            .map_err(|err| {
                self.cs.set();
                err
            })
    }

    fn receive_command(&self, command: Command, num_params: usize) -> Result<(), ErrorCode> {
        // // should be async
        //while self.ready.read() {}
        // debug!("Intra pe receive_command");
        self.wait_for_chip_ready()?;
        // panic! ("chip ready");
        self.wait_for_chip_select()?;
    
        self.write_buffer
            .take()
            .map_or(Err(ErrorCode::NOMEM), |buffer| {
                for i in 0..buffer.len() {
                    buffer[i] = 0xff;
                }
                // self.cs.clear();
                // while !self.ready.read() {}
                // if self.wait_for_chip_ready() {
                self.read_buffer
                    .take()
                    .map_or(Err(ErrorCode::NOMEM), move |read_buffer| {
                        let len = read_buffer.len();
                        debug!("receive {:?}", command);
                        self.status.set(Status::Receive(command));
                        self.spi
                            .read_write_bytes(
                                buffer,
                                Some(read_buffer), //TODO put the read buffer back
                                len,
                            )
                            .map_err(|(err, write_buffer, _)| {
                                self.write_buffer.replace(write_buffer);
                                err
                            })
                    })?;
                //self.status.set(Status::Receive(Command::GetFwVersion));
               // debug!("Actual status in receive_command{:?}", self.status.get());
                // } else {
                //     self.status.set(Status::Idle)
                // }
                Ok(())
            })
            .map_err(|err| {
                self.cs.set();
                err
            })
    }
}

impl<'a, S: SpiMasterDevice, P: Pin, A: Alarm<'a>> SpiMasterClient for NinaW102<'a, S, P, A> {
    fn read_write_done(
        &self,
        write_buffer: &'static mut [u8],
        read_buffer: Option<&'static mut [u8]>,
        len: usize,
        status: Result<(), ErrorCode>,
    ) {
        match self.status.get() {
            Status::Send(command) => {
                debug!("sent command {:?}", command);
               
                        self.write_buffer.replace(write_buffer);
                        self.receive_command(command, 1000);
                    
               
            }
            Status::Receive(command) => {
                debug!("received command {:?} {}", command, len);
                self.status.set(Status::Idle);

                // verific pachet
                
                self.write_buffer.replace(write_buffer);
                read_buffer.map(|read_buffer| {
                    let index = read_buffer
                        .iter()
                        .position(|value| *value == START_CMD || *value == ERROR_CMD); //TODO return noack if not index
                    debug!("index{:?}", index);
                    //TODO verificam daca exista indexul, altfel noACK
                    //TODO verificam startcmd
                    //TODO verificam flag-ul sa fie de reply cu shiftare (b>>7 = 1)
                    //TODO verificam daca exista endcmd dupa nr de parametri(pozitia index+3 in buffer)
                    //TODO aifsam stringul de la index+4 la index+10
                    let status = if let Some(index) = index {
                        if index < read_buffer.len() - 4 {
                            debug!("{:?}", &read_buffer[index..index + 11]);
                            //TODO inform client if start is correct
                            // let param_number = (read_buffer[index+3] as usize + index as usize + 3) as usize ;
                            if (read_buffer[index] == START_CMD
                                && read_buffer[index + POS_CMD] == (command as u8) | REPLY_FLAG)
                            {
                                debug!("Pachetul e ok");
                               /*  debug!(
                                    "{:?}",
                                    core::str::from_utf8(&read_buffer[index + 2..index + 4])
                                );*/
                                let param_len = read_buffer[index + POS_LEN];//comanda start scan networks are si param len

                                // debug!("params {}", param_len);

                                let mut current_position = 0;
                                for parameter_index in 0..param_len {
                                    let pos = index + POS_PARAM + current_position;
                                    // debug!("params position {}", param_len);
                                    // debug!("Schimba currebnt pos");
                                    if pos < read_buffer.len() {
                                        current_position = (current_position + read_buffer[pos] as usize) as usize;
                                        // debug!("Schimba currebnt pos");
                                    } else {
                                        break;
                                    }
                                    current_position = current_position + 1;
                                }
                                // debug!("Iese din for");
                                let end_pos = index + POS_PARAM + current_position;

                                // debug!("End pos este {:?} ", end_pos);
                                // debug!("read_buffer[end_pos] este {:?}", read_buffer[end_pos]);
                                if end_pos < read_buffer.len() && read_buffer[end_pos] == END_CMD {
                                    // ok
                                    // debug!("A gasit end cmd");
                                    match command {
                                        Command::GetFwVersion => {

                                            debug!(
                                                "{:?}",
                                                core::str::from_utf8(&read_buffer[index + 4 ..index + 10]));
                                            self.get_connection_status();

                                        }
                                        Command::ScanNetworksCmd => {
                                            // debug!("Received command {:?}", command);
                                            self.status.set(Status::ScanNetworks);
                                            self.alarm.set_alarm(self.alarm.now(), self.alarm.ticks_from_ms(2000));
                                        }

                                        Command::StartScanNetworksCmd => {
                                            // debug!("Received command {:?}", command);
                                            self.status.set(Status::ScanNetworks);
                                            self.alarm.set_alarm(self.alarm.now(), self.alarm.ticks_from_ms(2000));
                                        }

                                        Command::GetConnStatusCmd => {
                                            // debug!("Received command {:?}", command);
                                            self.get_firmware_version();
                                        }
                                        _=>{}
                                    }
                                    Ok(())
                                } else {
                                    Err(ErrorCode::INVAL)
                                }
                            } else {
                                match command {
                                    Command::ScanNetworksCmd => {
                                        // debug!("Received command {:?}", command);
                                        self.status.set(Status::ScanNetworks);
                                        self.alarm.set_alarm(self.alarm.now(), self.alarm.ticks_from_ms(2000));
                                    }
                                    Command::StartScanNetworksCmd => {
                                        self.status.set(Status::StartScanNetworks);
                                        self.alarm.set_alarm(self.alarm.now(), self.alarm.ticks_from_ms(2000));
                                    }
                                    _=>{}
                                }
                                Err(ErrorCode::FAIL)
                            }
                        } else {
                            Err(ErrorCode::INVAL)
                        }
                    } else {
                        Err(ErrorCode::NOACK)
                    };

                    // debug!("ready {}", self.ready.read());
                    // TODO make sure this while exists
                    // while self.ready.read() {}
                    // debug!("ready {}", self.ready.read());
                    self.read_buffer.replace(read_buffer)
                });
            }
            Status::Idle => {
                self.write_buffer.replace(write_buffer);
                read_buffer.map(|read_buffer| self.read_buffer.replace(read_buffer));
            }

            Status::GetFirmware => {
                debug!("status get firmware");
            }

            Status::ScanNetworks => {
                debug!("status scan networks");
            }

            _ => {}
        }
    }
}
use kernel::hil::time::AlarmClient;

impl<'a, S: SpiMasterDevice, P: Pin, A: Alarm<'a>> AlarmClient for NinaW102<'a, S, P, A> {
    fn alarm(&self) {
        debug!("alarm {:?}", self.status.get());
        match self.status.get() {
            Status::Init(init_status) => match init_status {
                InitStatus::Starting => {
                    debug!("alarm status starting");
                    self.reset.set();
                    self.alarm
                        .set_alarm(self.alarm.now(), self.alarm.ticks_from_ms(750));
                    self.status.set(Status::Init(InitStatus::Initialized));
                }

                InitStatus::Initialized => {
                    debug!("Alarm status initialized");
                    self.gpio0.clear();
                    self.gpio0.make_input();
                    self.status.set(Status::Idle);
                //    self.get_firmware_version();
                   self.get_connection_status();
                //  self.start_scan_networks();
                   
                }
            },

            Status::GetFirmware => {
                // debug!("status get firmware");
               
            }

            Status::StartScanNetworks => {
                self.start_scan_networks();
            }

            Status::ScanNetworks => {
                // debug!("status scan networks");
                self.scan_networks();
            }
            
            Status::GetConnStatus => {
                // debug!("status get conn status");
               // self.get_connection_status();
            }

            _ => {
                panic!("Alarm not starting");
            }
        }
        
    }
}
