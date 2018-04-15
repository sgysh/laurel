/****************************************************************************
 *   Copyright (c) 2018 Yoshinori Sugino
 *   This software is released under the MIT License.
 ****************************************************************************/
MEMORY
{
	flash (rx)  : ORIGIN = 0x00000000, LENGTH = 0x00010000
	sram  (rwx) : ORIGIN = 0x20000000, LENGTH = 0x00002000
}

OUTPUT_ARCH(arm)
ENTRY(__start)

PROVIDE(sp_main = ORIGIN(sram) + LENGTH(sram));

SECTIONS
{
	.text : {
		_stext = ABSOLUTE(.);
                LONG(sp_main)
		KEEP(*(.reset_vector))
		KEEP(*(.exceptions))
		KEEP(*(.interrupts))
		*(.text .text.*)
		_etext = ABSOLUTE(.);
	} > flash

	_data_lma_start = ABSOLUTE(LOADADDR(.data));

	.data : {
		_sdata = ABSOLUTE(.);
		*(.data .data.*)
		_edata = ABSOLUTE(.);
	} > sram AT > flash

	.bss : {
		_sbss = ABSOLUTE(.);
		*(.bss .bss.*)
		_ebss = ABSOLUTE(.);
	} > sram
}

