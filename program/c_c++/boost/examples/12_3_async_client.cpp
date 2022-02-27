#define BOOST_ASIO_ENABLE_HANDLER_TRACKING

#include <iostream>
#include <memory>
#include <boost/bind.hpp>
#include <boost/asio.hpp>
#include <boost/asio/ip/tcp.hpp>

using namespace boost;

using namespace asio;
using namespace system;

using std::cerr;
using std::cout;
using std::endl;
using std::vector;

class Client
{
	typedef Client this_type;
	typedef ip::tcp::endpoint endpoint_type;
	typedef ip::tcp::socket socket_type;
	typedef ip::address address_type;
	typedef shared_ptr<socket_type> socket_ptr;
	typedef vector<char> buffer_type;

private:
	io_service m_io;
	buffer_type m_buf;
	endpoint_type m_endpoint;

public:
	Client() : m_buf(100, 0),
			   m_endpoint(address_type::from_string("127.0.0.1"), 6688)
	{
		start();
	}

	void run()
	{
		m_io.run();
	}

	void start()
	{
		socket_ptr sock(new socket_type(m_io));
		sock->async_connect(m_endpoint, [this, sock](const error_code &ec) {
			if (ec)
				return;
			cout << "server: " << sock->remote_endpoint().address() << endl;
			sock->async_read_some(
				buffer(m_buf),
				[this, sock](const error_code &ec, size_t) {
					read_handle(ec, sock);
				});
		});
	}

	void read_handle(const error_code &ec, socket_ptr sock)
	{
		if (ec)
			return;
		cout << "recv msg: " << &m_buf[0] << endl;

		sock->async_read_some(
			buffer(m_buf),
			bind(&this_type::read_handle, this, asio::placeholders::error, sock));
	}
};

int main(int argc, char const *argv[])
{
	try
	{
		cout << "server start." << endl;
		Client c;
		c.run();
	}
	catch (const std::exception &e)
	{
		cerr << e.what() << '\n';
	}

	return 0;
}
