#include <iostream>
#include <boost/program_options.hpp>

/* 实例可以在boost的 libs/program_options/example 下查看 */

namespace po = boost::program_options;

using namespace std;
using namespace boost;


// A helper function to simplify the main part.
template<class T>
ostream& operator<<(ostream& os, const vector<T>& v)
{
    copy(v.begin(), v.end(), ostream_iterator<T>(os, " "));
    return os;
}

int main(int argc, char const *argv[])
{
	po::options_description desc("Allowed options");
	desc.add_options()
		("help,h", "product help message")
		("include-path,I", po::value< vector<string> >(), "include path")
		("debug", "set debug level")
	;

	po::variables_map vm;
	po::store(po::parse_command_line(argc, argv, desc), vm);
	po::notify(vm);

	if (vm.count("help"))
	{
		cout << desc << "\n";
		return 1;
	}

	vector<string> test = {"hello", "world"};

	if (vm.count("include-path")) {
		cout << "Include paths are: " << vm["include-path"].as< vector<string> >() << "\n";
	}

	return 0;
}
