#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#define SECTION(x) 	__attribute__((section(x)))

void test_struct()
{

	// #pragma pack(4)

	struct test1
	{
		char a[31];
		short b;
		char b_1;
		char b_2;
		char b_3;
		char b_4;
		int c;
		double d;
	};

	struct __attribute__((__packed__)) test2
	{
		char a[31];
		short b;
		char b_1;
		char b_2;
		char b_3;
		char b_4;
		int c;
		double d;
	};

	struct __attribute__((aligned(32))) test3
	{
		char a[31];
		short b;
		char b_1;
		char b_2;
		char b_3;
		char b_4;
		int c;
		double d;
	};

	printf("size of struct test1: %lu\n", sizeof(struct test1));

	printf("size of __packed__ struct test2: %lu\n", sizeof(struct test2));

	printf("size of aligned(16) struct test3: %lu\n", sizeof(struct test3));
}

void test_bitfield()
{
	struct __attribute__ ((aligned(64))) bit_field
	{
		uint8_t a : 4; /* 独占一个 uint8_t */
		uint8_t b : 5; /* 4+5 > 8, 从新的存储单元开始存放*/
		uint8_t : 3;   /* 空域, 5+3 = 8, 这个位域和前一个位域刚好填满一个存储单元*/
		uint32_t c : 3; /* 独占一个 uint32_t */
	};

	printf("size of bit_field: %lu\n", sizeof(struct bit_field));
}

int a1 SECTION("seg_a");
int a2 SECTION("seg_a");
int a3 SECTION("seg_a");




/***********************************************************************************/

typedef long (*syscall_func)(void);

struct finsh_syscall
{
    const char*     name;       /* the name of system call */
    syscall_func func;      /* the function address of system call */
};

#define FINSH_FUNCTION_EXPORT_CMD(name, cmd)                      			\
                const char __fsym_##cmd##_name[] = #cmd;                            \
                const struct finsh_syscall __fsym_##cmd SECTION("FSymTab")= 		\
                {                                                                   \
                    __fsym_##cmd##_name,                                            \
                    (syscall_func)&name                                             \
                };

#define FINSH_FUNCTION_EXPORT(name)   										\
    FINSH_FUNCTION_EXPORT_CMD(name, name)

long version(void)
{
	printf("I'm %s\n", __func__);
    return 0;
}
FINSH_FUNCTION_EXPORT(version);

long hello(void)
{
	printf("I'm %s\n", __func__);
    return 0;
}
FINSH_FUNCTION_EXPORT(hello);

extern struct finsh_syscall __start_FSymTab;
extern struct finsh_syscall __stop_FSymTab;

struct finsh_syscall *_syscall_table_begin  = NULL;
struct finsh_syscall *_syscall_table_end    = NULL;

void finsh_system_function_init()
{
	for (struct finsh_syscall *iter = &__start_FSymTab; iter < &__stop_FSymTab; ++iter) {
		printf("%s: ", iter->name);
		iter->func();
	}
}


// FSymTab         0x00000000004006b0       0x20
//  FSymTab        0x00000000004006b0       0x20 /tmp/ccw7kKou.o
//                 0x00000000004006b0                __fsym_version
//                 0x00000000004006c0                __fsym_hello

// 根据map文件可知, 多个相同的SECTION会进入同一块段中, 每一个SECTION的名称, 都会有一个对应的__start_SECTIONNAME 和 __stop_SECTIONNAME变量
// 遍历这对变量就可以获取, 这个段上所有相同类型的变量数组了
/***********************************************************************************/

int main(int argc, char const *argv[])
{
	test_struct();

	test_bitfield();

	finsh_system_function_init();

	return 0;
}
