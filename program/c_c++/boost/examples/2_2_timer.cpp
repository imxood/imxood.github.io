#include <iostream>
#include <sstream>
#include <boost/version.hpp> //包含 Boost 头文件
#include <boost/config.hpp>	 //包含 Boost 头文件
#include <boost/timer.hpp>
#include <boost/progress.hpp>

using namespace std;
using namespace boost;

int main()
{
	timer t;
	progress_timer pt;
	stringstream ss;
	{
		progress_timer pt(ss);
	}
	cout << "timespan: " << t.elapsed() << "s" << endl;
	cout << "min timespan: " << t.elapsed_min() << "s" << endl;
	cout << "max timespan: " << t.elapsed_max() / 3600 << "h" << endl;
	cout << ss.str() << endl;
}
