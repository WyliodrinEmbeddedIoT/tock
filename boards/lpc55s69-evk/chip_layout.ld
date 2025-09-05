MEMORY
{
  rom(rx) : ORIGIN = 0x00000000, LENGTH = 64K
  ram(rwx) : ORIGIN = 0x20000000, LENGTH = 64K
  prog(rx) : ORIGIN = 0x00010000, LENGTH = 566K
}

PAGE_SIZE = 4K;

ENTRY(initialize_ram_jump_to_main)
