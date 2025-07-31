<<<<<<< HEAD
<<<<<<< HEAD
MEMORY
{
  rom(rx) : ORIGIN = 0x00000000, LENGTH = 64K
  ram(rwx) : ORIGIN = 0x20000000, LENGTH = 64K
  prog(rx) : ORIGIN = 0x00010000, LENGTH = 566K
=======
/* Licensed under the Apache License, Version 2.0 or the MIT License. */
/* SPDX-License-Identifier: Apache-2.0 OR MIT                         */
/* Copyright Tock Contributors 2025. */

=======
>>>>>>> 2cc808484 (Add initial code for the GPIO)
MEMORY
{
  rom(rx) : ORIGIN = 0x00000000, LENGTH = 630K
  ram(rwx) : ORIGIN = 0x20000000, LENGTH = 64K
  prog(rx) : ORIGIN = 0x20010000, LENGTH = 64K
>>>>>>> 9d726ec54 (Adding support for NXP-LPC55)
}

PAGE_SIZE = 4K;

<<<<<<< HEAD
<<<<<<< HEAD
ENTRY(initialize_ram_jump_to_main)
=======
>>>>>>> 9d726ec54 (Adding support for NXP-LPC55)
=======
ENTRY(initialize_ram_jump_to_main)
>>>>>>> 2cc808484 (Add initial code for the GPIO)
