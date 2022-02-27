#define BOOST_ASIO_ENABLE_HANDLER_TRACKING

#include <iostream>
#include <memory>
#include <boost/asio.hpp>
#include <boost/asio/ip/tcp.hpp>

using namespace boost;
using namespace asio;
using namespace std;

class Server
{
	typedef Server this_type;
	typedef ip::tcp::acceptor acceptor_type;
	typedef ip::tcp::endpoint endpoint_type;
	typedef ip::tcp::socket socket_type;
	typedef std::shared_ptr<socket_type> socket_ptr;

private:
	io_service m_io;
	acceptor_type m_acceptor;

public:
	Server() : m_acceptor(m_io, endpoint_type(ip::tcp::v4(), 6688))
	{
		accept();
	}

	void run()
	{
		m_io.run();
	}

	void accept()
	{
		socket_ptr sock(new socket_type(m_io));
		m_acceptor.async_accept(*sock, [this, sock](const error_code &ec) {
			if (ec)
				return;
			cout << "client: " << sock->remote_endpoint().address() << endl;

			/* sock->send(buffer("hello asio"));
			accept(); */

			sock->async_send(buffer("hello asio"), [](const error_code &ec, std::size_t) {
				cout << "send msg successed." << endl;
			});
			accept();
		});
	}
};

int main(int argc, char const *argv[])
{
	try
	{
		cout << "server start." << endl;
		Server s;
		s.run();
	}
	catch (const std::exception &e)
	{
		std::cerr << e.what() << '\n';
	}

	return 0;
}
