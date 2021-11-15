use axum::http::header::HeaderName;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::Html;
use axum::routing::get;
use axum::{Json, Router};
use errors::AppError;
use serde::Serialize;

pub mod errors;

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

/// Result 响应
async fn result() -> Result<&'static str, StatusCode> {
    let flag = false;
    if flag {
        Ok("Hello, axum.rs")
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[derive(Serialize)]
struct Info {
    web_site: String,
    email: String,
    level: i32,
}

/// 自定义结构体响应
async fn info_struct() -> Json<Info> {
    let info = Info {
        web_site: "https://axum.rs".to_string(),
        email: "team@axum.rs".to_string(),
        level: 123,
    };
    Json(info)
}

/// 响应自定义错误
async fn app_error() -> Result<&'static str, AppError> {
    let flag = false;
    if flag {
        Ok("Hello, axum.rs")
    } else {
        Err(AppError {
            message: "Opps!".to_string(),
        })
    }
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
        .route("/json", get(json))
        .route("/result", get(result))
        .route("/info_struct", get(info_struct))
        .route("/app_error", get(app_error));
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
