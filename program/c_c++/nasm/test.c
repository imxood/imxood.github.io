#include <stdio.h>

char *test()
{
	char s[10];
	for (size_t i = 0; i < 10; i++)
	{
		for (size_t j = 0; j < 10; j++)
		{
			s[i] = j + 0x30;
		}
	}
	s[0] = 0x55;
	s[0] = 0x55;
	s[0] = 0x55;
	s[0] = 0x55;
	s[0] = 0x55;

	printf("hello world\n");

	return s;
}
