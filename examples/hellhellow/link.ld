ENTRY(_start);

SECTIONS
{
  .text 0x00201000 : { *(.text*) }
  .rodata : { *(.rodata*) }
  .data : { *(.data*) }
  .bss : { *(.bss*) }
}