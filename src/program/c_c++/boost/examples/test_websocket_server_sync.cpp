#include <iostream>
#include <thread>
#include <boost/beast/core.hpp>
#include <boost/beast/websocket.hpp>
#include <boost/asio/ip/tcp.hpp>

using std::cerr;
using std::cout;
using std::endl;

using namespace boost;
namespace websocket = boost::beast::websocket;

using boost::asio::ip::tcp;

void do_session(tcp::socket &socket)
{
	try
	{
		websocket::stream<tcp::socket> ws{std::move(socket)};
		ws.accept();

		for (;;)
		{
			beast::multi_buffer buffer;
			ws.read(buffer);
			ws.text(ws.got_text());
			ws.write(buffer.data());
		}
	}
	catch (system::system_error const &se)
	{
		if (se.code() != websocket::error::closed)
			std::cerr << "Error: " << se.code().message() << endl;
		else
			std::cerr << "client closed" << endl;
	}
	catch (const std::exception &e)
	{
		std::cerr << e.what() << '\n';
	}
}

int main(int argc, char const *argv[])
{
	try
	{
		auto const address = asio::ip::address::from_string("127.0.0.1");
		auto const port = static_cast<unsigned short>(6688);

		asio::io_context ioc{1};

		tcp::acceptor acceptor{ioc, {address, port}};

		for (;;)
		{
			tcp::socket socket{ioc};
			acceptor.accept(socket);
			std::thread{std::bind(&do_session, std::move(socket))}.detach();
		}
	}
	catch (const std::exception &e)
	{
		std::cerr << e.what() << '\n';
	}

	return 0;
}
