use axum::http::header::HeaderName;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::Html;
use axum::routing::get;
use axum::{Json, Router};

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

/// HTML 响应
async fn html() -> Html<&'static str> {
    Html("Hello, <em>axum.rs</em>!")
}

/// JSON 响应
async fn json() -> Json<serde_json::Value> {
    Json(serde_json::json!({"axum.rs":"axum中文网"}))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/str", get(str_response))
        .route("/string", get(string_response))
        .route("/404", get(not_found))
        .route("/with_headers", get(with_headers))
        .route("/with_headers_and_status", get(with_headers_and_status))
        .route("/html", get(html))
        .route("/json", get(json));
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
