use axum::routing::get;
use axum::Router;

async fn str_response() -> &'static str {
    "你好，axum.rs"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/str", get(str_response));
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
