
            global      main
            extern      printf

            section     .text
main:
            mov         rdi, message
            xor         rax, rax
            call        printf
            ret

print_func:


            section     .data
message:    db          'Hello, world!', 10, 0

            section     .bss
buffer:     resb        64 ; reserve 64 bytes
wordvar:    resw        1 ; reserve a word
realarray:  resq        10 ; array of ten reals
ymmval:     resy        1 ; one YMM register
zmmvals:    resz        32 ; 32 ZMM registers
