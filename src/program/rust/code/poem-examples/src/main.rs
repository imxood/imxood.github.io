use poem::{get, handler, listener::TcpListener, web::Path, Route, Server};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello, {}", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "poem=debug");
    tracing_subscriber::fmt::init();

    let app = Route::new().at("/hello/:name", get(hello));

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .name("hello-world")
        .run(app)
        .await
}

#[tokio::test]
async fn test_hello() {
    use poem::test::TestClient;

    let name = "xiaoming";

    let app = Route::new().at("/hello/:name", get(hello));
    let cli = TestClient::new(app);

    let resp = cli.get("/hello/xiaoming").send().await;
    resp.assert_status_is_ok();

    resp.assert_text(format!("hello, {}", name)).await;
}
