# Licensed under the Apache License, Version 2.0 or the MIT License.
# SPDX-License-Identifier: Apache-2.0 OR MIT
# Copyright Tock Contributors 2022.

# Makefile for building the tock kernel for the NUCLEO-F429ZI platform
#

include ../Makefile.common

PROBE_RS=probe-rs

# The specific chip on the NUCLEO-F429ZI board.
# probe-rs needs this to know what it's flashing.
iCHIP=LPC55S69JBD100

<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
APP=/home/pc1/Projects/libtock-c/examples/blink/build/cortex-m7/cortex-m7.tbf
KERNEL=$(TOCK_ROOT_DIRECTORY)/target/$(TARGET)/release/$(PLATFORM).elf
KERNEL_WITH_APP=$(TOCK_ROOT_DIRECTORY)/target/$(TARGET)/release/$(PLATFORM)-app.elf

=======
>>>>>>> 9d726ec54 (Adding support for NXP-LPC55)
# Default target for installing the kernel.
.PHONY: install
install: flash

# flash-debug is no longer needed as a separate target, but kept for
# compatibility. `make debug` is the new recommended command.
.PHONY: flash-debug
flash-debug: debug

# Flash the release version of the kernel to the board.
.PHONY: flash
flash: $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/release/$(PLATFORM).elf
	$(PROBE_RS) flash --chip $(CHIP) --erase-all $<

# Flash the debug version and attach an RTT console to view kernel logs.
# This is the recommended command for debugging.
.PHONY: debug
debug: $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/debug/$(PLATFORM).elf
	$(PROBE_RS) run --chip $(iCHIP) $<

# An alias for the default `install` target.
.PHONY: program
<<<<<<< HEAD
program: release
	arm-none-eabi-objcopy --set-section-flags .apps=LOAD,ALLOC $(KERNEL) $(KERNEL_WITH_APP)
	arm-none-eabi-objcopy --update-section .apps=$(APP) $(KERNEL_WITH_APP)
	$(PROBE_RS) run --chip $(iCHIP) $(KERNEL_WITH_APP)
=======
program: install
>>>>>>> 9d726ec54 (Adding support for NXP-LPC55)
=======
# --- Paths to Artifacts ---
DEBUG_ELF = $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/debug/$(PLATFORM)
RELEASE_ELF = $(TOCK_ROOT_DIRECTORY)/target/$(TARGET)/release/$(PLATFORM)
=======
APP=/home/pc1/Projects/libtock-c/examples/buttons/build/cortex-m7/cortex-m7.tbf
KERNEL=$(TOCK_ROOT_DIRECTORY)/target/$(TARGET)/release/$(PLATFORM).elf
KERNEL_WITH_APP=$(TOCK_ROOT_DIRECTORY)/target/$(TARGET)/release/$(PLATFORM)-app.elf
>>>>>>> 6eecc9169 (Add the posibility to upload a process to the board)

# Default target for installing the kernel.
.PHONY: install
install: flash
<<<<<<< HEAD
program: debug
>>>>>>> 2cc808484 (Add initial code for the GPIO)
=======

# flash-debug is no longer needed as a separate target, but kept for
# compatibility. `make debug` is the new recommended command.
.PHONY: flash-debug
flash-debug: debug

# Flash the release version of the kernel to the board.
.PHONY: flash
flash: $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/release/$(PLATFORM).elf
	$(PROBE_RS) flash --chip $(CHIP) --erase-all $<

# Flash the debug version and attach an RTT console to view kernel logs.
# This is the recommended command for debugging.
.PHONY: debug
debug: $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/debug/$(PLATFORM).elf
	$(PROBE_RS) run --chip $(iCHIP) $<

# An alias for the default `install` target.
.PHONY: program
program: release
	arm-none-eabi-objcopy --set-section-flags .apps=LOAD,ALLOC $(KERNEL) $(KERNEL_WITH_APP)
	arm-none-eabi-objcopy --update-section .apps=$(APP) $(KERNEL_WITH_APP)
	$(PROBE_RS) run --chip $(iCHIP) $(KERNEL_WITH_APP)
>>>>>>> 6eecc9169 (Add the posibility to upload a process to the board)
