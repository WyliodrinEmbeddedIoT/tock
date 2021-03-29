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
// #![doc(html_logo_url = "https://raw.githubusercontent.com/qmonnet/rbpf/master/misc/rbpf.png",
//        html_favicon_url = "https://raw.githubusercontent.com/qmonnet/rbpf/master/misc/rbpf.ico")]

// #![warn(missing_docs)]
// // There are unused mut warnings due to unsafe code.
// #![allow(unused_mut)]
// // Allows old-style clippy
// #![allow(renamed_and_removed_lints)]

#![cfg_attr(feature = "cargo-clippy", allow(redundant_field_names, single_match, cast_lossless, doc_markdown, match_same_arms, unreadable_literal))]
#![feature(alloc)]

use kernel::common::byteorder::{ByteOrder, LittleEndian};
use kernel::{ReturnCode};
use crate::bpf::ebpf;
use crate::bpf::verifier;

extern crate alloc;
use alloc::vec::Vec;
use alloc::vec;

/// eBPF verification function that returns an error if the program does not meet its requirements.
///
/// Some examples of things the verifier may reject the program for:
///
///   - Program does not terminate.
///   - Unknown instructions.
///   - Bad formed instruction.
///   - Unknown eBPF helper index.
pub type Verifier = fn(prog: &[u8]) -> Result<(), ReturnCode>;


// A metadata buffer with two offset indications. It can be used in one kind of eBPF VM to simulate
// the use of a metadata buffer each time the program is executed, without the user having to
// actually handle it. The offsets are used to tell the VM where in the buffer the pointers to
// packet data start and end should be stored each time the program is run on a new packet.
struct MetaBuff {
    data_offset:     usize,
    data_end_offset: usize,
    buffer:          Vec<u8>,
}

/// A virtual machine to run eBPF program. This kind of VM is used for programs expecting to work
/// on a metadata buffer containing pointers to packet data.
///
/// # Examples
///
/// ```
/// let prog = &[
///     0x79, 0x11, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, // Load mem from mbuff at offset 8 into R1.
///     0x69, 0x10, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, // ldhx r1[2], r0
///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
/// ];
/// let mem = &mut [
///     0xaa, 0xbb, 0x11, 0x22, 0xcc, 0xdd
/// ];
///
/// // Just for the example we create our metadata buffer from scratch, and we store the pointers
/// // to packet data start and end in it.
/// let mut mbuff = [0u8; 32];
/// unsafe {
///     let mut data     = mbuff.as_ptr().offset(8)  as *mut u64;
///     let mut data_end = mbuff.as_ptr().offset(24) as *mut u64;
///     *data     = mem.as_ptr() as u64;
///     *data_end = mem.as_ptr() as u64 + mem.len() as u64;
/// }
///
/// // Instantiate a VM.
/// let mut vm = rbpf::EbpfVmMbuff::new(Some(prog)).unwrap();
///
/// // Provide both a reference to the packet data, and to the metadata buffer.
/// let res = vm.execute_program(mem, &mut mbuff).unwrap();
/// assert_eq!(res, 0x2211);
/// ```
pub struct EbpfVmMbuff<'a> {
    prog:     Option<&'a [u8]>,
    verifier: Verifier,
}

impl<'a> EbpfVmMbuff<'a> {

    /// Create a new virtual machine instance, and load an eBPF program into that instance.
    /// When attempting to load the program, it passes through a simple verifier.
    ///
    /// # Examples
    ///
    /// ```
    /// let prog = &[
    ///     0x79, 0x11, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, // Load mem from mbuff into R1.
    ///     0x69, 0x10, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, // ldhx r1[2], r0
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    ///
    /// // Instantiate a VM.
    /// let mut vm = rbpf::EbpfVmMbuff::new(Some(prog)).unwrap();
    /// ```
    pub fn new(prog: Option<&'a [u8]>) -> Result<EbpfVmMbuff<'a>, ReturnCode> {
        if let Some(prog) = prog {
            verifier::check(prog)?;
        }

        Ok(EbpfVmMbuff {
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
    ///     0x79, 0x11, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, // Load mem from mbuff into R1.
    ///     0x69, 0x10, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, // ldhx r1[2], r0
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    ///
    /// // Instantiate a VM.
    /// let mut vm = rbpf::EbpfVmMbuff::new(Some(prog1)).unwrap();
    /// vm.set_program(prog2).unwrap();
    /// ```
    pub fn set_program(&mut self, prog: &'a [u8]) -> Result<(), ReturnCode> {
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
    pub fn set_verifier(&mut self, verifier: Verifier) -> Result<(), ReturnCode> {
        if let Some(prog) = self.prog {
            verifier(prog)?;
        }
        self.verifier = verifier;
        Ok(())
    }

    /// Register a built-in or user-defined helper function in order to use it later from within
    /// the eBPF program. The helper is registered into a hashmap, so the `key` can be any `u32`.
    ///
    /// If using JIT-compiled eBPF programs, be sure to register all helpers before compiling the
    /// program. You should be able to change registered helpers after compiling, but not to add
    /// new ones (i.e. with new keys).
    ///
    /// # Examples
    ///
    /// ```
    /// use rbpf::helpers;
    ///
    /// // This program was compiled with clang, from a C program containing the following single
    /// // instruction: `return bpf_trace_printk("foo %c %c %c\n", 10, 1, 2, 3);`
    /// let prog = &[
    ///     0x18, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // load 0 as u64 into r1 (That would be
    ///     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // replaced by tc by the address of
    ///                                                     // the format string, in the .map
    ///                                                     // section of the ELF file).
    ///     0xb7, 0x02, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, // mov r2, 10
    ///     0xb7, 0x03, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // mov r3, 1
    ///     0xb7, 0x04, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, // mov r4, 2
    ///     0xb7, 0x05, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, // mov r5, 3
    ///     0x85, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, // call helper with key 6
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    ///
    /// // Instantiate a VM.
    /// let mut vm = rbpf::EbpfVmMbuff::new(Some(prog)).unwrap();
    ///
    /// // Register a helper.
    /// // On running the program this helper will print the content of registers r3, r4 and r5 to
    /// // standard output.
    /// vm.register_helper(6, helpers::bpf_trace_printf).unwrap();
    /// ```
    // pub fn register_helper(&mut self, key: u32, function: Helper) -> Result<(), ReturnCode> {
    //     self.helpers.insert(key, function);
    //     Ok(())
    // }

    /// Execute the program loaded, with the given packet data and metadata buffer.
    ///
    /// If the program is made to be compatible with Linux kernel, it is expected to load the
    /// address of the beginning and of the end of the memory area used for packet data from the
    /// metadata buffer, at some appointed offsets. It is up to the user to ensure that these
    /// pointers are correctly stored in the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// let prog = &[
    ///     0x79, 0x11, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, // Load mem from mbuff into R1.
    ///     0x69, 0x10, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, // ldhx r1[2], r0
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    /// let mem = &mut [
    ///     0xaa, 0xbb, 0x11, 0x22, 0xcc, 0xdd
    /// ];
    ///
    /// // Just for the example we create our metadata buffer from scratch, and we store the
    /// // pointers to packet data start and end in it.
    /// let mut mbuff = [0u8; 32];
    /// unsafe {
    ///     let mut data     = mbuff.as_ptr().offset(8)  as *mut u64;
    ///     let mut data_end = mbuff.as_ptr().offset(24) as *mut u64;
    ///     *data     = mem.as_ptr() as u64;
    ///     *data_end = mem.as_ptr() as u64 + mem.len() as u64;
    /// }
    ///
    /// // Instantiate a VM.
    /// let mut vm = rbpf::EbpfVmMbuff::new(Some(prog)).unwrap();
    ///
    /// // Provide both a reference to the packet data, and to the metadata buffer.
    /// let res = vm.execute_program(mem, &mut mbuff).unwrap();
    /// assert_eq!(res, 0x2211);
    /// ```
    #[allow(unknown_lints)]
    #[allow(cyclomatic_complexity)]
    pub fn execute_program(&self, mem: &[u8], mbuff: &[u8]) -> Result<u64, ReturnCode> {
        const U32MAX: u64 = u32::MAX as u64;

        let prog = match self.prog { 
            Some(prog) => prog,
            //"Error: No program set, call prog_set() to load one"
            None => Err(ReturnCode::EINVAL)?,
        };
        let stack = vec![0u8;ebpf::STACK_SIZE];

        // R1 points to beginning of memory area, R10 to stack
        let mut reg: [u64;11] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, stack.as_ptr() as u64 + stack.len() as u64
        ];
        if !mbuff.is_empty() {
            reg[1] = mbuff.as_ptr() as u64;
        }
        else if !mem.is_empty() {
            reg[1] = mem.as_ptr() as u64;
        }

        let check_mem_load = | addr: u64, len: usize, insn_ptr: usize | {
            EbpfVmMbuff::check_mem(addr, len, "load", insn_ptr, mbuff, mem, &stack)
        };
        let check_mem_store = | addr: u64, len: usize, insn_ptr: usize | {
            EbpfVmMbuff::check_mem(addr, len, "store", insn_ptr, mbuff, mem, &stack)
        };

        // Loop on instructions
        let mut insn_ptr:usize = 0;
        while insn_ptr * ebpf::INSN_SIZE < prog.len() {
            let insn = ebpf::get_insn(prog, insn_ptr);
            insn_ptr += 1;
            let _dst = insn.dst as usize;
            let _src = insn.src as usize;

            match insn.opc {

                // BPF_LD class
                // LD_ABS_* and LD_IND_* are supposed to load pointer to data from metadata buffer.
                // Since this pointer is constant, and since we already know it (mem), do not
                // bother re-fetching it, just use mem already.
                ebpf::LD_ABS_B   => reg[0] = {
                    let x = (mem.as_ptr() as u64 + (insn.imm as u32) as u64) as *const u8;
                    check_mem_load(mem, x as u64, 8, insn_ptr)?;
                    // println!("LD_ABS_B {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    mem[(x as usize - mem.as_ptr() as usize)] as u64
                },
                ebpf::LD_ABS_H   => reg[0] = {
                    let x = (mem.as_ptr() as u64 + (insn.imm as u32) as u64) as *const u16;
                    check_mem_load(mem, x as u64, 8, insn_ptr)?;
                    // println!("LD_ABS_H {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    (mem[(x as usize - mem.as_ptr() as usize)] << 8 + mem[(x as usize - mem.as_ptr() as usize) + 1]) as u64
                },
                ebpf::LD_ABS_W   => reg[0] = {
                    let x = (mem.as_ptr() as u64 + (insn.imm as u32) as u64) as *const u32;
                    check_mem_load(mem, x as u64, 8, insn_ptr)?;
                    // println!("LD_ABS_W {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    (mem[(x as usize - mem.as_ptr() as usize)] << 24 + mem[(x as usize - mem.as_ptr() as usize) + 1] << 16 +
                        mem[(x as usize - mem.as_ptr() as usize) + 2] << 8 + mem[(x as usize - mem.as_ptr() as usize) + 3]) as u64
                },
                ebpf::LD_ABS_DW  => reg[0] = {
                    let x = (mem.as_ptr() as u64 + (insn.imm as u32) as u64) as *const u64;
                    check_mem_load(mem, x as u64, 8, insn_ptr)?;
                    // println!("LD_ABS_DW {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    (mem[(x as usize - mem.as_ptr() as usize)] << 56 + mem[(x as usize - mem.as_ptr() as usize) + 1] << 48 +
                        mem[(x as usize - mem.as_ptr() as usize) + 2] << 40 + mem[(x as usize - mem.as_ptr() as usize) + 3] << 32 +
                        mem[(x as usize - mem.as_ptr() as usize) + 4] << 24 + mem[(x as usize - mem.as_ptr() as usize) + 5] << 16 +
                        mem[(x as usize - mem.as_ptr() as usize) + 6] << 8 + mem[(x as usize - mem.as_ptr() as usize) + 7]) as u64
                },
                ebpf::LD_IND_B   => reg[0] = {
                    let x = (mem.as_ptr() as u64 + reg[_src] + (insn.imm as u32) as u64) as *const u8;
                    check_mem_load(mem, x as u64, 8, insn_ptr)?;
                    // println!("LD_IND_B {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    mem[(x as usize - mem.as_ptr() as usize)] as u64
                },
                ebpf::LD_IND_H   => reg[0] = {
                    let x = (mem.as_ptr() as u64 + reg[_src] + (insn.imm as u32) as u64) as *const u16;
                    check_mem_load(mem, x as u64, 8, insn_ptr)?;
                    // println!("LD_IND_H {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    (mem[(x as usize - mem.as_ptr() as usize)] << 8 + mem[(x as usize - mem.as_ptr() as usize) + 1]) as u64
                },
                ebpf::LD_IND_W   => reg[0] = {
                    let x = (mem.as_ptr() as u64 + reg[_src] + (insn.imm as u32) as u64) as *const u32;
                    check_mem_load(mem, x as u64, 8, insn_ptr)?;
                    // println!("LD_IND_W {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    (mem[(x as usize - mem.as_ptr() as usize)] << 24 + mem[(x as usize - mem.as_ptr() as usize) + 1] << 16 +
                        mem[(x as usize - mem.as_ptr() as usize) + 2] << 8 + mem[(x as usize - mem.as_ptr() as usize) + 3]) as u64
                },
                ebpf::LD_IND_DW  => reg[0] = {
                    let x = (mem.as_ptr() as u64 + reg[_src] + (insn.imm as u32) as u64) as *const u64;
                    check_mem_load(mem, x as u64, 8, insn_ptr)?;
                    // println!("LD_IND_DW {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    (mem[(x as usize - mem.as_ptr() as usize)] << 56 + mem[(x as usize - mem.as_ptr() as usize) + 1] << 48 +
                        mem[(x as usize - mem.as_ptr() as usize) + 2] << 40 + mem[(x as usize - mem.as_ptr() as usize) + 3] << 32 +
                        mem[(x as usize - mem.as_ptr() as usize) + 4] << 24 + mem[(x as usize - mem.as_ptr() as usize) + 5] << 16 +
                        mem[(x as usize - mem.as_ptr() as usize) + 6] << 8 + mem[(x as usize - mem.as_ptr() as usize) + 7]) as u64
                },

                ebpf::LD_DW_IMM  => {
                    let next_insn = ebpf::get_insn(prog, insn_ptr);
                    insn_ptr += 1;
                    reg[_dst] = ((insn.imm as u32) as u64) + ((next_insn.imm as u64) << 32);
                },

                // BPF_LDX class
                ebpf::LD_B_REG   => reg[_dst] = {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_src] as u64 + insn.off as u64) as *const u8;
                    check_mem_load(mem, x as u64, 1, insn_ptr)?;
                    // println!("LD_B_REG {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    mem[(x as usize - mem.as_ptr() as usize)] as u64
                },
                ebpf::LD_H_REG   => reg[_dst] = {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_src] as u64 + insn.off as u64) as *const u16;
                    check_mem_load(mem, x as u64, 2, insn_ptr)?;
                    // println!("LD_H_REG {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    (mem[(x as usize - mem.as_ptr() as usize)] << 8 + mem[(x as usize - mem.as_ptr() as usize) + 1]) as u64
                },
                ebpf::LD_W_REG   => reg[_dst] = {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_src] as u64 + insn.off as u64) as *const u32;
                    check_mem_load(mem, x as u64, 4, insn_ptr)?;
                    // println!("LD_W_REG {:#x}", (mem[(x as usize - mem.as_ptr() as usize)] as u32) << 24 + (mem[(x as usize - mem.as_ptr() as usize) + 1] as u32) << 16);
                    // println!("LD_W_REG {:#x}", (mem[(x as usize - mem.as_ptr() as usize) + 1] as u32) << 16);
                    // println!("LD_W_REG {:#x}", (mem[(x as usize - mem.as_ptr() as usize) + 2] as u32) << 8);
                    // println!("LD_W_REG {:#x}", mem[(x as usize - mem.as_ptr() as usize) + 3] as u32);
                    let val = u32::from_be_bytes([mem[(x as usize - mem.as_ptr() as usize)], mem[(x as usize - mem.as_ptr() as usize) + 1],
                        mem[(x as usize - mem.as_ptr() as usize) + 2], mem[(x as usize - mem.as_ptr() as usize) + 3]]);
                    val as u64
                },
                ebpf::LD_DW_REG  => reg[_dst] = {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_src] as u64 + insn.off as u64) as *const u64;
                    check_mem_load(mem, x as u64, 8, insn_ptr)?;
                    // println!("LD_DW_REG {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    (mem[(x as usize - mem.as_ptr() as usize)] << 56 + mem[(x as usize - mem.as_ptr() as usize) + 1] << 48 +
                        mem[(x as usize - mem.as_ptr() as usize) + 2] << 40 + mem[(x as usize - mem.as_ptr() as usize) + 3] << 32 +
                        mem[(x as usize - mem.as_ptr() as usize) + 4] << 24 + mem[(x as usize - mem.as_ptr() as usize) + 5] << 16 +
                        mem[(x as usize - mem.as_ptr() as usize) + 6] << 8 + mem[(x as usize - mem.as_ptr() as usize) + 7]) as u64
                },

                // BPF_ST class
                ebpf::ST_B_IMM   => {
                    let x = (reg[_dst] as u64 + insn.off as u64) as *mut u8;
                    check_mem_store(mem, x as u64, 1, insn_ptr)?;
                    // println!("ST_B_IMM {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    mem[(x as usize - mem.as_ptr() as usize)] = insn.imm as u8;
                },
                ebpf::ST_H_IMM   => {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_dst] as u64 + insn.off as u64) as *mut u16;
                    check_mem_store(mem, x as u64, 2, insn_ptr)?;
                    // println!("ST_H_IMM {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    mem[(x as usize - mem.as_ptr() as usize)] = insn.imm as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 1] = (insn.imm >> 8) as u8;
                },
                ebpf::ST_W_IMM   => {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_dst] as u64 + insn.off as u64) as *mut u32;
                    check_mem_store(mem, x as u64, 4, insn_ptr)?;
                    // println!("ST_W_IMM {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    mem[(x as usize - mem.as_ptr() as usize)] = insn.imm as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 1] = (insn.imm >> 8) as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 2] = (insn.imm >> 16) as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 3] = (insn.imm >> 24) as u8;
                },
                ebpf::ST_DW_IMM  => {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_dst] as u64 + insn.off as u64) as *mut u64;
                    check_mem_store(mem, x as u64, 8, insn_ptr)?;
                    // println!("ST_DW_IMM {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    mem[(x as usize - mem.as_ptr() as usize)] = insn.imm as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 1] = (insn.imm >> 8) as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 2] = (insn.imm >> 16) as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 3] = (insn.imm >> 24) as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 4] = (insn.imm as u64 >> 32) as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 5] = (insn.imm as u64 >> 40) as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 6] = (insn.imm as u64 >> 48) as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 7] = (insn.imm as u64 >> 56) as u8;
                },

                // BPF_STX class
                ebpf::ST_B_REG   => {
                    let x = (reg[_dst] as u64 + insn.off as u64) as *mut u8;
                    check_mem_store(mem, x as u64, 1, insn_ptr)?;
                    // println!("ST_B_REG {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    mem[(x as usize - mem.as_ptr() as usize)] = reg[_src] as u8;
                },
                ebpf::ST_H_REG   => {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_dst] as u64 + insn.off as u64) as *mut u16;
                    check_mem_store(mem, x as u64, 2, insn_ptr)?;
                    // println!("ST_H_REG {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    mem[(x as usize - mem.as_ptr() as usize)] = reg[_src] as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 1] = (reg[_src] >> 8) as u8;
                },
                ebpf::ST_W_REG   => {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_dst] as u64 + insn.off as u64) as *mut u32;
                    check_mem_store(mem, x as u64, 4, insn_ptr)?;
                    // println!("ST_W_REG {:#x}", reg[_src] as u32);
                    mem[(x as usize - mem.as_ptr() as usize)] = reg[_src] as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 1] = (reg[_src] >> 8) as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 2] = (reg[_src] >> 16) as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 3] = (reg[_src] >> 24) as u8;
                },
                ebpf::ST_DW_REG  => {
                    #[allow(cast_ptr_alignment)]
                    let x = (reg[_dst] as u64 + insn.off as u64) as *mut u64;
                    check_mem_store(mem, x as u64, 8, insn_ptr)?;
                    // println!("ST_DW_REG {:#x}", mem[(x as usize - mem.as_ptr() as usize)]);
                    mem[(x as usize - mem.as_ptr() as usize)] = reg[_src] as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 1] = (reg[_src] >> 8) as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 2] = (reg[_src] >> 16) as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 3] = (reg[_src] >> 24) as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 4] = (reg[_src] >> 32) as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 5] = (reg[_src] >> 40) as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 6] = (reg[_src] >> 48) as u8;
                    mem[(x as usize - mem.as_ptr() as usize) + 7] = (reg[_src] >> 56) as u8;
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
                        Err(Error::new(ErrorKind::Other,"Error: division by 0"))?;
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
                        Err(Error::new(ErrorKind::Other,"Error: division by 0"))?;
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
                        Err(Error::new(ErrorKind::Other,"Error: division by 0"))?;
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
                        Err(Error::new(ErrorKind::Other,"Error: division by 0"))?;
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
                ebpf::JA         =>                                             insn_ptr = (insn_ptr as i16 + insn.off) as usize,
                ebpf::JEQ_IMM    => if  reg[_dst] == insn.imm as u64          { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JEQ_REG    => if  reg[_dst] == reg[_src]                { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JGT_IMM    => if  reg[_dst] >  insn.imm as u64          { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JGT_REG    => if  reg[_dst] >  reg[_src]                { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JGE_IMM    => if  reg[_dst] >= insn.imm as u64          { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JGE_REG    => if  reg[_dst] >= reg[_src]                { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JLT_IMM    => if  reg[_dst] <  insn.imm as u64          { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JLT_REG    => if  reg[_dst] <  reg[_src]                { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JLE_IMM    => if  reg[_dst] <= insn.imm as u64          { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JLE_REG    => if  reg[_dst] <= reg[_src]                { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JSET_IMM   => if  reg[_dst] &  insn.imm as u64 != 0     { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JSET_REG   => if  reg[_dst] &  reg[_src]       != 0     { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JNE_IMM    => if  reg[_dst] != insn.imm as u64          { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JNE_REG    => if  reg[_dst] != reg[_src]                { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JSGT_IMM   => if  reg[_dst] as i64 >  insn.imm  as i64  { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JSGT_REG   => if  reg[_dst] as i64 >  reg[_src] as i64  { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JSGE_IMM   => if  reg[_dst] as i64 >= insn.imm  as i64  { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JSGE_REG   => if  reg[_dst] as i64 >= reg[_src] as i64  { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JSLT_IMM   => if (reg[_dst] as i64) <  insn.imm  as i64 { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JSLT_REG   => if (reg[_dst] as i64) <  reg[_src] as i64 { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JSLE_IMM   => if (reg[_dst] as i64) <= insn.imm  as i64 { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                ebpf::JSLE_REG   => if (reg[_dst] as i64) <= reg[_src] as i64 { insn_ptr = (insn_ptr as i16 + insn.off) as usize; },
                // Do not delegate the check to the verifier, since registered functions can be
                // changed after the program has been verified.
                ebpf::CALL       => if let Some(function) = self.helpers.get(&(insn.imm as u32)) {
                    reg[0] = function(reg[1], reg[2], reg[3], reg[4], reg[5]);
                } else {
                    Err(Error::new(ErrorKind::Other, format!("Error: unknown helper function (id: {:#x})", insn.imm as u32)))?;
                },
                ebpf::TAIL_CALL  => unimplemented!(),
                ebpf::EXIT       => return Ok(reg[0]),

                _                => unreachable!()
            }
        }

        unreachable!()
    }

    fn check_mem(addr: u64, len: usize, access_type: &str, insn_ptr: usize,
                 mbuff: &[u8], mem: &[u8], stack: &[u8]) -> Result<(), ReturnCode> {
        if mbuff.as_ptr() as u64 <= addr && addr + len as u64 <= mbuff.as_ptr() as u64 + mbuff.len() as u64 {
            return Ok(())
        }
        if mem.as_ptr() as u64 <= addr && addr + len as u64 <= mem.as_ptr() as u64 + mem.len() as u64 {
            return Ok(())
        }
        if stack.as_ptr() as u64 <= addr && addr + len as u64 <= stack.as_ptr() as u64 + stack.len() as u64 {
            return Ok(())
        }

        panic!(
            "Error: out of bounds memory {} (insn #{:?}), addr {:#x}, size {:?}\nmbuff: {:#x}/{:#x}, mem: {:#x}/{:#x}, stack: {:#x}/{:#x}",
            access_type, insn_ptr, addr, len,
            mbuff.as_ptr() as u64, mbuff.len(),
            mem.as_ptr() as u64, mem.len(),
            stack.as_ptr() as u64, stack.len()
        );
    }
}

/// A virtual machine to run eBPF program. This kind of VM is used for programs expecting to work
/// on a metadata buffer containing pointers to packet data, but it internally handles the buffer
/// so as to save the effort to manually handle the metadata buffer for the user.
///
/// This struct implements a static internal buffer that is passed to the program. The user has to
/// indicate the offset values at which the eBPF program expects to find the start and the end of
/// packet data in the buffer. On calling the `execute_program()` or `execute_program_jit()` functions, the
/// struct automatically updates the addresses in this static buffer, at the appointed offsets, for
/// the start and the end of the packet data the program is called upon.
///
/// # Examples
///
/// This was compiled with clang from the following program, in C:
///
/// ```c
/// #include <linux/bpf.h>
/// #include "path/to/linux/samples/bpf/bpf_helpers.h"
///
/// SEC(".classifier")
/// int classifier(struct __sk_buff *skb)
/// {
///   void *data = (void *)(long)skb->data;
///   void *data_end = (void *)(long)skb->data_end;
///
///   // Check program is long enough.
///   if (data + 5 > data_end)
///     return 0;
///
///   return *((char *)data + 5);
/// }
/// ```
///
/// Some small modifications have been brought to have it work, see comments.
///
/// ```
/// let prog = &[
///     0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r0, 0
///     // Here opcode 0x61 had to be replace by 0x79 so as to load a 8-bytes long address.
///     // Also, offset 0x4c had to be replace with e.g. 0x40 so as to prevent the two pointers
///     // from overlapping in the buffer.
///     0x79, 0x12, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, // load pointer to mem from r1[0x40] to r2
///     0x07, 0x02, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, // add r2, 5
///     // Here opcode 0x61 had to be replace by 0x79 so as to load a 8-bytes long address.
///     0x79, 0x11, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, // load ptr to mem_end from r1[0x50] to r1
///     0x2d, 0x12, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, // if r2 > r1 skip 3 instructions
///     0x71, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // load r2 (= *(mem + 5)) into r0
///     0x67, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, // r0 >>= 56
///     0xc7, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, // r0 <<= 56 (arsh) extend byte sign to u64
///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
/// ];
/// let mem1 = &mut [
///     0xaa, 0xbb, 0x11, 0x22, 0xcc, 0xdd
/// ];
/// let mem2 = &mut [
///     0xaa, 0xbb, 0x11, 0x22, 0xcc, 0x27
/// ];
///
/// // Instantiate a VM. Note that we provide the start and end offsets for mem pointers.
/// let mut vm = rbpf::EbpfVmFixedMbuff::new(Some(prog), 0x40, 0x50).unwrap();
///
/// // Provide only a reference to the packet data. We do not manage the metadata buffer.
/// let res = vm.execute_program(mem1).unwrap();
/// assert_eq!(res, 0xffffffffffffffdd);
///
/// let res = vm.execute_program(mem2).unwrap();
/// assert_eq!(res, 0x27);
/// ```
pub struct EbpfVmFixedMbuff<'a> {
    parent: EbpfVmMbuff<'a>,
    mbuff:  MetaBuff,
}

impl<'a> EbpfVmFixedMbuff<'a> {

    /// Create a new virtual machine instance, and load an eBPF program into that instance.
    /// When attempting to load the program, it passes through a simple verifier.
    ///
    /// # Examples
    ///
    /// ```
    /// let prog = &[
    ///     0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r0, 0
    ///     0x79, 0x12, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, // load mem from r1[0x40] to r2
    ///     0x07, 0x02, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, // add r2, 5
    ///     0x79, 0x11, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, // load mem_end from r1[0x50] to r1
    ///     0x2d, 0x12, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, // if r2 > r1 skip 3 instructions
    ///     0x71, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // load r2 (= *(mem + 5)) into r0
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    ///
    /// // Instantiate a VM. Note that we provide the start and end offsets for mem pointers.
    /// let mut vm = rbpf::EbpfVmFixedMbuff::new(Some(prog), 0x40, 0x50).unwrap();
    /// ```
    pub fn new(prog: Option<&'a [u8]>, data_offset: usize, data_end_offset: usize) -> Result<EbpfVmFixedMbuff<'a>, ReturnCode> {
        let parent = EbpfVmMbuff::new(prog)?;
        let get_buff_len = | x: usize, y: usize | if x >= y { x + 8 } else { y + 8 };
        let buffer = vec![0u8; get_buff_len(data_offset, data_end_offset)];
        let mbuff = MetaBuff {
            data_offset:     data_offset,
            data_end_offset: data_end_offset,
            buffer:          buffer,
        };
        Ok(EbpfVmFixedMbuff {
            parent: parent,
            mbuff:  mbuff,
        })
    }

    /// Load a new eBPF program into the virtual machine instance.
    ///
    /// At the same time, load new offsets for storing pointers to start and end of packet data in
    /// the internal metadata buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// let prog1 = &[
    ///     0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r0, 0
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    /// let prog2 = &[
    ///     0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r0, 0
    ///     0x79, 0x12, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, // load mem from r1[0x40] to r2
    ///     0x07, 0x02, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, // add r2, 5
    ///     0x79, 0x11, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, // load mem_end from r1[0x50] to r1
    ///     0x2d, 0x12, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, // if r2 > r1 skip 3 instructions
    ///     0x71, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // load r2 (= *(mem + 5)) into r0
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    ///
    /// let mem = &mut [
    ///     0xaa, 0xbb, 0x11, 0x22, 0xcc, 0x27,
    /// ];
    ///
    /// let mut vm = rbpf::EbpfVmFixedMbuff::new(Some(prog1), 0, 0).unwrap();
    /// vm.set_program(prog2, 0x40, 0x50);
    ///
    /// let res = vm.execute_program(mem).unwrap();
    /// assert_eq!(res, 0x27);
    /// ```
    pub fn set_program(&mut self, prog: &'a [u8], data_offset: usize, data_end_offset: usize) -> Result<(), ReturnCode> {
        let get_buff_len = | x: usize, y: usize | if x >= y { x + 8 } else { y + 8 };
        let buffer = vec![0u8; get_buff_len(data_offset, data_end_offset)];
        self.mbuff.buffer = buffer;
        self.mbuff.data_offset = data_offset;
        self.mbuff.data_end_offset = data_end_offset;
        self.parent.set_program(prog)?;
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
    pub fn set_verifier(&mut self, verifier: Verifier) -> Result<(), ReturnCode> {
        self.parent.set_verifier(verifier)
    }

    /// Register a built-in or user-defined helper function in order to use it later from within
    /// the eBPF program. The helper is registered into a hashmap, so the `key` can be any `u32`.
    ///
    /// If using JIT-compiled eBPF programs, be sure to register all helpers before compiling the
    /// program. You should be able to change registered helpers after compiling, but not to add
    /// new ones (i.e. with new keys).
    ///
    /// # Examples
    ///
    /// ```
    /// use rbpf::helpers;
    ///
    /// // This program was compiled with clang, from a C program containing the following single
    /// // instruction: `return bpf_trace_printk("foo %c %c %c\n", 10, 1, 2, 3);`
    /// let prog = &[
    ///     0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r0, 0
    ///     0x79, 0x12, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, // load mem from r1[0x40] to r2
    ///     0x07, 0x02, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, // add r2, 5
    ///     0x79, 0x11, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, // load mem_end from r1[0x50] to r1
    ///     0x2d, 0x12, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, // if r2 > r1 skip 6 instructions
    ///     0x71, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // load r2 (= *(mem + 5)) into r1
    ///     0xb7, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r2, 0
    ///     0xb7, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r3, 0
    ///     0xb7, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r4, 0
    ///     0xb7, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r5, 0
    ///     0x85, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // call helper with key 1
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    ///
    /// let mem = &mut [
    ///     0xaa, 0xbb, 0x11, 0x22, 0xcc, 0x09,
    /// ];
    ///
    /// // Instantiate a VM.
    /// let mut vm = rbpf::EbpfVmFixedMbuff::new(Some(prog), 0x40, 0x50).unwrap();
    ///
    /// // Register a helper. This helper will store the result of the square root of r1 into r0.
    /// vm.register_helper(1, helpers::sqrti);
    ///
    /// let res = vm.execute_program(mem).unwrap();
    /// assert_eq!(res, 3);
    /// ```
    // pub fn register_helper(&mut self, key: u32, function: fn (u64, u64, u64, u64, u64) -> u64) -> Result<(), ReturnCode> {
    //     self.parent.register_helper(key, function)
    // }

    /// Execute the program loaded, with the given packet data.
    ///
    /// If the program is made to be compatible with Linux kernel, it is expected to load the
    /// address of the beginning and of the end of the memory area used for packet data from some
    /// metadata buffer, which in the case of this VM is handled internally. The offsets at which
    /// the addresses should be placed should have be set at the creation of the VM.
    ///
    /// # Examples
    ///
    /// ```
    /// let prog = &[
    ///     0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r0, 0
    ///     0x79, 0x12, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, // load mem from r1[0x40] to r2
    ///     0x07, 0x02, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, // add r2, 5
    ///     0x79, 0x11, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, // load mem_end from r1[0x50] to r1
    ///     0x2d, 0x12, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, // if r2 > r1 skip 3 instructions
    ///     0x71, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // load r2 (= *(mem + 5)) into r0
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    /// let mem = &mut [
    ///     0xaa, 0xbb, 0x11, 0x22, 0xcc, 0xdd
    /// ];
    ///
    /// // Instantiate a VM. Note that we provide the start and end offsets for mem pointers.
    /// let mut vm = rbpf::EbpfVmFixedMbuff::new(Some(prog), 0x40, 0x50).unwrap();
    ///
    /// // Provide only a reference to the packet data. We do not manage the metadata buffer.
    /// let res = vm.execute_program(mem).unwrap();
    /// assert_eq!(res, 0xdd);
    /// ```
    pub fn execute_program(&mut self, mem: &'a mut [u8]) -> Result<u64, ReturnCode> {
        let l = self.mbuff.buffer.len();
        // Can this ever happen? Probably not, should be ensured at mbuff creation.
        if self.mbuff.data_offset + 8 > l || self.mbuff.data_end_offset + 8 > l {
            // "Error: buffer too small ({:?}), cannot use data_offset {:?} and data_end_offset {:?}" l, self.mbuff.data_offset, self.mbuff.data_end_offset
            Err(ReturnCode::ESIZE)?;
            // Err(Error::new(ErrorKind::Other, format!("Error: buffer too small ({:?}), cannot use data_offset {:?} and data_end_offset {:?}",
            // l, self.mbuff.data_offset, self.mbuff.data_end_offset)))?;
        }
        LittleEndian::write_u64(&mut self.mbuff.buffer[(self.mbuff.data_offset) .. ], mem.as_ptr() as u64);
        LittleEndian::write_u64(&mut self.mbuff.buffer[(self.mbuff.data_end_offset) .. ], mem.as_ptr() as u64 + mem.len() as u64);
        self.parent.execute_program(mem, &self.mbuff.buffer)
    }
}

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
    parent: EbpfVmMbuff<'a>,
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
    pub fn new(prog: Option<&'a [u8]>) -> Result<EbpfVmRaw<'a>, ReturnCode> {
        let parent = EbpfVmMbuff::new(prog)?;
         Ok(EbpfVmRaw {
            parent: parent,
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
    pub fn set_program(&mut self, prog: &'a [u8]) -> Result<(), ReturnCode> {
        self.parent.set_program(prog)?;
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
    pub fn set_verifier(&mut self, verifier: Verifier) -> Result<(), ReturnCode> {
        self.parent.set_verifier(verifier)
    }

    /// Register a built-in or user-defined helper function in order to use it later from within
    /// the eBPF program. The helper is registered into a hashmap, so the `key` can be any `u32`.
    ///
    /// If using JIT-compiled eBPF programs, be sure to register all helpers before compiling the
    /// program. You should be able to change registered helpers after compiling, but not to add
    /// new ones (i.e. with new keys).
    ///
    /// # Examples
    ///
    /// ```
    /// use rbpf::helpers;
    ///
    /// let prog = &[
    ///     0x79, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // ldxdw r1, r1[0x00]
    ///     0xb7, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r2, 0
    ///     0xb7, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r3, 0
    ///     0xb7, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r4, 0
    ///     0xb7, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r5, 0
    ///     0x85, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // call helper with key 1
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    ///
    /// let mem = &mut [
    ///     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01
    /// ];
    ///
    /// // Instantiate a VM.
    /// let mut vm = rbpf::EbpfVmRaw::new(Some(prog)).unwrap();
    ///
    /// // Register a helper. This helper will store the result of the square root of r1 into r0.
    /// vm.register_helper(1, helpers::sqrti);
    ///
    /// let res = vm.execute_program(mem).unwrap();
    /// assert_eq!(res, 0x10000000);
    /// ```
    // pub fn register_helper(&mut self, key: u32, function: fn (u64, u64, u64, u64, u64) -> u64) -> Result<(), ReturnCode> {
    //     self.parent.register_helper(key, function)
    // }

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
    pub fn execute_program(&self, mem: &'a mut [u8]) -> Result<u64, ReturnCode> {
        self.parent.execute_program(mem, &[])
    }
}

/// A virtual machine to run eBPF program. This kind of VM is used for programs that do not work
/// with any memory area—no metadata buffer, no packet data either.
///
/// # Examples
///
/// ```
/// let prog = &[
///     0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r0, 0
///     0xb7, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // mov r1, 1
///     0xb7, 0x02, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, // mov r2, 2
///     0xb7, 0x03, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, // mov r3, 3
///     0xb7, 0x04, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, // mov r4, 4
///     0xb7, 0x05, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, // mov r5, 5
///     0xb7, 0x06, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, // mov r6, 6
///     0xb7, 0x07, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, // mov r7, 7
///     0xb7, 0x08, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, // mov r8, 8
///     0x4f, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // or r0, r5
///     0x47, 0x00, 0x00, 0x00, 0xa0, 0x00, 0x00, 0x00, // or r0, 0xa0
///     0x57, 0x00, 0x00, 0x00, 0xa3, 0x00, 0x00, 0x00, // and r0, 0xa3
///     0xb7, 0x09, 0x00, 0x00, 0x91, 0x00, 0x00, 0x00, // mov r9, 0x91
///     0x5f, 0x90, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // and r0, r9
///     0x67, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, // lsh r0, 32
///     0x67, 0x00, 0x00, 0x00, 0x16, 0x00, 0x00, 0x00, // lsh r0, 22
///     0x6f, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // lsh r0, r8
///     0x77, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, // rsh r0, 32
///     0x77, 0x00, 0x00, 0x00, 0x13, 0x00, 0x00, 0x00, // rsh r0, 19
///     0x7f, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // rsh r0, r7
///     0xa7, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, // xor r0, 0x03
///     0xaf, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // xor r0, r2
///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
/// ];
///
/// // Instantiate a VM.
/// let vm = rbpf::EbpfVmNoData::new(Some(prog)).unwrap();
///
/// // Provide only a reference to the packet data.
/// let res = vm.execute_program().unwrap();
/// assert_eq!(res, 0x11);
/// ```
pub struct EbpfVmNoData<'a> {
    parent: EbpfVmRaw<'a>,
}

impl<'a> EbpfVmNoData<'a> {

    /// Create a new virtual machine instance, and load an eBPF program into that instance.
    /// When attempting to load the program, it passes through a simple verifier.
    ///
    /// # Examples
    ///
    /// ```
    /// let prog = &[
    ///     0xb7, 0x00, 0x00, 0x00, 0x11, 0x22, 0x00, 0x00, // mov r0, 0x2211
    ///     0xdc, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, // be16 r0
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    ///
    /// // Instantiate a VM.
    /// let vm = rbpf::EbpfVmNoData::new(Some(prog));
    /// ```
    pub fn new(prog: Option<&'a [u8]>) -> Result<EbpfVmNoData<'a>, ReturnCode> {
        let parent = EbpfVmRaw::new(prog)?;
        Ok(EbpfVmNoData {
            parent: parent,
        })
    }

    /// Load a new eBPF program into the virtual machine instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let prog1 = &[
    ///     0xb7, 0x00, 0x00, 0x00, 0x11, 0x22, 0x00, 0x00, // mov r0, 0x2211
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    /// let prog2 = &[
    ///     0xb7, 0x00, 0x00, 0x00, 0x11, 0x22, 0x00, 0x00, // mov r0, 0x2211
    ///     0xdc, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, // be16 r0
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    ///
    /// let mut vm = rbpf::EbpfVmNoData::new(Some(prog1)).unwrap();
    ///
    /// let res = vm.execute_program().unwrap();
    /// assert_eq!(res, 0x2211);
    ///
    /// vm.set_program(prog2);
    ///
    /// let res = vm.execute_program().unwrap();
    /// assert_eq!(res, 0x1122);
    /// ```
    pub fn set_program(&mut self, prog: &'a [u8]) -> Result<(), ReturnCode> {
        self.parent.set_program(prog)?;
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
    pub fn set_verifier(&mut self, verifier: Verifier) -> Result<(), ReturnCode> {
        self.parent.set_verifier(verifier)
    }

    /// Register a built-in or user-defined helper function in order to use it later from within
    /// the eBPF program. The helper is registered into a hashmap, so the `key` can be any `u32`.
    ///
    /// If using JIT-compiled eBPF programs, be sure to register all helpers before compiling the
    /// program. You should be able to change registered helpers after compiling, but not to add
    /// new ones (i.e. with new keys).
    ///
    /// # Examples
    ///
    /// ```
    /// use rbpf::helpers;
    ///
    /// let prog = &[
    ///     0xb7, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // mov r1, 0x010000000
    ///     0xb7, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r2, 0
    ///     0xb7, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r3, 0
    ///     0xb7, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r4, 0
    ///     0xb7, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r5, 0
    ///     0x85, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // call helper with key 1
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    ///
    /// let mut vm = rbpf::EbpfVmNoData::new(Some(prog)).unwrap();
    ///
    /// // Register a helper. This helper will store the result of the square root of r1 into r0.
    /// vm.register_helper(1, helpers::sqrti).unwrap();
    ///
    /// let res = vm.execute_program().unwrap();
    /// assert_eq!(res, 0x1000);
    /// ```
    // pub fn register_helper(&mut self, key: u32, function: fn (u64, u64, u64, u64, u64) -> u64) -> Result<(), ReturnCode> {
    //     self.parent.register_helper(key, function)
    // }

    /// Execute the program loaded, without providing pointers to any memory area whatsoever.
    ///
    /// # Examples
    ///
    /// ```
    /// let prog = &[
    ///     0xb7, 0x00, 0x00, 0x00, 0x11, 0x22, 0x00, 0x00, // mov r0, 0x2211
    ///     0xdc, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, // be16 r0
    ///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    /// ];
    ///
    /// let vm = rbpf::EbpfVmNoData::new(Some(prog)).unwrap();
    ///
    /// // For this kind of VM, the `execute_program()` function needs no argument.
    /// let res = vm.execute_program().unwrap();
    /// assert_eq!(res, 0x1122);
    /// ```
    pub fn execute_program(&self) -> Result<u64, ReturnCode> {
        self.parent.execute_program(&mut [])
    }
}
