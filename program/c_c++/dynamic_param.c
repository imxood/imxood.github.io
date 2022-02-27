#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <stdarg.h>

#define CASE_NAME_BUF_LENGTH 128

static char CASE_NAME_BUF[CASE_NAME_BUF_LENGTH];
static char CASE_NAME_INT_BUF[CASE_NAME_BUF_LENGTH];

static int CHAR_pos;

const char *CHAR(int number)
{
	char *buffer = CASE_NAME_INT_BUF + CHAR_pos;

	sprintf(buffer, "%d", number);

	uint32_t len = strlen(buffer);
	buffer[len] = 0;

	CHAR_pos += len + 1;

	return buffer;
}

const char *CHAR_BASE(int number, int base)
{
	char *buffer = CASE_NAME_INT_BUF + CHAR_pos;

	if (base == 16)
	{
		sprintf(buffer, "0x%x", number);
	}
	else if (base == 10)
	{
		sprintf(buffer, "%d", number);
	}
	else if (base == 8)
	{
		sprintf(buffer, "0o%o", number);
	}
	else
	{
		return "INVALIDATEBASE";
	}

	uint32_t len = strlen(buffer);
	buffer[len] = 0;

	CHAR_pos += len + 1;

	return buffer;
}

const char *CASE_NAME(int argc, ...)
{
	char *buffer = CASE_NAME_BUF;
	char *arg = NULL;

	va_list valist;
	va_start(valist, argc);

	for (int i = 0; i < argc; i++)
	{
		arg = va_arg(valist, char *);
		strcpy(buffer, arg);
		buffer += strlen(arg);
		*buffer = '-';
		buffer++;
	}

	*(--buffer) = '\0';

	return CASE_NAME_BUF;
}

int main(int argc, char const *argv[])
{
	const char *name = CASE_NAME(7, __func__, CHAR(0), CHAR_BASE(0x01, 16), CHAR(0x10), CHAR(0x10), CHAR(0x10), CHAR(0x10));

	// name = CASE_NAME(3, "hello", "world", INT_CAT(2, 1, 2));
	printf("%s\n", name);

	CHAR_pos = 0;

	return 0;
}
