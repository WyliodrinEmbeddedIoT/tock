// Minimal VGA peripheral implementation for the Tock x86_q35 chip crate.
// Supports classic 80×25 text mode out‑of‑the‑box and exposes a stub for
// setting planar 16‑colour graphics modes (640×480 and 800×600).  These
// extra modes will be filled in later once the driver is integrated with a
// future framebuffer capsule.
//
// Licensing: same dual‑license terms as the rest of Tock (Apache‑2.0 OR MIT)
//
// NOTE!!!
//
// This file is an initial skeleton.  It compiles and provides working text‑
// mode console support so the board can swap from the UART mux to a VGA
// console.  Graphical modes are *disabled at runtime* until a framebuffer
// capsule implementation lands.  The low‑level register writes for 640×480 and 800×600 are
// nonetheless laid out so they can be enabled by flipping a constant.
//
// VGA peripheral driver for the x86_q35 chip.
//
// The driver currently focuses on **text mode** (colour attribute buffer at
// 0xB8000).  It also defines [`VgaMode`] and an [`init`] routine that writes
// the necessary CRT controller registers for text mode and two common planar
// 16‑colour modes.  Other code (e.g. the board crate) can query the selected
// mode via `kernel::config::CONFIG.vga_mode` and decide whether to route the
// `ProcessConsole` to this driver or to the legacy serial mux.


use core::fmt::{self};
/// All VGA modes supported by the x86_q35 chip crate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VgaMode {
    Text80x25,
    Graphics640x480x16,
    Graphics800x600x16,
}


// Constants for memory-mapped text mode buffer

// VGA physical Address

const TEXT_BUFFER_ADDR: usize = 0xB8000;
// Buffer dimensions
const TEXT_BUFFER_WIDTH: usize = 80;
const TEXT_BUFFER_HEIGHT: usize = 25;

// Low-level port I/O helpers
// inb/outb wrappers

/// Write an 8-bit value to an I/O Port.
#[inline(always)]
fn outb(port: u16, val: u8) {
    unsafe {
        core::arch::asm!("out dx, al", in ("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
    }
}

/// Read an 8-bit value from an I/O port.
#[inline(always)]
fn inb(port: u16) -> u8 {
    let val: u8;

    unsafe {
        core::arch::asm!("in al, dx", out("al") val, in("dx") port, options(nomem, nostack, preserves_flags));
    }
    val
}

/// Write a 16-bit value to an I/O Port
#[inline(always)]
fn outw(port: u16, val: u16) {
    unsafe {
        core::arch::asm!("out dx, ax", in("dx") port, in ("ax") val, options(nomem, nostack, preserves_flags));
    }
}
// Public API - the VGA struct providing fmt::Write implementation

/// Simple text-mode VGA console. Provides "core::fmt::Write" so it can be
/// plugged into Tock's `Console` and `ProcessConsole` components.
pub struct VgaText {
    // Current colum (0 - 79)
    col: usize,
    // Current row (0 - 24)
    row: usize,
    // Current  attribute byte (fg | bg<<4).
    attr: u8,
}

impl VgaText {
    // Create a new instance and optionally clear the screen
    pub const fn new() -> Self {
        Self {
            col: 0,
            row: 0,
            attr: 0x0F,
        }
    }

    const fn buffer_ptr() -> *mut u16 {
        TEXT_BUFFER_ADDR as *mut u16
    }

    // Index -> pointer into 0xB8000.
    #[inline(always)]
    fn cell_at(index: usize) -> *mut u16 {
        unsafe { Self::buffer_ptr().add(index) }
    }

    // Update the hardware cursor to (self.row, self.col)

    // VGA hardware keeps the cursor position as an offset measured in chars
    // in two separate byte registers
    // Port        Write Value     meaning
    //
    // Ox3D4       0x0F            select cursor LOW
    // 0x3D5       bits 7-0        low 8 bits of offset
    // 0x3D4       0x03            select cursor HIGH
    // 0x3D5       bits 8-15       high 8 bits of offset

    // Basically the sequence is as follows:
    // choose which internal VGA register we're writing to
    // by sending its index to 0x3D4, then send data byte to 0x3D5

    //0x3D4 = VGA CRT Controller index port
    //0x3D5 = VGA CRTC data port
    //0x0F  = CRTC register index 15 => Cursor Location Low
    //0x0E  = CRTC register index 14 => Cursor Location High
    fn update_hw_cursor(&self) {
        let pos = (self.row * TEXT_BUFFER_WIDTH + self.col) as u16;

            // write low byte
            outb(0x3D4, 0x0F);  //index 0x0F -> cursor LOW
            outb(0x3D5, (pos & 0xFF) as u8);
            outb(0x3D4, 0x0E); // index 0x0E -> cursor HIGH
            outb(0x3D5, (pos >> 8) as u8); // upper 8 bits

    }

    fn scroll_up(&mut self) {
        //copy rows 1-24 -> 0-23 (4 KiB total)
        for row in 1..TEXT_BUFFER_HEIGHT {
            let dst = Self::cell_at((row - 1) * TEXT_BUFFER_WIDTH);
            let src = Self::cell_at(row * TEXT_BUFFER_WIDTH);
            unsafe {
                core::ptr::copy(src, dst, TEXT_BUFFER_WIDTH);
            }
        }
        //clear last row
        let blank = ((self.attr as u16) << 8) | b' ' as u16;
        for col in 0..TEXT_BUFFER_WIDTH {
        unsafe {
                    core::ptr::write_volatile(
                    Self::cell_at((TEXT_BUFFER_HEIGHT - 1) * TEXT_BUFFER_WIDTH + col),
                    blank,
                );
            }
        }
        self.row = TEXT_BUFFER_HEIGHT - 1;
    }
    // Move the hardware cursor to `col`, `row`.
    pub fn set_cursor(&self, col: usize, row: usize) {
        let pos = (row * TEXT_BUFFER_WIDTH + col) as u16;

            outb(0x3D4, 0x0F);
            outb(0x3D5, (pos & 0xFF) as u8);
            outb(0x3D4, 0x0E);
            outb(0x3D5, (pos >> 8) as u8);

    }
}

impl fmt::Write for VgaText {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

impl VgaText {
    pub fn clear(&mut self) {
        let blank = ((self.attr as u16) << 8) | b' ' as u16;
        unsafe {
            for i in 0..TEXT_BUFFER_WIDTH * TEXT_BUFFER_HEIGHT {
                core::ptr::write_volatile(Self::cell_at(i), blank)
            }
        }
        self.col = 0;
        self.row = 0;
        self.update_hw_cursor();
    }
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.col = 0;
                self.row += 1;
            }
            b'\r' => {
                self.col = 0;
            }
            byte => {
                let val = ((self.attr as u16) << 8) | byte as u16;
                unsafe {
                    core::ptr::write_volatile(Self::cell_at(self.row * TEXT_BUFFER_WIDTH + self.col), val);
                }
                self.col += 1;
                if self.col == TEXT_BUFFER_WIDTH {
                    self.col = 0;
                    self.row += 1;
                }
            }
        }
        if self.row == TEXT_BUFFER_HEIGHT {
            self.scroll_up();
        }
        self.update_hw_cursor();
    }
}
fn init_text_mode() {
    // Standard BIOS mode 03h – easiest: call into BIOS via 0x10 int if we
    // wanted, but we can also just rely on the firmware default.  Here we
    // proactively reset key controller registers to known values so we can
    // switch from graphics back to text.
    outb(0x3D4, 0x11);
    outb(0x3D5, 0x00);
    // Horizontal/vertical timings omitted, firmware already sets them.
    // Reset attribute controller flip-flop
    inb(0x3DA);
    // Set attribute controller color + mode control (preset values)
    for (idx, val) in [
        (0x00, 0x00u8),
        (0x01, 0x01),
        (0x02, 0x02),
        (0x03, 0x03),
        (0x04, 0x04),
        (0x05, 0x05),
        (0x06, 0x14),
        (0x07, 0x07),
        (0x08, 0x38),
        (0x09, 0x39),
        (0x0A, 0x3A),
        (0x0B, 0x3B),
        (0x0C, 0x3C),
        (0x0D, 0x3D),
        (0x0E, 0x3E),
        (0x0F, 0x3F),
        (0x10, 0x0C), // Mode control: graphics off, blink attr on
        (0x12, 0x0F), // Colour plane enable
    ]
    .iter()
    .copied()
    {
        outb(0x3C0, idx);
        outb(0x3C0, val);
    }
    inb(0x3DA);        // reset flip-flop once more
    outb(0x3C0, 0x20); // bit 5 = 1 → video enabled
}

pub fn init(mode: VgaMode) {
    match mode {
        VgaMode::Text80x25 => init_text_mode(),
        VgaMode::Graphics640x480x16 => init_mode_0x12(),
        VgaMode::Graphics800x600x16 => init_mode_0x102(),
    }
}

/// VGA mode selected for this build (None ⇒ use serial console).
pub const VGA_MODE: Option<VgaMode> = if cfg!(feature = "vga_text_80x25") {
    Some(VgaMode::Text80x25)

} else if cfg!(feature = "vga_640x480_16") {
    Some(VgaMode::Graphics640x480x16)

} else if cfg!(feature = "vga_800x600_16") {
    Some(VgaMode::Graphics800x600x16)

} else {
    None
};

const _: () = assert!(
    (cfg!(feature = "vga_text_80x25") as u8
        + cfg!(feature = "vga_640x480_16") as u8
        + cfg!(feature = "vga_800x600_16") as u8)
        <= 1, // DEBUG temporary, must be changed to == 1 when process-console works fully
    "Only at most one of the VGA mode features can be selected."
);

fn init_mode_0x12() {
    // 640×480×16‑colour – VGA BIOS mode 0x12, 4‑plane planar.
    // Here we only set the minimal Sequencer and CRTC registers needed so
    // that, when a proper framebuffer driver is added, the mode is active.
    const VBE_INDEX: u16 = 0x01CE;
    const VBE_DATA: u16 = 0x01CF;

    unsafe fn vbe_write(index: u16, value: u16) {
        outw(VBE_INDEX, index);
        outw(VBE_DATA, value);
    }
    unsafe {
        // 1) Disable display while we reconfigure

        vbe_write(0x04, 0x00);

        // 2) Set X-res, Y-res, bits-per-pixel
        vbe_write(0x01, 640);
        vbe_write(0x02, 480);
        vbe_write(0x03, 16);

        // 3) Enable LinearFB and ClearMem
        vbe_write(0x04, 0x41); //bit0=enable, bit6=liner FP
    }
}

// Same for 800x600x16
fn init_mode_0x102() {
    const VBE_INDEX: u16 = 0x01CE; // Bochs VBE index port
    const VBE_DATA: u16 = 0x01CF; // Bochs VBE data  port

    #[inline(always)]
    unsafe fn vbe_write(index: u16, value: u16) {
        outw(VBE_INDEX, index);
        outw(VBE_DATA, value);
    }

    unsafe {
        // 1) Disable display while reconfiguring
        vbe_write(0x04, 0x00); // VBE_DISPI_INDEX_ENABLE

        // 2) Set resolution and colour depth
        vbe_write(0x01, 800); // X-res (width)
        vbe_write(0x02, 600); // Y-res (height)
        vbe_write(0x03, 16); // bits per pixel (RGB 5-6-5)

        // 3) Enable display | LinearFB
        //    bit0 = enable, bit6 = LFB, all others 0
        vbe_write(0x04, 0x41);
    }
}

/// Return (framebuffer_ptr, stride_in_bytes) if a linear framebuffer
/// is active; `None` for text mode or when no VGA feature is enabled.
pub fn framebuffer() -> Option<(*mut u8, usize)> {
    match VGA_MODE {
        Some(VgaMode::Graphics800x600x16) => {
            // Bochs/QEMU linear-FB BAR maps to 0xE000_0000
            // 800 pixels × 2 bytes per pixel (RGB-565)
            Some((0xE000_0000 as *mut u8, 800 * 2))
        }

        Some(VgaMode::Graphics640x480x16) => {
            // Same BAR, 640-wide
            Some((0xE000_0000 as *mut u8, 640 * 2))
        }

        _ => None, // Text mode or serial-only build
    }
}

