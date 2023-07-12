# Licensed under the Apache License, Version 2.0 or the MIT License.
# SPDX-License-Identifier: Apache-2.0 OR MIT
# Copyright Tock Contributors 2022.

# Makefile for building the tock kernel for the LPCXpresso55S69 platform
#
TARGET=thumbv7em-none-eabi
PLATFORM=lpcxpresso55s69

include ../Makefile.common

OPENOCD=openocd
OPENOCD_OPTIONS=-f openocd.cfg

APP= ../../../libtock-c/examples/rtc_app/build/cortex-m4/cortex-m4.tbf
KERNEL=$(TOCK_ROOT_DIRECTORY)/target/$(TARGET)/debug/$(PLATFORM).elf
KERNEL_WITH_APP=$(TOCK_ROOT_DIRECTORY)/target/$(TARGET)/debug/$(PLATFORM)-app.elf

# Default target for installing the kernel.
.PHONY: install
install: flash

.PHONY: flash-debug
flash-debug: $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/debug/$(PLATFORM).elf
	$(error Find out what tool the board uses for flashing instead of OpenOCD)

.PHONY: flash
flash: $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/release/$(PLATFORM).elf
	crt_emu_cm_redlink \
		--flash-load-exec "$(TOCK_ROOT_DIRECTORY)target/$(TARGET)/release/$(PLATFORM).elf" \
		-p MIMXRT1052xxxxB --ConnectScript RT1050_connect.scp \
		--flash-driver= -x . --flash-dir $(MCUX_IDE_BIN)/Flash --flash-hashing


.PHONY: program
program: $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/debug/$(PLATFORM).elf
	$(error See README.md and update this section accordingly)
