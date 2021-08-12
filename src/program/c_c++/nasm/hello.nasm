SECTION .data

msg:            db  'msg: hello, the world', 10, 0
msg_len_str:    db  'msg_len: %d, int_i: %d', 10, 0
msg_len:        equ $-msg_len_str
int_i:          dd  100

x:              dd  2
y:              dd  1
msg1:           db  'x is greater than y', 10, 0
msg2:           db  'y is greater than x', 10, 0

variable_val:   db  'variable val is: %d', 10, 0

msg3:           db  'the result is %d', 10, 0

cmd_msg1:       db  'argc = %d', 10, 0
cmd_msg2:       db  "'%s'", 10, 0

SECTION .bss

msg4_len:       equ     15
msg4:           resb    msg4_len



SECTION .text

extern printf
global main

main:
    ; 创建栈帧
    push    ebp
    mov     ebp, esp


    push    msg
    call    printf


    push    dword [int_i]
    push    dword msg_len
    push    msg_len_str
    call    printf


    ; if (x > y) {
    ;     msg1
    ; } else {
    ;     msg2
    ; }

    ; 比较大小
    mov     eax, dword [x]
    mov     ebx, dword [y]

    cmp     eax, ebx ; 比较x与y的值
    jg      .xGreatThany

.xGreatThany:
    push msg1
    call printf
    jmp .done

.xLessThany:
    push msg2
    call printf
    jmp .done

.done:


    ; 在栈空间中使用变量

    ; 分配栈空间
    sub     esp, 16
    mov     dword [ebp - 4], 0xff
    mov     dword [ebp - 8], 0xfe

    push    dword [ebp - 4]
    push    dword variable_val
    call    printf

    push    dword [ebp - 8]
    push    dword variable_val
    call    printf

    add     esp, 16



    ; 乘除法, (edx:eax) / ecx
    mov     eax, 80
    mov     edx, 0
    mov     ecx, 2

    mul     ecx

    push    eax
    push    msg3
    call    printf

    add     esp, 8


    mov     eax, 80
    mov     edx, 0
    mov     ecx, 2

    div     ecx

    push    eax
    push    msg3
    call    printf

    add     esp, 8


    ; syscall

    ; 从标准输入读取
    mov     eax, 3      ;
    mov     ebx, 0      ; 标准输入
    mov     ecx, msg4
    mov     edx, msg4_len     ; 最大字节数

    int     0x80        ; 系统中断

    ; 输出到标准输出
    mov     eax, 4
    mov     ebx, 1      ; 标准输出
    mov     ecx, msg4
    mov     edx, 15

    int     0x80        ; 系统中断


    ; 打印命令行参数
    mov     ecx, dword [ebp + 8]    ; 第一个参数
    mov     ebx, dword [ebp + 12]   ; 第二个参数

    push    ecx
    push    cmd_msg1
    call    printf

    add     esp, 4
    pop     ecx

.print_cmd:

    push    ebx
    push    ecx

    push    dword [ebx]
    push    cmd_msg2
    call    printf

    add     esp, 8

    pop    ecx
    pop    ebx

    dec     ecx
    cmp     ecx, 0
    jng      .gone1

    add     ebx, 4
    jmp     .print_cmd
.gone1:

    ; 销毁栈帧
    mov     esp, ebp
    pop     ebp
    ret
