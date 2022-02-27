// #define BOOST_ASIO_ENABLE_HANDLER_TRACKING

#include <iostream>
#include <boost/beast/core.hpp>
#include <boost/beast/websocket.hpp>
#include <boost/asio/ip/tcp.hpp>
#include <boost/asio/strand.hpp>

using namespace boost;
using asio::ip::tcp;
namespace websocket = beast::websocket;
namespace http = beast::http;

void fail(beast::error_code &ec, char const *what)
{
	std::cerr << what << ": " << ec.message() << std::endl;
}

class Session : public std::enable_shared_from_this<Session>
{
private:
	websocket::stream<beast::tcp_stream> m_ws;
	beast::flat_buffer m_buf;
	tcp::endpoint m_endpoint;

public:
	explicit Session(asio::io_context &ioc, tcp::endpoint endpoint)
		: m_ws(asio::make_strand(ioc)), m_endpoint(endpoint)
	{
	}

	void run()
	{
		beast::get_lowest_layer(m_ws).expires_after(std::chrono::seconds(30));
		beast::get_lowest_layer(m_ws).async_connect(m_endpoint, beast::bind_front_handler(&Session::on_connect, shared_from_this()));
	}

	void on_connect(beast::error_code ec)
	{
		if (ec)
			return fail(ec, "connect");
		beast::get_lowest_layer(m_ws).expires_never();
		m_ws.set_option(websocket::stream_base::timeout::suggested(beast::role_type::client));
		m_ws.set_option(websocket::stream_base::decorator([](websocket::request_type &req) {
			req.set(http::field::user_agent, std::string(BOOST_BEAST_VERSION_STRING) + " websocket-client-async");
		}));
		std::string host = m_endpoint.address().to_string();
		host += ":" + m_endpoint.port();
		m_ws.async_handshake(host, "/", beast::bind_front_handler(&Session::on_handshake, shared_from_this()));
	}

	void on_handshake(beast::error_code ec)
	{
		if (ec)
			return fail(ec, "handshake");
		m_ws.async_write(asio::buffer("hello async server"), beast::bind_front_handler(&Session::on_write, shared_from_this()));
	}

	void on_read(beast::error_code ec, size_t trnsffered_size)
	{
		if (ec)
			return fail(ec, "read");
		m_ws.async_close(websocket::close_code::normal, beast::bind_front_handler(&Session::on_close, shared_from_this()));
	}

	void on_close(beast::error_code ec)
	{
		if (ec)
			return fail(ec, "close");
	}

	void on_write(beast::error_code ec, size_t transferred_size)
	{
		if (ec)
			return fail(ec, "write");
		m_buf.consume(m_buf.size());
		m_ws.async_read(m_buf, beast::bind_front_handler(&Session::on_read, shared_from_this()));
	}
};

int main(int argc, char const *argv[])
{
	try
	{
		auto const address = asio::ip::make_address("127.0.0.1");
		const uint16_t port = 6688;

		asio::io_context ioc{1};

		std::make_shared<Session>(ioc, tcp::endpoint{address, port})->run();

		ioc.run();
	}
	catch (const std::exception &e)
	{
		std::cerr << e.what() << '\n';
	}

	return 0;
}
