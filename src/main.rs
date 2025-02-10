use axum::{Router, routing::get};
use tokio::net::TcpListener;

mod parse;
mod pages;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/{path}", get(pages::adaptive_page));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
