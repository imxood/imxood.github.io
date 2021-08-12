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
		typedef ip::tcp::acceptor acceptor_type;
		typedef ip::tcp::endpoint endpoint_type;
		typedef ip::tcp::socket socket_type;

		cout << "server start." << endl;
		io_service io;

		acceptor_type acceptor(io, endpoint_type(ip::tcp::v4(), 6688));
		cout << "server: " << acceptor.local_endpoint().address() << endl;

		for (;;) {
			socket_type sock(io);
			acceptor.accept(sock);

			cout << "client: " << sock.remote_endpoint().address() << endl;
			sock.send(buffer("hello asio"));
		}
	}
	catch (const std::exception &e)
	{
		std::cerr << e.what() << '\n';
	}

	return 0;
}
