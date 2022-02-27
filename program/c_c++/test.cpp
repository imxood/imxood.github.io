#include <iostream>

int main(int argc, char const *argv[])
{
	int i = 10;
	int &ii = i;
	int c = i;
	int d = ii;

	int &&iii = 10;
	int e = iii;

	iii++;

	std::cout << iii << std::endl;

	// std::string str = "hello";
	// std::cout << str.starts_with("he") << std::endl;

	return 0;
}
