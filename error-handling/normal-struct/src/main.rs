use axum::{routing::post, Router};

mod extract;
mod handler;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/login", post(handler::login));
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
