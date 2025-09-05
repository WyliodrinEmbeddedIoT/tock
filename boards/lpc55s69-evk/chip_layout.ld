<<<<<<< HEAD
=======
<<<<<<< HEAD
<<<<<<< HEAD
>>>>>>> 2fdc59ab742fd1fac587e9ae8266e3bb55226622
MEMORY
{
  rom(rx) : ORIGIN = 0x00000000, LENGTH = 64K
  ram(rwx) : ORIGIN = 0x20000000, LENGTH = 64K
  prog(rx) : ORIGIN = 0x00010000, LENGTH = 566K
<<<<<<< HEAD
=======
=======
/* Licensed under the Apache License, Version 2.0 or the MIT License. */
/* SPDX-License-Identifier: Apache-2.0 OR MIT                         */
/* Copyright Tock Contributors 2025. */

=======
>>>>>>> 2cc808484 (Add initial code for the GPIO)
MEMORY
{
  rom(rx) : ORIGIN = 0x00000000, LENGTH = 64K
  ram(rwx) : ORIGIN = 0x20000000, LENGTH = 64K
<<<<<<< HEAD
  prog(rx) : ORIGIN = 0x20010000, LENGTH = 64K
>>>>>>> 9d726ec54 (Adding support for NXP-LPC55)
=======
  prog(rx) : ORIGIN = 0x00010000, LENGTH = 566K
>>>>>>> 6eecc9169 (Add the posibility to upload a process to the board)
>>>>>>> 2fdc59ab742fd1fac587e9ae8266e3bb55226622
}

PAGE_SIZE = 4K;

<<<<<<< HEAD
ENTRY(initialize_ram_jump_to_main)
=======
<<<<<<< HEAD
<<<<<<< HEAD
ENTRY(initialize_ram_jump_to_main)
=======
>>>>>>> 9d726ec54 (Adding support for NXP-LPC55)
=======
ENTRY(initialize_ram_jump_to_main)
>>>>>>> 2cc808484 (Add initial code for the GPIO)
>>>>>>> 2fdc59ab742fd1fac587e9ae8266e3bb55226622
