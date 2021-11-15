use axum::routing::get;
use axum::Router;

async fn str_response() -> &'static str {
    "Hello, axum.rs"
}
async fn string_response() -> String {
    "Hello, axum.rs".to_string()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/str", get(str_response))
        .route("/string", get(string_response));
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
