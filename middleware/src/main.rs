use axum::{extract::extractor_middleware, routing::get, Router};
use tower_http::trace::TraceLayer;

pub mod user_agent;

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
        .layer(TraceLayer::new_for_http())
        .layer(extractor_middleware::<user_agent::UserAgentInfo>());
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
