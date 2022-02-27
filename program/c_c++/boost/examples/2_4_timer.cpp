#include <iostream>
#include <thread>
#include <boost/progress.hpp>

using namespace std;
using namespace boost;

int main()
{
	int size = 10;
	progress_display d(size);
	for (int i = 0; i < size; i++) {
		this_thread::sleep_for(chrono::milliseconds(100));
		++d;
	}
}
