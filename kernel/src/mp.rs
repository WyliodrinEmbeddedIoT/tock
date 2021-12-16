use crate::grant::{AllowRoCount, AllowRwCount, Grant, UpcallCount};
use crate::process::Error;
use crate::processbuffer::{ReadOnlyProcessBuffer, ReadWriteProcessBuffer, ReadableProcessBuffer};
use crate::syscall::{CommandReturn, SyscallDriver};
use crate::utilities::cells::TakeCell;
use crate::{ErrorCode, ProcessId};

enum MessageStatus {
    Ready = 0b00,
    Sent = 0b01,
    Ack = 0b10,
    Error = 0xb11,
}

impl From<u8> for MessageStatus {
    fn from(original: u8) -> MessageStatus {
        match original {
            0b00 => MessageStatus::Ready,
            0b01 => MessageStatus::Sent,
            0b10 => MessageStatus::Ack,
            _ => MessageStatus::Error,
        }
    }
}

const SEND_COMMAND_MASK: usize = 0x10;
const ACK_COMMAND_MASK: usize = 0x20;
const READ_COMMAND_MASK: usize = 0x40;

const SEARCH_SERVICE_BUFFER: usize = 0;
const REPLY_BUFFER: usize = 1;

mod allow_ro {
    pub const SEARCH_SERVICE_BUFFER: usize = 0;
    pub const COUNT: usize = 1;
}

// NUM_BUFFERS has to be NUM_PROCS * 2
pub type MpGrant<const NUM_PROCS: usize, const NUM_BUFFERS: usize> = Grant<
    ProcessStorage,
    UpcallCount<NUM_PROCS>,
    AllowRoCount<{ allow_ro::COUNT }>,
    AllowRwCount<NUM_BUFFERS>,
>;

struct MessageHeader {
    value: usize,
    len: usize,
}

#[derive(Default)]
pub struct ProcessStorage {
    send_status: u64,
    receive_status: u32,
}

pub struct Mp<const NUM_PROCS: usize, const NUM_BUFFERS: usize> {
    processes: MpGrant<NUM_PROCS, NUM_BUFFERS>,
    identifiers: TakeCell<'static, [Option<ProcessId>; NUM_PROCS]>,
}

impl<const NUM_PROCS: usize, const NUM_BUFFERS: usize> Mp<NUM_PROCS, NUM_BUFFERS> {
    pub fn new(
        processes: MpGrant<NUM_PROCS, NUM_BUFFERS>,
        identifiers: &'static mut [Option<ProcessId>; NUM_PROCS],
    ) -> Mp<NUM_PROCS, NUM_BUFFERS> {
        assert!(NUM_PROCS <= 32);
        Mp {
            processes,
            identifiers: TakeCell::new(identifiers),
        }
    }

    fn get_sent_message_status(sent_status: u64, destination_index: usize) -> MessageStatus {
        let status = sent_status >> (destination_index * 2) & 0b11;
        MessageStatus::from(status as u8)
    }

    fn set_sent_message_status(
        sent_status: &mut u64,
        destination_index: usize,
        status: MessageStatus,
    ) {
        *sent_status &= !(0b11 << (destination_index * 2));
        *sent_status |= (status as u64) << (destination_index * 2);
    }

    fn filter_identifiers(&self) {
        self.identifiers.map(|identifiers| {
            for identifier in identifiers.iter_mut() {
                if let Some(process_id) = identifier {
                    if process_id.index().is_none() {
                        *identifier = None
                    }
                }
            }
        });
    }

    fn get_identifier(&self, process: ProcessId) -> u32 {
        let mut id = 0;
        self.filter_identifiers();
        self.identifiers
            .map(|identifiers| {
                for (index, identifier) in identifiers.iter().enumerate() {
                    if let Some(process_id) = identifier {
                        if process_id.id() == process.id() {
                            id = index;
                            return;
                        }
                    }
                }
                for (index, identifier) in identifiers.iter_mut().enumerate() {
                    if let None = identifier {
                        *identifier = Some(process);
                        id = index;
                        return;
                    }
                }
            })
            .expect("unable set a mp identifier");
        id as u32
    }
}

impl<const NUM_PROCS: usize, const NUM_BUFFERS: usize> SyscallDriver
    for Mp<NUM_PROCS, NUM_BUFFERS>
{
    fn command(
        &self,
        command_num: usize,
        value: usize,
        len: usize,
        process_id: ProcessId,
    ) -> CommandReturn {
        match command_num {
            0 => CommandReturn::success(),
            // search for process
            1 => {
                let res = self
                    .processes
                    .enter(process_id, |process, kernel_grant| {
                        kernel_grant
                            .get_readwrite_processbuffer(allow_ro::SEARCH_SERVICE_BUFFER)
                            .and_then(|buffer_ref| {
                                buffer_ref
                                    .enter(|buffer| {
                                        match self.processes.kernel.process_until(|p| {
                                            let s = p.get_process_name().as_bytes();
                                            // are slices equal?
                                            if s.len() == buffer.len()
                                                && s.iter()
                                                    .zip(buffer.iter())
                                                    .all(|(c1, c2)| *c1 == c2.get())
                                            {
                                                Some(self.get_identifier(p.processid()))
                                            } else {
                                                None
                                            }
                                        }) {
                                            Some(id) => Ok(id),
                                            None => Err(ErrorCode::NODEVICE),
                                        }
                                    })
                                    .map_err(|error| error.into())
                                    .and_then(|x| x)
                            })
                            .map_err(|x| x.into())
                        // process.read_only_buffers[SEARCH_SERVICE_BUFFER]
                        //     .enter(|buffer| {
                        //
                        //     })
                        //     .map_err(|err| err.into())
                        //     .and_then(|x| x)
                    })
                    .map_err(|err| err.into())
                    .and_then(|x| x);
                match res {
                    Ok(id) => CommandReturn::success_u32(id),
                    Err(err) => CommandReturn::failure(err),
                }
            }
            // Send command
            command_num if SEND_COMMAND_MASK & command_num != 0 => {
                // let destination_process_index = command_num & SEND_COMMAND_MASK;
                // if destination_process_index < NUM_PROCS
                //     && destination_process_index != process_id.index()
                // {
                //     self.processes.enter(process_id, |process, _| {
                //         let status = Mp::get_sent_message_status(
                //             process.sent_status,
                //             destination_process_index,
                //         );
                //         match status {
                //             MessageStatus::Ready | MessageStatus::Sent => {
                //                 if len <= process.buffers[destination_process_index].len() {
                //                     process.sent_messages[destination_process_index] =
                //                         Some(MessageHeader { value, len });
                //                     Mp::set_sent_message_status(
                //                         process.sent_status,
                //                         destination_process_index,
                //                         MessageStatus::Sent,
                //                     );
                //                     Ok(())
                //                 } else {
                //                     Err(ErrorCode::NOMEM)
                //                 }
                //             }
                //             MessageStatus::Ack => Err(ErrorCode::BUSY),
                //             MessageStatus::Error => Err(ErrorCode::FAIL),
                //         }
                //     });
                // } else {
                //     CommandReturn::failure(ErrorCode::INVAL)
                // }
                CommandReturn::failure(ErrorCode::NOSUPPORT)
            }
            // ack command
            command_num if ACK_COMMAND_MASK & command_num != 0 => CommandReturn::success(),
            // read command
            command_num if READ_COMMAND_MASK & command_num != 0 => CommandReturn::success(),
            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
        }
    }

    fn allocate_grant(&self, process_id: ProcessId) -> Result<(), Error> {
        self.processes.enter(process_id, |_, _| {})
    }
}
