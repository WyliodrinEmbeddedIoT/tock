MEMORY
{
  rom (rx)  : ORIGIN = 0x40380000, LENGTH = 0x50000
  ram (rwx) : ORIGIN = 0x3FCB0000, LENGTH = 0x30000
  prog (rx) : ORIGIN = 0x403D0000, LENGTH = 0x10000
}

MPU_MIN_ALIGN = 1K;

INCLUDE ./esp32c3.rom.ld
INCLUDE ../kernel_layout.ld
