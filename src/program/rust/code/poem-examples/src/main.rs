use poem::{get, handler, listener::TcpListener, web::Data, EndpointExt, Route, Server};
use std::sync::{Mutex, Arc};

#[derive(Debug, Clone)]
struct AppState {
    health_check_response: String,
    visit_count: Arc<Mutex<u32>>,
}

#[handler]
async fn health_check(Data(app_state): Data<&AppState>) -> String {
    let mut count = app_state.visit_count.lock().unwrap();
    let s = format!("{} {} times", app_state.health_check_response, count);

    *count += 1;

    s
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let shared_data = AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Arc::new(Mutex::new(0)),
    };

    let app = Route::new()
        .at("/health", get(health_check))
        .data(shared_data);

    Server::new(TcpListener::bind("localhost:3000"))
        .name("teacher-service")
        .run(app)
        .await
}
