#define BOOST_ASIO_ENABLE_HANDLER_TRACKING

#include <iostream>
#include <thread>
#include <memory>
#include <boost/beast/core.hpp>
#include <boost/beast/websocket.hpp>
#include <boost/asio/ip/tcp.hpp>
#include <boost/asio/strand.hpp>
#include <boost/asio/dispatch.hpp>

using std::cerr;
using std::cout;
using std::endl;

using namespace boost;
using asio::ip::tcp;
namespace websocket = beast::websocket;
namespace http = beast::http;
namespace ip = asio::ip;

void fail(beast::error_code &ec, char const *what)
{
	std::cerr << what << ": " << ec.message() << endl;
}

class Session : public std::enable_shared_from_this<Session>
{
private:
	websocket::stream<tcp::socket> m_ws;
	beast::flat_buffer m_buf;

public:
	explicit Session(tcp::socket &&socket)
		: m_ws(std::move(socket))
	{
	}

	void run()
	{
		asio::dispatch(m_ws.get_executor(), beast::bind_front_handler(&Session::on_run, shared_from_this()));
	}

	void on_run()
	{
		m_ws.set_option(websocket::stream_base::timeout::suggested(beast::role_type::server));
		m_ws.set_option(websocket::stream_base::decorator([](websocket::response_type &res) {
			res.set(http::field::server, std::string(BOOST_BEAST_VERSION_STRING) + " websocket-server-async");
		}));

		m_ws.async_accept(beast::bind_front_handler(&Session::on_accept, shared_from_this()));
	}

	void on_accept(beast::error_code ec)
	{
		if (ec)
			return fail(ec, "accept");
		do_read();
	}

	void do_read()
	{
		m_ws.async_read(m_buf, beast::bind_front_handler(&Session::on_read, shared_from_this()));
	}

	void on_read(beast::error_code ec, size_t trnsffered_size)
	{
		if (ec == websocket::error::closed)
		{
			cout << "client closed" << endl;
			return;
		}
		if (ec)
			return fail(ec, "read");
		m_ws.text(m_ws.got_text());
		m_ws.async_write(m_buf.data(), beast::bind_front_handler(&Session::on_write, shared_from_this()));
	}

	void on_write(beast::error_code ec, size_t transferred_size)
	{
		if (ec)
			return fail(ec, "write");
		m_buf.consume(m_buf.size());
		do_read();
	}
};

class Listener : public std::enable_shared_from_this<Listener>
{
private:
	asio::io_context &m_ioc;
	tcp::acceptor m_acceptor;

public:
	Listener(asio::io_context &ioc, tcp::endpoint endpoint) : m_ioc(ioc), m_acceptor(m_ioc)
	{
		beast::error_code ec;
		m_acceptor.open(endpoint.protocol(), ec);
		if (ec)
		{
			fail(ec, "open");
			return;
		}
		m_acceptor.set_option(asio::socket_base::reuse_address(true), ec);
		if (ec)
		{
			fail(ec, "set_option");
			return;
		}
		m_acceptor.bind(endpoint, ec);
		if (ec)
		{
			fail(ec, "bind");
			return;
		}
		m_acceptor.listen(asio::socket_base::max_connections, ec);
		if (ec)
		{
			fail(ec, "listen");
			return;
		}
	}

	void run()
	{
		do_accept();
	}

private:
	void do_accept()
	{
		m_acceptor.async_accept(asio::make_strand(m_ioc), beast::bind_front_handler(&Listener::on_accept, shared_from_this()));
	}

	void on_accept(beast::error_code ec, tcp::socket socket)
	{
		if (ec)
			fail(ec, "accept");
		else
			std::make_shared<Session>(std::move(socket))->run();
		do_accept();
	}
};

int main(int argc, char const *argv[])
{
	try
	{
		auto const address = ip::make_address("127.0.0.1");
		const uint16_t port = 6688;

		asio::io_context ioc{1};

		std::make_shared<Listener>(ioc, tcp::endpoint{address, port})->run();

		ioc.run();
	}
	catch (const std::exception &e)
	{
		std::cerr << e.what() << '\n';
	}

	return 0;
}
