   1              		.file	"test.c"
   2              		.text
   3              	.Ltext0:
   4              		.section	.rodata
   5              	.LC0:
   6 0000 73697A65 		.string	"size of struct test1: %lu\n"
   6      206F6620 
   6      73747275 
   6      63742074 
   6      65737431 
   7 001b 00000000 		.align 8
   7      00
   8              	.LC1:
   9 0020 73697A65 		.string	"size of __packed__ struct test2: %lu\n"
   9      206F6620 
   9      5F5F7061 
   9      636B6564 
   9      5F5F2073 
  10 0046 0000     		.align 8
  11              	.LC2:
  12 0048 73697A65 		.string	"size of aligned(16) struct test3: %lu\n"
  12      206F6620 
  12      616C6967 
  12      6E656428 
  12      31362920 
  13              		.text
  14              		.globl	test_struct
  16              	test_struct:
  17              	.LFB5:
  18              		.file 1 "test.c"
   1:test.c        **** #include <stdio.h>
   2:test.c        **** #include <stdlib.h>
   3:test.c        **** #include <string.h>
   4:test.c        **** #include <stdint.h>
   5:test.c        **** 
   6:test.c        **** #define SECTION(x) 	__attribute__((section(x)))
   7:test.c        **** 
   8:test.c        **** void test_struct()
   9:test.c        **** {
  19              		.loc 1 9 1
  20              		.cfi_startproc
  21 0000 55       		pushq	%rbp
  22              		.cfi_def_cfa_offset 16
  23              		.cfi_offset 6, -16
  24 0001 4889E5   		movq	%rsp, %rbp
  25              		.cfi_def_cfa_register 6
  10:test.c        **** 
  11:test.c        **** 	// #pragma pack(4)
  12:test.c        **** 
  13:test.c        **** 	struct test1
  14:test.c        **** 	{
  15:test.c        **** 		char a[31];
  16:test.c        **** 		short b;
  17:test.c        **** 		char b_1;
  18:test.c        **** 		char b_2;
  19:test.c        **** 		char b_3;
  20:test.c        **** 		char b_4;
  21:test.c        **** 		int c;
  22:test.c        **** 		double d;
  23:test.c        **** 	};
  24:test.c        **** 
  25:test.c        **** 	struct __attribute__((__packed__)) test2
  26:test.c        **** 	{
  27:test.c        **** 		char a[31];
  28:test.c        **** 		short b;
  29:test.c        **** 		char b_1;
  30:test.c        **** 		char b_2;
  31:test.c        **** 		char b_3;
  32:test.c        **** 		char b_4;
  33:test.c        **** 		int c;
  34:test.c        **** 		double d;
  35:test.c        **** 	};
  36:test.c        **** 
  37:test.c        **** 	struct __attribute__((aligned(32))) test3
  38:test.c        **** 	{
  39:test.c        **** 		char a[31];
  40:test.c        **** 		short b;
  41:test.c        **** 		char b_1;
  42:test.c        **** 		char b_2;
  43:test.c        **** 		char b_3;
  44:test.c        **** 		char b_4;
  45:test.c        **** 		int c;
  46:test.c        **** 		double d;
  47:test.c        **** 	};
  48:test.c        **** 
  49:test.c        **** 	printf("size of struct test1: %lu\n", sizeof(struct test1));
  26              		.loc 1 49 2
  27 0004 BE380000 		movl	$56, %esi
  27      00
  28 0009 BF000000 		movl	$.LC0, %edi
  28      00
  29 000e B8000000 		movl	$0, %eax
  29      00
  30 0013 E8000000 		call	printf
  30      00
  50:test.c        **** 
  51:test.c        **** 	printf("size of __packed__ struct test2: %lu\n", sizeof(struct test2));
  31              		.loc 1 51 2
  32 0018 BE310000 		movl	$49, %esi
  32      00
  33 001d BF000000 		movl	$.LC1, %edi
  33      00
  34 0022 B8000000 		movl	$0, %eax
  34      00
  35 0027 E8000000 		call	printf
  35      00
  52:test.c        **** 
  53:test.c        **** 	printf("size of aligned(16) struct test3: %lu\n", sizeof(struct test3));
  36              		.loc 1 53 2
  37 002c BE400000 		movl	$64, %esi
  37      00
  38 0031 BF000000 		movl	$.LC2, %edi
  38      00
  39 0036 B8000000 		movl	$0, %eax
  39      00
  40 003b E8000000 		call	printf
  40      00
  54:test.c        **** }
  41              		.loc 1 54 1
  42 0040 90       		nop
  43 0041 5D       		popq	%rbp
  44              		.cfi_def_cfa 7, 8
  45 0042 C3       		ret
  46              		.cfi_endproc
  47              	.LFE5:
  49              		.section	.rodata
  50              	.LC3:
  51 006f 73697A65 		.string	"size of bit_field: %lu\n"
  51      206F6620 
  51      6269745F 
  51      6669656C 
  51      643A2025 
  52              		.text
  53              		.globl	test_bitfield
  55              	test_bitfield:
  56              	.LFB6:
  55:test.c        **** 
  56:test.c        **** void test_bitfield()
  57:test.c        **** {
  57              		.loc 1 57 1
  58              		.cfi_startproc
  59 0043 55       		pushq	%rbp
  60              		.cfi_def_cfa_offset 16
  61              		.cfi_offset 6, -16
  62 0044 4889E5   		movq	%rsp, %rbp
  63              		.cfi_def_cfa_register 6
  58:test.c        **** 	struct __attribute__ ((aligned(64))) bit_field
  59:test.c        **** 	{
  60:test.c        **** 		uint8_t a : 4; /* 独占一个 uint8_t */
  61:test.c        **** 		uint8_t b : 5; /* 4+5 > 8, 从新的存储单元开始存放*/
  62:test.c        **** 		uint8_t : 3;   /* 空域, 5+3 = 8, 这个位域和前一个位域刚好填满一个存储单元*/
  63:test.c        **** 		uint32_t c : 3; /* 独占一个 uint32_t */
  64:test.c        **** 	};
  65:test.c        **** 
  66:test.c        **** 	printf("size of bit_field: %lu\n", sizeof(struct bit_field));
  64              		.loc 1 66 2
  65 0047 BE400000 		movl	$64, %esi
  65      00
  66 004c BF000000 		movl	$.LC3, %edi
  66      00
  67 0051 B8000000 		movl	$0, %eax
  67      00
  68 0056 E8000000 		call	printf
  68      00
  67:test.c        **** }
  69              		.loc 1 67 1
  70 005b 90       		nop
  71 005c 5D       		popq	%rbp
  72              		.cfi_def_cfa 7, 8
  73 005d C3       		ret
  74              		.cfi_endproc
  75              	.LFE6:
  77              		.globl	a1
  78              		.section	seg_a,"aw"
  79              		.align 4
  82              	a1:
  83 0000 00000000 		.zero	4
  84              		.globl	a2
  85              		.align 4
  88              	a2:
  89 0004 00000000 		.zero	4
  90              		.globl	a3
  91              		.align 4
  94              	a3:
  95 0008 00000000 		.zero	4
  96              		.section	.rodata
  97              	.LC4:
  98 0087 49276D20 		.string	"I'm %s\n"
  98      25730A00 
  99              		.text
 100              		.globl	version
 102              	version:
 103              	.LFB7:
  68:test.c        **** 
  69:test.c        **** int a1 SECTION("seg_a");
  70:test.c        **** int a2 SECTION("seg_a");
  71:test.c        **** int a3 SECTION("seg_a");
  72:test.c        **** 
  73:test.c        **** 
  74:test.c        **** 
  75:test.c        **** 
  76:test.c        **** /***********************************************************************************/
  77:test.c        **** 
  78:test.c        **** typedef long (*syscall_func)(void);
  79:test.c        **** 
  80:test.c        **** struct finsh_syscall
  81:test.c        **** {
  82:test.c        ****     const char*     name;       /* the name of system call */
  83:test.c        ****     syscall_func func;      /* the function address of system call */
  84:test.c        **** };
  85:test.c        **** 
  86:test.c        **** #define FINSH_FUNCTION_EXPORT_CMD(name, cmd)                      			\
  87:test.c        ****                 const char __fsym_##cmd##_name[] = #cmd;                            \
  88:test.c        ****                 const struct finsh_syscall __fsym_##cmd SECTION("FSymTab")= 		\
  89:test.c        ****                 {                                                                   \
  90:test.c        ****                     __fsym_##cmd##_name,                                            \
  91:test.c        ****                     (syscall_func)&name                                             \
  92:test.c        ****                 };
  93:test.c        **** 
  94:test.c        **** #define FINSH_FUNCTION_EXPORT(name)   										\
  95:test.c        ****     FINSH_FUNCTION_EXPORT_CMD(name, name)
  96:test.c        **** 
  97:test.c        **** long version(void)
  98:test.c        **** {
 104              		.loc 1 98 1
 105              		.cfi_startproc
 106 005e 55       		pushq	%rbp
 107              		.cfi_def_cfa_offset 16
 108              		.cfi_offset 6, -16
 109 005f 4889E5   		movq	%rsp, %rbp
 110              		.cfi_def_cfa_register 6
  99:test.c        **** 	printf("I'm %s\n", __func__);
 111              		.loc 1 99 2
 112 0062 BE000000 		movl	$__func__.3139, %esi
 112      00
 113 0067 BF000000 		movl	$.LC4, %edi
 113      00
 114 006c B8000000 		movl	$0, %eax
 114      00
 115 0071 E8000000 		call	printf
 115      00
 100:test.c        ****     return 0;
 116              		.loc 1 100 12
 117 0076 B8000000 		movl	$0, %eax
 117      00
 101:test.c        **** }
 118              		.loc 1 101 1
 119 007b 5D       		popq	%rbp
 120              		.cfi_def_cfa 7, 8
 121 007c C3       		ret
 122              		.cfi_endproc
 123              	.LFE7:
 125              		.globl	__fsym_version_name
 126              		.section	.rodata
 127 008f 00       		.align 8
 130              	__fsym_version_name:
 131 0090 76657273 		.string	"version"
 131      696F6E00 
 132              		.globl	__fsym_version
 133              		.section	FSymTab,"a"
 134              		.align 16
 137              	__fsym_version:
 138 0000 00000000 		.quad	__fsym_version_name
 138      00000000 
 139 0008 00000000 		.quad	version
 139      00000000 
 140              		.text
 141              		.globl	hello
 143              	hello:
 144              	.LFB8:
 102:test.c        **** FINSH_FUNCTION_EXPORT(version);
 103:test.c        **** 
 104:test.c        **** long hello(void)
 105:test.c        **** {
 145              		.loc 1 105 1
 146              		.cfi_startproc
 147 007d 55       		pushq	%rbp
 148              		.cfi_def_cfa_offset 16
 149              		.cfi_offset 6, -16
 150 007e 4889E5   		movq	%rsp, %rbp
 151              		.cfi_def_cfa_register 6
 106:test.c        **** 	printf("I'm %s\n", __func__);
 152              		.loc 1 106 2
 153 0081 BE000000 		movl	$__func__.3145, %esi
 153      00
 154 0086 BF000000 		movl	$.LC4, %edi
 154      00
 155 008b B8000000 		movl	$0, %eax
 155      00
 156 0090 E8000000 		call	printf
 156      00
 107:test.c        ****     return 0;
 157              		.loc 1 107 12
 158 0095 B8000000 		movl	$0, %eax
 158      00
 108:test.c        **** }
 159              		.loc 1 108 1
 160 009a 5D       		popq	%rbp
 161              		.cfi_def_cfa 7, 8
 162 009b C3       		ret
 163              		.cfi_endproc
 164              	.LFE8:
 166              		.globl	__fsym_hello_name
 167              		.section	.rodata
 170              	__fsym_hello_name:
 171 0098 68656C6C 		.string	"hello"
 171      6F00
 172              		.globl	__fsym_hello
 173              		.section	FSymTab
 174              		.align 16
 177              	__fsym_hello:
 178 0010 00000000 		.quad	__fsym_hello_name
 178      00000000 
 179 0018 00000000 		.quad	hello
 179      00000000 
 180              		.globl	_syscall_table_begin
 181              		.bss
 182              		.align 8
 185              	_syscall_table_begin:
 186 0000 00000000 		.zero	8
 186      00000000 
 187              		.globl	_syscall_table_end
 188              		.align 8
 191              	_syscall_table_end:
 192 0008 00000000 		.zero	8
 192      00000000 
 193              		.section	.rodata
 194              	.LC5:
 195 009e 25733A20 		.string	"%s: "
 195      00
 196              		.text
 197              		.globl	finsh_system_function_init
 199              	finsh_system_function_init:
 200              	.LFB9:
 109:test.c        **** FINSH_FUNCTION_EXPORT(hello);
 110:test.c        **** 
 111:test.c        **** extern struct finsh_syscall __start_FSymTab;
 112:test.c        **** extern struct finsh_syscall __stop_FSymTab;
 113:test.c        **** 
 114:test.c        **** struct finsh_syscall *_syscall_table_begin  = NULL;
 115:test.c        **** struct finsh_syscall *_syscall_table_end    = NULL;
 116:test.c        **** 
 117:test.c        **** void finsh_system_function_init()
 118:test.c        **** {
 201              		.loc 1 118 1
 202              		.cfi_startproc
 203 009c 55       		pushq	%rbp
 204              		.cfi_def_cfa_offset 16
 205              		.cfi_offset 6, -16
 206 009d 4889E5   		movq	%rsp, %rbp
 207              		.cfi_def_cfa_register 6
 208 00a0 4883EC10 		subq	$16, %rsp
 119:test.c        ****     // _syscall_table_begin = ;
 120:test.c        ****     // _syscall_table_end = ;
 121:test.c        **** 	// printf("_syscall_table_begin: %p, _syscall_table_end: %p\n", _syscall_table_begin, _syscall_tab
 122:test.c        **** 	// printf("length: 0x%x\n", (uint8_t*)_syscall_table_end - (uint8_t*)_syscall_table_begin);
 123:test.c        **** 	int i = 0;
 209              		.loc 1 123 6
 210 00a4 C745FC00 		movl	$0, -4(%rbp)
 210      000000
 211              	.LBB2:
 124:test.c        **** 	for (struct finsh_syscall *iter = &__start_FSymTab; iter < &__stop_FSymTab; ++iter, ++i) {
 212              		.loc 1 124 29
 213 00ab 48C745F0 		movq	$__start_FSymTab, -16(%rbp)
 213      00000000 
 214              		.loc 1 124 2
 215 00b3 EB2C     		jmp	.L8
 216              	.L9:
 125:test.c        **** 		printf("%s: ", iter->name);
 217              		.loc 1 125 3 discriminator 3
 218 00b5 488B45F0 		movq	-16(%rbp), %rax
 219 00b9 488B00   		movq	(%rax), %rax
 220 00bc 4889C6   		movq	%rax, %rsi
 221 00bf BF000000 		movl	$.LC5, %edi
 221      00
 222 00c4 B8000000 		movl	$0, %eax
 222      00
 223 00c9 E8000000 		call	printf
 223      00
 126:test.c        **** 		iter->func();
 224              		.loc 1 126 7 discriminator 3
 225 00ce 488B45F0 		movq	-16(%rbp), %rax
 226 00d2 488B4008 		movq	8(%rax), %rax
 227              		.loc 1 126 3 discriminator 3
 228 00d6 FFD0     		call	*%rax
 229              	.LVL0:
 124:test.c        **** 	for (struct finsh_syscall *iter = &__start_FSymTab; iter < &__stop_FSymTab; ++iter, ++i) {
 230              		.loc 1 124 78 discriminator 3
 231 00d8 488345F0 		addq	$16, -16(%rbp)
 231      10
 124:test.c        **** 	for (struct finsh_syscall *iter = &__start_FSymTab; iter < &__stop_FSymTab; ++iter, ++i) {
 232              		.loc 1 124 86 discriminator 3
 233 00dd 8345FC01 		addl	$1, -4(%rbp)
 234              	.L8:
 124:test.c        **** 	for (struct finsh_syscall *iter = &__start_FSymTab; iter < &__stop_FSymTab; ++iter, ++i) {
 235              		.loc 1 124 2 discriminator 1
 236 00e1 48817DF0 		cmpq	$__stop_FSymTab, -16(%rbp)
 236      00000000 
 237 00e9 72CA     		jb	.L9
 238              	.LBE2:
 127:test.c        **** 		// printf("i: %d\n", i);
 128:test.c        **** 	}
 129:test.c        **** }
 239              		.loc 1 129 1
 240 00eb 90       		nop
 241 00ec 90       		nop
 242 00ed C9       		leave
 243              		.cfi_def_cfa 7, 8
 244 00ee C3       		ret
 245              		.cfi_endproc
 246              	.LFE9:
 248              		.globl	main
 250              	main:
 251              	.LFB10:
 130:test.c        **** 
 131:test.c        **** 
 132:test.c        **** // FSymTab         0x00000000004006b0       0x20
 133:test.c        **** //  FSymTab        0x00000000004006b0       0x20 /tmp/ccw7kKou.o
 134:test.c        **** //                 0x00000000004006b0                __fsym_version
 135:test.c        **** //                 0x00000000004006c0                __fsym_hello
 136:test.c        **** 
 137:test.c        **** // 根据map文件可知, 多个相同的SECTION会进入同一块段中
 138:test.c        **** 
 139:test.c        **** /***********************************************************************************/
 140:test.c        **** 
 141:test.c        **** int main(int argc, char const *argv[])
 142:test.c        **** {
 252              		.loc 1 142 1
 253              		.cfi_startproc
 254 00ef 55       		pushq	%rbp
 255              		.cfi_def_cfa_offset 16
 256              		.cfi_offset 6, -16
 257 00f0 4889E5   		movq	%rsp, %rbp
 258              		.cfi_def_cfa_register 6
 259 00f3 4883EC10 		subq	$16, %rsp
 260 00f7 897DFC   		movl	%edi, -4(%rbp)
 261 00fa 488975F0 		movq	%rsi, -16(%rbp)
 143:test.c        **** 	test_struct();
 262              		.loc 1 143 2
 263 00fe B8000000 		movl	$0, %eax
 263      00
 264 0103 E8000000 		call	test_struct
 264      00
 144:test.c        **** 
 145:test.c        **** 	test_bitfield();
 265              		.loc 1 145 2
 266 0108 B8000000 		movl	$0, %eax
 266      00
 267 010d E8000000 		call	test_bitfield
 267      00
 146:test.c        **** 
 147:test.c        **** 	finsh_system_function_init();
 268              		.loc 1 147 2
 269 0112 B8000000 		movl	$0, %eax
 269      00
 270 0117 E8000000 		call	finsh_system_function_init
 270      00
 148:test.c        **** 
 149:test.c        **** 	return 0;
 271              		.loc 1 149 9
 272 011c B8000000 		movl	$0, %eax
 272      00
 150:test.c        **** }
 273              		.loc 1 150 1
 274 0121 C9       		leave
 275              		.cfi_def_cfa 7, 8
 276 0122 C3       		ret
 277              		.cfi_endproc
 278              	.LFE10:
 280              		.section	.rodata
 281 00a3 00000000 		.align 8
 281      00
 284              	__func__.3139:
 285 00a8 76657273 		.string	"version"
 285      696F6E00 
 288              	__func__.3145:
 289 00b0 68656C6C 		.string	"hello"
 289      6F00
 290              		.text
 291              	.Letext0:
 292              		.file 2 "/usr/local/lib/gcc/x86_64-pc-linux-gnu/9.2.0/include/stddef.h"
 293              		.file 3 "/usr/include/x86_64-linux-gnu/bits/types.h"
 294              		.file 4 "/usr/include/x86_64-linux-gnu/bits/libio.h"
 295              		.file 5 "/usr/include/stdio.h"
 296              		.file 6 "/usr/include/x86_64-linux-gnu/bits/sys_errlist.h"
