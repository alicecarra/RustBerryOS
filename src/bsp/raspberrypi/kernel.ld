__rpi_phys_binary_load_addr = 0x80000;


ENTRY(__rpi_phys_binary_load_addr)

/* Flags:
 *     4 == R
 *     5 == RX
 *     6 == RW
 */
PHDRS
{
    segment_boot_core_stack PT_LOAD FLAGS(6);
    segment_code            PT_LOAD FLAGS(5);
    segment_data            PT_LOAD FLAGS(6);
}

SECTIONS
{
    /* Set the link address to 32 MiB */
    . = 0x2000000;

    .boot_core_stack (NOLOAD) :
    {
                                             /*   ^             */
                                             /*   | stack       */
        . += __rpi_phys_binary_load_addr;    /*   | growth      */
                                             /*   | direction   */
        __boot_core_stack_end_exclusive = .; /*   |             */
    } :segment_boot_core_stack

    __binary_nonzero_start = .;
    .text :
    {
        KEEP(*(.text._start))
        *(.text._start_arguments) /* Constants (or statics in Rust speak) read by _start(). */
        *(.text._start_rust)      /* The Rust entry point */
        *(.text*)                 /* Everything else */
    } :segment_code

    .rodata : ALIGN(8) { *(.rodata*) } :segment_code

    .data : { *(.data*) } :segment_data

    /* Fill up to 8 byte, b/c relocating the binary is done in u64 chunks */
    . = ALIGN(8);
    __binary_nonzero_end_exclusive = .;

    /* Section is zeroed in pairs of u64. Align start and end to 16 bytes */
    .bss (NOLOAD) : ALIGN(16)
    {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(16);
        __bss_end_exclusive = .;
    } :segment_data


    /* misc */
    .got : { *(.got*) }
    ASSERT(SIZEOF(.got) == 0, "Relocation support not expected")

    /DISCARD/ : { *(.comment*) }
}
