use askama::Template;
use axum::{Router, http::HeaderMap, http::header, response::Html, routing::get};
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
}

async fn index(headers: HeaderMap) -> Html<String> {
    let host = headers
        .get(header::HOST)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();
    let template = IndexTemplate { name: host };
    Html(template.render().unwrap())
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    println!("shutting down");
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", ServeDir::new("assets"));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server running on http://0.0.0.0:8080");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
