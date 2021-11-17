use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

async fn foo() -> &'static str {
    "Welcome to axum.rs"
}

async fn bar() -> &'static str {
    "Powered by axum.rs"
}

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tower_http=debug,middleware=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/foo", get(foo))
        .route("/bar", get(bar))
        .layer(TraceLayer::new_for_http());
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
