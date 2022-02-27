#include <iostream>
#include <thread>
#include <string>
#include <cstdlib>
#include <boost/beast/core.hpp>
#include <boost/beast/websocket.hpp>
#include <boost/asio/ip/tcp.hpp>

using std::cerr;
using std::cout;
using std::endl;

using namespace boost;

using boost::asio::ip::tcp;

void do_session(tcp::socket &socket)
{
	try
	{
		beast::websocket::stream<tcp::socket> ws{std::move(socket)};
		ws.accept();

		for (;;)
		{
			beast::multi_buffer buffer;
			ws.read(buffer);
			ws.text(ws.got_text());
			ws.write(buffer.data());
		}
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
		const char *host = "127.0.0.1";
		// auto const address = asio::ip::address::from_string("127.0.0.1");
		auto const *port = "6688";

		asio::io_context ioc{1};

		tcp::resolver resolver{ioc};
		beast::websocket::stream<tcp::socket> ws{ioc};

		auto const results = resolver.resolve(host, port);
		asio::connect(ws.next_layer(), results.begin(), results.end());

		ws.handshake(host, "/");

		ws.write(asio::buffer("hello websocket"));

		beast::flat_buffer buffer;
		do
		{
			ws.read_some(buffer, 512);
		} while (!ws.is_message_done());

		ws.close(beast::websocket::close_code::normal);

		cout << beast::make_printable(buffer.data()) << endl;
	}
	catch (const std::exception &e)
	{
		std::cerr << e.what() << '\n';
	}

	return 0;
}
