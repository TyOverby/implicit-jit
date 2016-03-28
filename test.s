.text                    # text section
.globl _program 
.globl _print_int

_program:
 movq $1, %rax
 call _print_int
 ret
