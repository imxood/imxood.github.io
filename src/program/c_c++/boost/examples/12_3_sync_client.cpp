#define BOOST_ASIO_ENABLE_HANDLER_TRACKING

#include <iostream>
#include <boost/asio.hpp>
#include <boost/asio/ip/tcp.hpp>

using namespace std;
using namespace boost;
using namespace asio;

int main(int argc, char const *argv[])
{
	try
	{
		typedef ip::tcp::endpoint endpoint_type;
		typedef ip::tcp::socket socket_type;
		typedef ip::address address_type;

		cout << "client start." << endl;
		io_service io;

		socket_type sock(io);
		endpoint_type endpoint(address_type::from_string("127.0.0.1"), 6688);

		sock.connect(endpoint);
		cout << "sock available: " << sock.available() << endl;

		vector<char> str(1024, 0);
		sock.receive(buffer(str));

		cout << "receive from " << sock.remote_endpoint().address() << endl;
		cout << "msg from server: " << &str[0] << endl;
	}
	catch (const std::exception &e)
	{
		std::cerr << e.what() << '\n';
	}

	return 0;
}
