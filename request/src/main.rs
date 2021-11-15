use axum::{extract::Path, routing::get, Router};

/// 获取单个 Path 参数
async fn user_info(Path(id): Path<i32>) -> String {
    format!("User info for {}", id)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/user/:id", get(user_info));
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
