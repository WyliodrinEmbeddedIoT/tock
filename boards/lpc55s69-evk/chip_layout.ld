/* Licensed under the Apache License, Version 2.0 or the MIT License. */
/* SPDX-License-Identifier: Apache-2.0 OR MIT                         */
/* Copyright Tock Contributors 2025. */

MEMORY
{
  rom(rx) : ORIGIN = 0x00000000, LENGTH = 630K
  ram(rwx) : ORIGIN = 0x20000000, LENGTH = 64K
  prog(rx) : ORIGIN = 0x00080000, LENGTH = 128K
}

PAGE_SIZE = 4K;

SECTIONS {
  /* The .text section is where all code and read-only data goes */
  .text : {
    KEEP(*(.vector_table .vector_table.*))
  } > rom

  /* ... other sections for .bss, .data, .stack ... */
}
