# =============================================================================
# Makefile for the Tock-on-LPC55S69-EVK Board
# Based on the official Tock board Makefile pattern.
# =============================================================================

# This includes Tock's generic build variables and rules.
# It provides $(PLATFORM), $(TOCK_ROOT_DIRECTORY), and the rule to build
# the ELF file from the PLATFORM name.
include ../Makefile.common

# --- Board-Specific Configuration ---
CHIP = LPC55S69JBD100
PROBE_RS = probe-rs
PATCHER = $(TOCK_ROOT_DIRECTORY)tools/nxp-checksum-tool/nxp-checksum

# --- Tock Build System Customization ---
# Export the linker argument so Makefile.common's build rule can use it.
export TOCK_RUSTC_LINK_ARGS = RUSTFLAGS='-C link-arg=--entry=Reset'

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

# --- Main Targets ---
.PHONY: all install debug flash program patch-debug patch-release

# The default target. This tells `make` that our goal is to produce
# the debug ELF file. `Makefile.common` has the rule for how to do this.
all: $(DEBUG_ELF)

# --- Flash and Debug Rules ---

debug: all patch-debug
	@echo "✅ Build and patch complete."
	@echo "------------------------------------------------------------"
	@echo "ACTION REQUIRED: Please put the board into ISP mode now."
	@read -p "Press [Enter] to continue running..."
	$(PROBE_RS) run --chip $(CHIP) $(DEBUG_ELF)

flash: patch-release
	@echo "✅ Build and patch complete."
	@echo "------------------------------------------------------------"
	@echo "ACTION REQUIRED: Please put the board into ISP mode now."
	@read -p "Press [Enter] to continue flashing..."
	$(PROBE_RS) flash --chip $(CHIP) --erase-all $(RELEASE_ELF)

# --- Patching Logic ---

# Rule to ensure the patcher tool is built.
$(PATCHER):
	@echo "INFO: NXP checksum tool not found. Building..."
	(cd $(dir $(PATCHER)) && cargo build --release)

# Rule to patch the debug binary.
patch-debug: all $(PATCHER)
	@echo "INFO: Patching DEBUG binary..."
	$(PATCHER) $(DEBUG_ELF)

# Rule to patch the release binary.
patch-release: $(RELEASE_ELF) $(PATCHER)
	@echo "INFO: Patching RELEASE binary..."
	$(PATCHER) $(RELEASE_ELF)


# --- Aliases ---
install: flash
program: debug
>>>>>>> 2cc808484 (Add initial code for the GPIO)
