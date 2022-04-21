ENTRY(_Reset)
SECTIONS
{
	. = 0x40000000;
	# not required
	# .startup . : { build/entry.o(.text) }
	.text : { *(.text) }
	.data : { *(.data) }
	.bss : { *(.bss COMMON) }
	. = ALIGN(8);
	. = . + 0x1000; /* 4kB of stack memory */
	stack_top = .;
}
