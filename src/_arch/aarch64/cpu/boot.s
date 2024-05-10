.macro ADR_REL register, symbol
		adrp \register, \symbol
		add \register, \register, #:lo12:\symbol
.endm

.section .text._start

// Vector table here for ARM from exceptions.s

_start:
  // Change DTB pointer position from x1 to x10 for future usage by kernel
  ldr x1, =adr_dtb
  str x0, [x1]
  ldr x10, adr_dtb

.L_boot:
	mrs x0, MPIDR_EL1
	and x0, x0, 0b11
	ldr x1, BOOT_CORE_ID
	cmp x0, x1
	b.ne .L_loop
  
	ADR_REL x0, __bss_start
	ADR_REL x1, __bss_end_executive


.L_bss_init_loop:
	cmp x0, x1
	b.eq .L_prepare_rust
	stp xzr, xzr, [x0], 0x10
	b .L_bss_init_loop

.L_prepare_rust:
  ADR_REL x0, __boot_core_stack_end_executive
	mov sp, x0
	b _start_rust

.L_loop:
	wfe
	b .L_loop



.size _start, . - _start
.type _start, function
.global _start

adr_dtb: .quad
