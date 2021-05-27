// Derived from uBPF <https://github.com/iovisor/ubpf>
// Copyright 2015 Big Switch Networks, Inc
//      (uBPF: VM architecture, parts of the interpreter, originally in C)
// Copyright 2016 6WIND S.A. <quentin.monnet@6wind.com>
//      (Translation to Rust, MetaBuff/multiple classes addition, hashmaps for helpers)
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0> or
// the MIT license <http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.


//! Virtual machine and JIT compiler for eBPF programs.
#![doc(html_logo_url = "https://raw.githubusercontent.com/qmonnet/rbpf/master/misc/rbpf.png",
       html_favicon_url = "https://raw.githubusercontent.com/qmonnet/rbpf/master/misc/rbpf.ico")]

#![warn(missing_docs)]
// There are unused mut warnings due to unsafe code.
#![allow(unused_mut)]
// Allows old-style clippy
#![allow(renamed_and_removed_lints)]

#![cfg_attr(feature = "cargo-clippy", allow(redundant_field_names, single_match, cast_lossless, doc_markdown, match_same_arms, unreadable_literal))]
use kernel::debug;
use kernel::{ErrorCode};

use crate::bpf::ebpf;
use crate::bpf::verifier;

/// eBPF verification function that returns an error if the program does not meet its requirements.
///
/// Some examples of things the verifier may reject the program for:
///
///   - Program does not terminate.
///   - Unknown instructions.
///   - Bad formed instruction.
///   - Unknown eBPF helper index.
pub type Verifier = fn(prog: &[u8]) -> Result<(), ErrorCode>;

/// A virtual machine to run eBPF program. This kind of VM is used for programs expecting to work
/// directly on the memory area representing packet data.
///
/// # Examples
///
/// ```
/// let prog = &[
///     0x71, 0x11, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, // ldxb r1[0x04], r1
///     0x07, 0x01, 0x00, 0x00, 0x00, 0x22, 0x00, 0x00, // add r1, 0x22
///     0xbf, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r0, r1
///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
/// ];
/// let mem = &mut [
///     0xaa, 0xbb, 0x11, 0x22, 0xcc, 0xdd
/// ];
///
/// // Instantiate a VM.
/// let vm = rbpf::EbpfVmRaw::new(Some(prog)).unwrap();
///
/// // Provide only a reference to the packet data.
/// let res = vm.execute_program(mem).unwrap();
/// assert_eq!(res, 0x22cc);
/// ```
pub struct EbpfVmRaw<'a> {
    prog:     Option<&'a [u8]>,
    verifier: Verifier,
}

impl<'a> EbpfVmRaw<'a> {
    /// Create a new virtual machine instance, and load an eBPF program into that instance.
    /// When attempting to load the program, it passes through a simple verifier.
    ///
    /// # Examples
    ///
    /// ```
    /// let prog = &[
    ///     0x71, 0x11, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, // ldxb r1[0x04], r1
    ///     0x07, 0x01, 0x00, 0x00, 0x00, 0x22, 0x00, 0x00, // add r1, 0x22
    ///     0xbf, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r0, r1
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    ///
    /// // Instantiate a VM.
    /// let vm = rbpf::EbpfVmRaw::new(Some(prog)).unwrap();
    /// ```
    pub fn new(prog: Option<&'a [u8]>) -> Result<EbpfVmRaw<'a>, ErrorCode> {
        if let Some(prog) = prog {
            verifier::check(prog)?;
        }

        Ok(EbpfVmRaw {
            prog:     prog,
            verifier: verifier::check,
        })
    }

    /// Load a new eBPF program into the virtual machine instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let prog1 = &[
    ///     0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r0, 0
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    /// let prog2 = &[
    ///     0x71, 0x11, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, // ldxb r1[0x04], r1
    ///     0x07, 0x01, 0x00, 0x00, 0x00, 0x22, 0x00, 0x00, // add r1, 0x22
    ///     0xbf, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r0, r1
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    ///
    /// let mem = &mut [
    ///     0xaa, 0xbb, 0x11, 0x22, 0xcc, 0x27,
    /// ];
    ///
    /// let mut vm = rbpf::EbpfVmRaw::new(Some(prog1)).unwrap();
    /// vm.set_program(prog2);
    ///
    /// let res = vm.execute_program(mem).unwrap();
    /// assert_eq!(res, 0x22cc);
    /// ```
    pub fn set_program(&mut self, prog: &'a [u8]) -> Result<(), ErrorCode> {
        (self.verifier)(prog)?;
        self.prog = Some(prog);
        Ok(())
    }

    /// Set a new verifier function. The function should return an `Error` if the program should be
    /// rejected by the virtual machine. If a program has been loaded to the VM already, the
    /// verifier is immediately run.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{Error, ErrorKind};
    /// use rbpf::ebpf;
    ///
    /// // Define a simple verifier function.
    /// fn verifier(prog: &[u8]) -> Result<(), Error> {
    ///     let last_insn = ebpf::get_insn(prog, (prog.len() / ebpf::INSN_SIZE) - 1);
    ///     if last_insn.opc != ebpf::EXIT {
    ///         return Err(Error::new(ErrorKind::Other,
    ///                    "[Verifier] Error: program does not end with “EXIT” instruction"));
    ///     }
    ///     Ok(())
    /// }
    ///
    /// let prog1 = &[
    ///     0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r0, 0
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    ///
    /// // Instantiate a VM.
    /// let mut vm = rbpf::EbpfVmMbuff::new(Some(prog1)).unwrap();
    /// // Change the verifier.
    /// vm.set_verifier(verifier).unwrap();
    /// ```
    pub fn set_verifier(&mut self, verifier: Verifier) -> Result<(), ErrorCode> {
        if let Some(prog) = self.prog {
            verifier(prog)?;
        }
        self.verifier = verifier;
        Ok(())
    }

    /// Execute the program loaded, with the given packet data.
    ///
    /// # Examples
    ///
    /// ```
    /// let prog = &[
    ///     0x71, 0x11, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, // ldxb r1[0x04], r1
    ///     0x07, 0x01, 0x00, 0x00, 0x00, 0x22, 0x00, 0x00, // add r1, 0x22
    ///     0xbf, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r0, r1
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    ///
    /// let mem = &mut [
    ///     0xaa, 0xbb, 0x11, 0x22, 0xcc, 0x27
    /// ];
    ///
    /// let mut vm = rbpf::EbpfVmRaw::new(Some(prog)).unwrap();
    ///
    /// let res = vm.execute_program(mem).unwrap();
    /// assert_eq!(res, 0x22cc);
    /// ```
    #[allow(unknown_lints)]
    #[allow(cyclomatic_complexity)]
    pub fn execute_program(&self, mem: &mut [u8]) -> Result<u64, ErrorCode> {
        const U32MAX: u64 = u32::MAX as u64;

        let prog = match self.prog { 
            Some(prog) => prog,
            None => Err(ErrorCode::INVAL)?,
        };

        let mem_ptr = mem.as_ptr();
        let mem_len = mem.len();
        
        let stack = mem_ptr as u64 + mem_len as u64 - ebpf::STACK_SIZE as u64;

        // R1 points to beginning of memory area, R10 to stack
        let mut reg: [u64;11] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, stack as u64 + ebpf::STACK_SIZE as u64
        ];

        if !mem.is_empty() {
            reg[1] = mem_ptr as u64;
        }

        let check_mem_load = | mem: &mut [u8], addr: u64, len: usize, insn_ptr: usize | {
            EbpfVmRaw::check_mem(addr, len, "load", insn_ptr, mem, stack)
        };
        let check_mem_store = | mem: &mut [u8], addr: u64, len: usize, insn_ptr: usize | {
            EbpfVmRaw::check_mem(addr, len, "store", insn_ptr, mem, stack)
        };

        // let mut iterations = 0;
        // Loop on instructions
        let mut insn_ptr:usize = 0;
        while insn_ptr * ebpf::INSN_SIZE < prog.len() {
            let insn = ebpf::get_insn(prog, insn_ptr);
            insn_ptr += 1;
            let _dst = insn.dst as usize;
            let _src = insn.src as usize;

            // iterations += 1;
            match insn.opc {
                // BPF_LD class
                // LD_ABS_* and LD_IND_* are supposed to load pointer to data from buffer.
                // Since this pointer is constant, and since we already know it (mem), do not
                // bother re-fetching it, just use mem already.
                ebpf::LD_ABS_B   => reg[0] = {
                    let x = (mem_ptr as i64 + (insn.imm as i32) as i64) as *const u8;
                    check_mem_load(mem, x as u64, 8, insn_ptr)?;
                    // println!("LD_ABS_B {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    mem[(x as usize - mem_ptr as usize)] as u64
                },
                ebpf::LD_ABS_H   => reg[0] = {
                    let x = (mem_ptr as i64 + (insn.imm as i32) as i64) as *const u16;
                    check_mem_load(mem, x as u64, 8, insn_ptr)?;
                    // println!("LD_ABS_H {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    (u16::from_be_bytes([mem[(x as usize - mem_ptr as usize) + 1], mem[(x as usize - mem_ptr as usize)]])) as u64
                },
                ebpf::LD_ABS_W   => reg[0] = {
                    let x = (mem_ptr as i64 + (insn.imm as i32) as i64) as *const u32;
                    check_mem_load(mem, x as u64, 8, insn_ptr)?;
                    // println!("LD_ABS_W {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    (u32::from_be_bytes([mem[(x as usize - mem_ptr as usize) + 3], mem[(x as usize - mem_ptr as usize) + 2],
                        mem[(x as usize - mem_ptr as usize) + 1], mem[(x as usize - mem_ptr as usize)]])) as u64
                },
                ebpf::LD_ABS_DW  => reg[0] = {
                    let x = (mem.as_ptr() as i64 + (insn.imm as i32) as i64) as *const u64;
                    check_mem_load(mem, x as u64, 8, insn_ptr)?;
                    // println!("LD_ABS_DW {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    u64::from_be_bytes([mem[(x as usize - mem_ptr as usize) + 7], mem[(x as usize - mem_ptr as usize) + 6],
                        mem[(x as usize - mem_ptr as usize) + 5], mem[(x as usize - mem_ptr as usize) + 4],
                        mem[(x as usize - mem_ptr as usize) + 3], mem[(x as usize - mem_ptr as usize) + 2],
                        mem[(x as usize - mem_ptr as usize) + 1], mem[(x as usize - mem_ptr as usize)]])
                },
                ebpf::LD_IND_B   => reg[0] = {
                    let x = (mem_ptr as i64 + reg[_src] as i64 + (insn.imm as i32) as i64) as *const u8;
                    // println!("LD_IND_B mem[x]: {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    check_mem_load(mem, x as u64, 8, insn_ptr)?;
                    mem[(x as usize - mem_ptr as usize)] as u64
                },
                ebpf::LD_IND_H   => reg[0] = {
                    let x = (mem_ptr as i64 + reg[_src] as i64 + (insn.imm as i32) as i64) as *const u16;
                    check_mem_load(mem, x as u64, 8, insn_ptr)?;
                    // println!("LD_IND_H {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    (u16::from_be_bytes([mem[(x as usize - mem_ptr as usize) + 1], mem[(x as usize - mem_ptr as usize)]])) as u64
                },
                ebpf::LD_IND_W   => reg[0] = {
                    let x = (mem.as_ptr() as i64 + reg[_src] as i64 + (insn.imm as i32) as i64) as *const u32;
                    check_mem_load(mem, x as u64, 8, insn_ptr)?;
                    // println!("LD_IND_W {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    (u32::from_be_bytes([mem[(x as usize - mem_ptr as usize) + 3], mem[(x as usize - mem_ptr as usize) + 2],
                        mem[(x as usize - mem_ptr as usize) + 1], mem[(x as usize - mem_ptr as usize)]])) as u64
                },
                ebpf::LD_IND_DW  => reg[0] = {
                    let x = (mem.as_ptr() as i64 + reg[_src] as i64 + (insn.imm as i32) as i64) as *const u64;
                    check_mem_load(mem, x as u64, 8, insn_ptr)?;
                    // println!("LD_IND_DW {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    u64::from_be_bytes([mem[(x as usize - mem_ptr as usize) + 7], mem[(x as usize - mem_ptr as usize) + 6],
                        mem[(x as usize - mem_ptr as usize) + 5], mem[(x as usize - mem_ptr as usize) + 4],
                        mem[(x as usize - mem_ptr as usize) + 3], mem[(x as usize - mem_ptr as usize) + 2],
                        mem[(x as usize - mem_ptr as usize) + 1], mem[(x as usize - mem_ptr as usize)]])
                },

                ebpf::LD_DW_IMM  => {
                    let next_insn = ebpf::get_insn(prog, insn_ptr);
                    insn_ptr += 1;
                    // println!("LD_DW_IMM last 32: {:#x}, first 32 {:#x}", insn.imm, next_insn.imm);
                    reg[_dst] = ((insn.imm as u32) as u64) + ((next_insn.imm as u64) << 32);
                },

                // BPF_LDX class
                ebpf::LD_B_REG   => reg[_dst] = {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_src] as i64 + insn.off as i64) as *const u8;
                    check_mem_load(mem, x as u64, 1, insn_ptr)?;
                    // println!("LD_B_REG {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    mem[(x as usize - mem.as_ptr() as usize)] as u64
                },
                ebpf::LD_H_REG   => reg[_dst] = {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_src] as i64 + insn.off as i64) as *const u16;
                    check_mem_load(mem, x as u64, 2, insn_ptr)?;
                    // println!("LD_H_REG {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    (u16::from_be_bytes([mem[(x as usize - mem_ptr as usize) + 1], mem[(x as usize - mem_ptr as usize)]])) as u64
                },
                ebpf::LD_W_REG   => reg[_dst] = {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_src] as i64 + insn.off as i64) as *const u32;
                    check_mem_load(mem, x as u64, 4, insn_ptr)?;
                    // println!("LD_W_REG {:#x}", mem[(x as usize - mem.as_ptr() as usize)] as u32);
                    (u32::from_be_bytes([mem[(x as usize - mem_ptr as usize) + 3], mem[(x as usize - mem_ptr as usize) + 2],
                        mem[(x as usize - mem_ptr as usize) + 1], mem[(x as usize - mem_ptr as usize)]])) as u64
                },
                ebpf::LD_DW_REG  => reg[_dst] = {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_src] as i64 + insn.off as i64) as *const u64;
                    check_mem_load(mem, x as u64, 8, insn_ptr)?;
                    // println!("LD_DW_REG {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    u64::from_be_bytes([mem[(x as usize - mem_ptr as usize) + 7], mem[(x as usize - mem_ptr as usize) + 6],
                        mem[(x as usize - mem_ptr as usize) + 5], mem[(x as usize - mem_ptr as usize) + 4],
                        mem[(x as usize - mem_ptr as usize) + 3], mem[(x as usize - mem_ptr as usize) + 2],
                        mem[(x as usize - mem_ptr as usize) + 1], mem[(x as usize - mem_ptr as usize)]])
                },

                // BPF_ST class
                ebpf::ST_B_IMM   => {
                    let x = (reg[_dst] as i64 + insn.off as i64) as *mut u8;
                    check_mem_store(mem, x as u64, 1, insn_ptr)?;
                    // println!("ST_B_IMM {:#x}", mem[(x as usize - mem_ptr as usize)]);
                    mem[(x as usize - mem_ptr as usize)] = insn.imm as u8;
                },
                ebpf::ST_H_IMM   => {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_dst] as i64 + insn.off as i64) as *mut u16;
                    check_mem_store(mem, x as u64, 2, insn_ptr)?;
                    // println!("ST_H_IMM {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    let bytes = (insn.imm as u16).to_le_bytes();
                    mem[(x as usize - mem_ptr as usize)] = bytes[0];
                    mem[(x as usize - mem_ptr as usize) + 1] = bytes[1];
                },
                ebpf::ST_W_IMM   => {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_dst] as i64 + insn.off as i64) as *mut u32;
                    check_mem_store(mem, x as u64, 4, insn_ptr)?;
                    // println!("ST_W_IMM {:#x}", mem[(x as usize - mem_ptr as usize)]);
                    let bytes = (insn.imm as u32).to_le_bytes();
                    mem[(x as usize - mem_ptr as usize)] = bytes[0];
                    mem[(x as usize - mem_ptr as usize) + 1] = bytes[1];
                    mem[(x as usize - mem_ptr as usize) + 2] = bytes[2];
                    mem[(x as usize - mem_ptr as usize) + 3] = bytes[3];
                },
                ebpf::ST_DW_IMM  => {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_dst] as i64 + insn.off as i64) as *mut u64;
                    check_mem_store(mem, x as u64, 8, insn_ptr)?;
                    // println!("ST_DW_IMM {:#x}", mem[(x as usize - mem_ptr as usize)]);
                    let bytes = (insn.imm as u64).to_le_bytes();
                    mem[(x as usize - mem_ptr as usize)] = bytes[0];
                    mem[(x as usize - mem_ptr as usize) + 1] = bytes[1];
                    mem[(x as usize - mem_ptr as usize) + 2] = bytes[2];
                    mem[(x as usize - mem_ptr as usize) + 3] = bytes[3];
                    mem[(x as usize - mem_ptr as usize) + 4] = bytes[4];
                    mem[(x as usize - mem_ptr as usize) + 5] = bytes[5];
                    mem[(x as usize - mem_ptr as usize) + 6] = bytes[6];
                    mem[(x as usize - mem_ptr as usize) + 7] = bytes[7];
                },

                // BPF_STX class
                ebpf::ST_B_REG   => {
                    let x = (reg[_dst] as i64 + insn.off as i64) as *mut u8;
                    check_mem_store(mem, x as u64, 1, insn_ptr)?;
                    // println!("ST_B_REG {:#x}", reg[_src] as u8);
                    mem[(x as usize - mem_ptr as usize)] = reg[_src] as u8;
                },
                ebpf::ST_H_REG   => {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_dst] as i64 + insn.off as i64) as *mut u16;
                    check_mem_store(mem, x as u64, 2, insn_ptr)?;
                    // println!("ST_H_REG {:#x}", reg[_src] as u16);
                    let bytes = (reg[_src] as u16).to_le_bytes();
                    mem[(x as usize - mem_ptr as usize)] = bytes[0];
                    mem[(x as usize - mem_ptr as usize) + 1] = bytes[1];
                },
                ebpf::ST_W_REG   => {
                    #[allow(cast_ptr_alignment)]
                    // println!("ST_W_REG _dst: {:?}, reg[_dst]: {:?}, insn.off: {:?}", _dst as u64, reg[_dst] as i64, insn.off as i64);
                    let x = (reg[_dst] as i64 + insn.off as i64) as *mut u32;
                    check_mem_store(mem, x as u64, 4, insn_ptr)?;
                    let bytes = (reg[_src] as u32).to_le_bytes();
                    mem[(x as usize - mem_ptr as usize)] = bytes[0];
                    mem[(x as usize - mem_ptr as usize) + 1] = bytes[1];
                    mem[(x as usize - mem_ptr as usize) + 2] = bytes[2];
                    mem[(x as usize - mem_ptr as usize) + 3] = bytes[3];
                },
                ebpf::ST_DW_REG  => {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_dst] as i64 + insn.off as i64) as *mut u64;
                    check_mem_store(mem, x as u64, 8, insn_ptr)?;
                    // println!("ST_DW_REG {:#x}", reg[_src] as u64);
                    let bytes = (reg[_src] as u64).to_le_bytes();
                    mem[(x as usize - mem_ptr as usize)] = bytes[0];
                    mem[(x as usize - mem_ptr as usize) + 1] = bytes[1];
                    mem[(x as usize - mem_ptr as usize) + 2] = bytes[2];
                    mem[(x as usize - mem_ptr as usize) + 3] = bytes[3];
                    mem[(x as usize - mem_ptr as usize) + 4] = bytes[4];
                    mem[(x as usize - mem_ptr as usize) + 5] = bytes[5];
                    mem[(x as usize - mem_ptr as usize) + 6] = bytes[6];
                    mem[(x as usize - mem_ptr as usize) + 7] = bytes[7];
                },
                ebpf::ST_W_XADD  => unimplemented!(),
                ebpf::ST_DW_XADD => unimplemented!(),

                // BPF_ALU class
                // TODO Check how overflow works in kernel. Should we &= U32MAX all src register value
                // before we do the operation?
                // Cf ((0x11 << 32) - (0x1 << 32)) as u32 VS ((0x11 << 32) as u32 - (0x1 << 32) as u32
                ebpf::ADD32_IMM  => reg[_dst] = (reg[_dst] as i32).wrapping_add(insn.imm)         as u64, //((reg[_dst] & U32MAX) + insn.imm  as u64)     & U32MAX,
                ebpf::ADD32_REG  => reg[_dst] = (reg[_dst] as i32).wrapping_add(reg[_src] as i32) as u64, //((reg[_dst] & U32MAX) + (reg[_src] & U32MAX)) & U32MAX,
                ebpf::SUB32_IMM  => reg[_dst] = (reg[_dst] as i32).wrapping_sub(insn.imm)         as u64,
                ebpf::SUB32_REG  => reg[_dst] = (reg[_dst] as i32).wrapping_sub(reg[_src] as i32) as u64,
                ebpf::MUL32_IMM  => reg[_dst] = (reg[_dst] as i32).wrapping_mul(insn.imm)         as u64,
                ebpf::MUL32_REG  => reg[_dst] = (reg[_dst] as i32).wrapping_mul(reg[_src] as i32) as u64,
                ebpf::DIV32_IMM  => reg[_dst] = (reg[_dst] as u32 / insn.imm              as u32) as u64,
                ebpf::DIV32_REG  => {
                    if reg[_src] == 0 {
                        debug!("Error: division by 0");
                        Err(ErrorCode::NOSUPPORT)?;
                    }
                    reg[_dst] = (reg[_dst] as u32 / reg[_src] as u32) as u64;
                },
                ebpf::OR32_IMM   =>   reg[_dst] = (reg[_dst] as u32             | insn.imm  as u32) as u64,
                ebpf::OR32_REG   =>   reg[_dst] = (reg[_dst] as u32             | reg[_src] as u32) as u64,
                ebpf::AND32_IMM  =>   reg[_dst] = (reg[_dst] as u32             & insn.imm  as u32) as u64,
                ebpf::AND32_REG  =>   reg[_dst] = (reg[_dst] as u32             & reg[_src] as u32) as u64,
                ebpf::LSH32_IMM  =>   reg[_dst] = (reg[_dst] as u32).wrapping_shl(insn.imm  as u32) as u64,
                ebpf::LSH32_REG  =>   reg[_dst] = (reg[_dst] as u32).wrapping_shl(reg[_src] as u32) as u64,
                ebpf::RSH32_IMM  =>   reg[_dst] = (reg[_dst] as u32).wrapping_shr(insn.imm  as u32) as u64,
                ebpf::RSH32_REG  =>   reg[_dst] = (reg[_dst] as u32).wrapping_shr(reg[_src] as u32) as u64,
                ebpf::NEG32      => { reg[_dst] = (reg[_dst] as i32).wrapping_neg()                 as u64; reg[_dst] &= U32MAX; },
                ebpf::MOD32_IMM  =>   reg[_dst] = (reg[_dst] as u32             % insn.imm  as u32) as u64,
                ebpf::MOD32_REG  => {
                    if reg[_src] == 0 {
                        debug!("Error: division by 0");
                        Err(ErrorCode::NOSUPPORT)?;
                    }
                    reg[_dst] = (reg[_dst] as u32 % reg[_src] as u32) as u64;
                },
                ebpf::XOR32_IMM  =>   reg[_dst] = (reg[_dst] as u32             ^ insn.imm  as u32) as u64,
                ebpf::XOR32_REG  =>   reg[_dst] = (reg[_dst] as u32             ^ reg[_src] as u32) as u64,
                ebpf::MOV32_IMM  =>   reg[_dst] = insn.imm   as u32                                 as u64,
                ebpf::MOV32_REG  =>   reg[_dst] = (reg[_src] as u32)                                as u64,
                ebpf::ARSH32_IMM => { reg[_dst] = (reg[_dst] as i32).wrapping_shr(insn.imm  as u32) as u64; reg[_dst] &= U32MAX; },
                ebpf::ARSH32_REG => { reg[_dst] = (reg[_dst] as i32).wrapping_shr(reg[_src] as u32) as u64; reg[_dst] &= U32MAX; },
                ebpf::LE         => {
                    reg[_dst] = match insn.imm {
                        16 => (reg[_dst] as u16).to_le() as u64,
                        32 => (reg[_dst] as u32).to_le() as u64,
                        64 =>  reg[_dst].to_le(),
                        _  => unreachable!(),
                    };
                },
                ebpf::BE         => {
                    reg[_dst] = match insn.imm {
                        16 => (reg[_dst] as u16).to_be() as u64,
                        32 => (reg[_dst] as u32).to_be() as u64,
                        64 =>  reg[_dst].to_be(),
                        _  => unreachable!(),
                    };
                },

                // BPF_ALU64 class
                ebpf::ADD64_IMM  => reg[_dst] = reg[_dst].wrapping_add(insn.imm as u64),
                ebpf::ADD64_REG  => reg[_dst] = reg[_dst].wrapping_add(reg[_src]),
                ebpf::SUB64_IMM  => reg[_dst] = reg[_dst].wrapping_sub(insn.imm as u64),
                ebpf::SUB64_REG  => reg[_dst] = reg[_dst].wrapping_sub(reg[_src]),
                ebpf::MUL64_IMM  => reg[_dst] = reg[_dst].wrapping_mul(insn.imm as u64),
                ebpf::MUL64_REG  => reg[_dst] = reg[_dst].wrapping_mul(reg[_src]),
                ebpf::DIV64_IMM  => reg[_dst]                       /= insn.imm as u64,
                ebpf::DIV64_REG  => {
                    if reg[_src] == 0 {
                        debug!("Error: division by 0");
                        Err(ErrorCode::NOSUPPORT)?;
                    }
                    reg[_dst] /= reg[_src];
                },
                ebpf::OR64_IMM   => reg[_dst] |=  insn.imm as u64,
                ebpf::OR64_REG   => reg[_dst] |=  reg[_src],
                ebpf::AND64_IMM  => reg[_dst] &=  insn.imm as u64,
                ebpf::AND64_REG  => reg[_dst] &=  reg[_src],
                ebpf::LSH64_IMM  => reg[_dst] <<= insn.imm as u64,
                ebpf::LSH64_REG  => reg[_dst] <<= reg[_src],
                ebpf::RSH64_IMM  => reg[_dst] >>= insn.imm as u64,
                ebpf::RSH64_REG  => reg[_dst] >>= reg[_src],
                ebpf::NEG64      => reg[_dst] = -(reg[_dst] as i64) as u64,
                ebpf::MOD64_IMM  => reg[_dst] %=  insn.imm as u64,
                ebpf::MOD64_REG  => {
                    if reg[_src] == 0 {
                        debug!("Error: division by 0");
                        Err(ErrorCode::NOSUPPORT)?;
                    }
                    reg[_dst] %= reg[_src];
                },
                ebpf::XOR64_IMM  => reg[_dst] ^= insn.imm  as u64,
                ebpf::XOR64_REG  => reg[_dst] ^= reg[_src],
                ebpf::MOV64_IMM  => reg[_dst] =  insn.imm  as u64,
                ebpf::MOV64_REG  => reg[_dst] =  reg[_src],
                ebpf::ARSH64_IMM => reg[_dst] = (reg[_dst] as i64 >> insn.imm)  as u64,
                ebpf::ARSH64_REG => reg[_dst] = (reg[_dst] as i64 >> reg[_src]) as u64,

                // BPF_JMP class
                // TODO: check this actually works as expected for signed / unsigned ops
                ebpf::JA         =>                                           { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 }
                ebpf::JEQ_IMM    => if  reg[_dst] == insn.imm as u64          { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JEQ_REG    => if  reg[_dst] == reg[_src]                { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JGT_IMM    => if  reg[_dst] >  insn.imm as u64          { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JGT_REG    => if  reg[_dst] >  reg[_src]                { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JGE_IMM    => if  reg[_dst] >= insn.imm as u64          { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JGE_REG    => if  reg[_dst] >= reg[_src]                { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JLT_IMM    => if  reg[_dst] <  insn.imm as u64          { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JLT_REG    => if  reg[_dst] <  reg[_src]                { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JLE_IMM    => if  reg[_dst] <= insn.imm as u64          { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JLE_REG    => if  reg[_dst] <= reg[_src]                { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JSET_IMM   => if  reg[_dst] &  insn.imm as u64 != 0     { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JSET_REG   => if  reg[_dst] &  reg[_src]       != 0     { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JNE_IMM    => if  reg[_dst] != insn.imm as u64          { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JNE_REG    => if  reg[_dst] != reg[_src]                { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JSGT_IMM   => if  reg[_dst] as i64 >  insn.imm  as i64  { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JSGT_REG   => if  reg[_dst] as i64 >  reg[_src] as i64  { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JSGE_IMM   => if  reg[_dst] as i64 >= insn.imm  as i64  { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JSGE_REG   => if  reg[_dst] as i64 >= reg[_src] as i64  { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JSLT_IMM   => if (reg[_dst] as i64) <  insn.imm  as i64 { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JSLT_REG   => if (reg[_dst] as i64) <  reg[_src] as i64 { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JSLE_IMM   => if (reg[_dst] as i64) <= insn.imm  as i64 { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                ebpf::JSLE_REG   => if (reg[_dst] as i64) <= reg[_src] as i64 { insn_ptr = (insn_ptr as i16 + insn.off) as usize;                 },
                // Do not delegate the check to the verifier, since registered functions can be
                // changed after the program has been verified.
                ebpf::CALL       => {
                    debug!("Helper functions are not supported!");
                    Err(ErrorCode::NOSUPPORT)?;
                },
                ebpf::TAIL_CALL  => unimplemented!(),
                ebpf::EXIT       => {
                    // println!("Iterations: {:?}", iterations);
                    return Ok(reg[0]);
                },

                _                => unreachable!()
            }
        }

        unreachable!()
    }

    fn check_mem(addr: u64, len: usize, access_type: &str, insn_ptr: usize,
             mem: &[u8], stack: u64) -> Result<(), ErrorCode> {

        let mem_ptr = mem.as_ptr();
        let mem_len = mem.len();

        if mem_ptr as u64 <= addr && addr + len as u64 <= mem_ptr as u64 + mem_len as u64 {
            return Ok(())
        }
        if stack <= addr && addr + len as u64 <= stack + ebpf::STACK_SIZE as u64 {
            return Ok(())
        }

        panic!(
            "Error: out of bounds memory {} (insn #{:?}), addr {:#x}, size {:?}\n mem: {:#x}/{:#x}, stack: {:#x}/{:#x}",
            access_type, insn_ptr, addr, len,
            mem_ptr as u64, mem_len,
            stack, ebpf::STACK_SIZE as u64
        )
    }
}
