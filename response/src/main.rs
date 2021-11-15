use axum::http::header::HeaderName;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::routing::get;
use axum::Router;

/// 纯文本之 &str
async fn str_response() -> &'static str {
    "Hello, axum.rs"
}

/// 纯文本之 String
async fn string_response() -> String {
    "Hello, axum.rs".to_string()
}

/// 仅有状态码
async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}

/// 带响应头的响应
async fn with_headers() -> (HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-powered"),
        HeaderValue::from_static("axum.rs"),
    );
    (headers, "axum.rs")
}

/// 带响应头和状态码的响应
async fn with_headers_and_status() -> (StatusCode, HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-powered"),
        HeaderValue::from_static("axum.rs"),
    );
    (StatusCode::OK, headers, "axum.rs")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/str", get(str_response))
        .route("/string", get(string_response))
        .route("/404", get(not_found))
        .route("/with_headers", get(with_headers))
        .route("/with_headers_and_status", get(with_headers_and_status));
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
