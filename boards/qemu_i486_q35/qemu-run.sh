#!/usr/bin/env bash
#
# qemu-run.sh – wrapper around qemu-system-i386
#
#  If a VGA-mode feature was enabled at build time, open a graphics
#  window and keep COM1 on stdio.
#  Otherwise add “-nographic” so the whole UI runs in the terminal.
#  Never pass “stdio” twice (QEMU crashes if we do).

set -euo pipefail

ELF="$1"               # kernel ELF from the Makefile
FEATURES="${2:-}"      # optional: feature list the Makefile knows

# Decide whether VGA output was requested for this build

vga_enabled=false

# Look at the Cargo feature string passed in by the Makefile
if [[ "$FEATURES" == *vga_text_80x25* ]] \
  || [[ "$FEATURES" == *vga_640x480_16* ]] \
  || [[ "$FEATURES" == *vga_800x600_16* ]]; then
    vga_enabled=true
fi

# Fallback: try to spot a VGA symbol when FEATURES was not supplied
if ! $vga_enabled; then
    if readelf -s "$ELF" 2>/dev/null | grep -q 'VGA_MODE'; then
        vga_enabled=true          # symbol survived LTO/strip
    fi
fi

# Assemble QEMU command-line fragments

if $vga_enabled; then
    DISPLAY_ARGS=()               # open the GUI window
    SERIAL_ARGS=(-serial stdio)   # expose COM1 on our tty
else
    DISPLAY_ARGS=(-nographic)     # everything via stdio
    SERIAL_ARGS=()                # stdio already claimed
fi


# Launch QEMU
exec qemu-system-i386 \
    -cpu 486 -machine q35 \
    -net none \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    "${SERIAL_ARGS[@]}" \
    "${DISPLAY_ARGS[@]}" \
    -kernel "$ELF"
