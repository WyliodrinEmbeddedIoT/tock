use crate::grant::Grant;
use crate::process::{Error};
use crate::processbuffer::{ReadWriteProcessBuffer, ReadOnlyProcessBuffer, ReadableProcessBuffer};
use crate::syscall::{CommandReturn, SyscallDriver};
use crate::{ErrorCode, ProcessId};
use crate::utilities::cells::TakeCell;

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

struct MessageHeader {
    value: usize,
    len: usize,
}

pub struct ProcessStorage<const NUM_PROCS: usize> {
    buffers: [ReadWriteProcessBuffer; NUM_PROCS],
    read_only_buffers: [ReadOnlyProcessBuffer; 2],
    sent_messages: [Option<MessageHeader>; NUM_PROCS],
    received_messages: [Option<MessageHeader>; NUM_PROCS],
    send_status: u64,
    receive_status: u32,
}

impl<const NUM_PROCS: usize> Default for ProcessStorage<NUM_PROCS> {
    fn default() -> ProcessStorage<NUM_PROCS> {
        const EMPTY_BUFFER: ReadWriteProcessBuffer = ReadWriteProcessBuffer::const_default();
        const EMPTY_MESSAGE_HEADER: Option<MessageHeader> = None;
        ProcessStorage {
            buffers: [EMPTY_BUFFER; NUM_PROCS],
            read_only_buffers: [ReadOnlyProcessBuffer::default(), ReadOnlyProcessBuffer::default()],
            sent_messages: [EMPTY_MESSAGE_HEADER; NUM_PROCS],
            received_messages: [EMPTY_MESSAGE_HEADER; NUM_PROCS],
            send_status: 0,
            receive_status: 0,
        }
    }
}

pub struct Mp<const NUM_PROCS: usize> {
    processes: Grant<ProcessStorage<NUM_PROCS>, NUM_PROCS>,
    identifiers: TakeCell<'static, [Option<ProcessId>; NUM_PROCS]>,
}

impl<const NUM_PROCS: usize> Mp<NUM_PROCS> {
    pub fn new(processes: Grant<ProcessStorage<NUM_PROCS>, NUM_PROCS>, identifiers: &'static mut [Option<ProcessId>; NUM_PROCS]) -> Mp<NUM_PROCS> {
        assert!(NUM_PROCS <= 32);
        Mp { processes, identifiers: TakeCell::new(identifiers) }
    }

    fn get_sent_message_status(sent_status: u64, destination_index: usize) -> MessageStatus {
        let status = sent_status >> (destination_index * 2) & 0b11;
        MessageStatus::from(status as u8)
    }

    fn set_sent_message_status(sent_status: &mut u64, destination_index: usize, status: MessageStatus) {
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
        self.identifiers.map(|identifiers| {
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
        }).expect("unable set a mp identifier");
        id as u32
    }
}

impl<const NUM_PROCS: usize> SyscallDriver for Mp<NUM_PROCS> {
    fn allow_readwrite(
        &self,
        process_id: ProcessId,
        allow_num: usize,
        mut buffer: ReadWriteProcessBuffer,
    ) -> Result<ReadWriteProcessBuffer, (ReadWriteProcessBuffer, ErrorCode)> {
        if allow_num < NUM_PROCS {
            match self.processes.enter(process_id, |process, _| {
                core::mem::swap(&mut process.buffers[allow_num], &mut buffer);
            }) {
                Ok(()) => Ok(buffer),
                Err(err) => Err((buffer, err.into())),
            }
        } else {
            Err((buffer, ErrorCode::NOSUPPORT))
        }
    }

    fn allow_readonly(
        &self,
        process_id: ProcessId,
        allow_num: usize,
        mut buffer: ReadOnlyProcessBuffer,
    ) -> Result<ReadOnlyProcessBuffer, (ReadOnlyProcessBuffer, ErrorCode)> {
        if allow_num < 2 {
            match self.processes.enter(process_id, |process, _| {
                core::mem::swap(&mut process.read_only_buffers[allow_num], &mut buffer);
            }) {
                Ok(()) => Ok(buffer),
                Err(err) => Err((buffer, err.into())),
            }
        } else {
            Err((buffer, ErrorCode::NOSUPPORT))
        }
    }

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
                let res = self.processes.enter(process_id, |process, _| {
                    process.read_only_buffers[SEARCH_SERVICE_BUFFER].enter(|buffer| {
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
                            None => Err(ErrorCode::NODEVICE)
                        }
                    }).map_err(|err| err.into()).and_then(|x| x)
                }).map_err(|err| err.into()).and_then(|x| x);
                match res {
                    Ok(id) => CommandReturn::success_u32(id),
                    Err(err) => CommandReturn::failure(err) 
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
