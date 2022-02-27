#include <iostream>
#include <boost/version.hpp> //包含 Boost 头文件
#include <boost/config.hpp>	 //包含 Boost 头文件
#include <boost/timer.hpp>

using namespace std;
using namespace boost;

int main()
{
	cout << BOOST_VERSION << endl;	   //Boost 版本号
	cout << BOOST_LIB_VERSION << endl; //Boost 版本号
	cout << BOOST_PLATFORM << endl;	   //操作系统
	cout << BOOST_COMPILER << endl;	   //编译器
	cout << BOOST_STDLIB << endl;	   //标准库
}
