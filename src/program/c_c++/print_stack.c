
#include <stdio.h>
#include <execinfo.h>

#define STACK_SIZE 32

static void printStack(void)
{
	void *trace[STACK_SIZE];
	size_t size = backtrace(trace, STACK_SIZE);
	char **symbols = (char **)backtrace_symbols(trace,size);
	size_t i = 0;
	for(; i<size; i++)
	{
		printf("%s\n", symbols[i]);
	}
	return;
}

int main(int argc, char const *argv[])
{
    printStack();
    return 0;
}
