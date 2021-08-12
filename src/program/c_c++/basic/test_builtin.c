#include <stdio.h>

int main(int argc, char const *argv[])
{
	int a = 0b110;
	// 2
	printf("a: %d, __builtin_ffs(a): %d\n", a, __builtin_ffs(a));
	return 0;
}
