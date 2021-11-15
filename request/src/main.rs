use axum::{extract::Path, routing::get, Router};
use serde::Deserialize;

/// 获取单个 Path 参数
async fn user_info(Path(id): Path<i32>) -> String {
    format!("User info for {}", id)
}

/// 用元组获取多个 Path 参数
async fn repo_info(Path((user_name, repo_name)): Path<(String, String)>) -> String {
    format!(
        "Repository: user name: {} and repository name: {}",
        user_name, repo_name
    )
}

#[derive(Deserialize)]
pub struct RepoInfo {
    pub user_name: String,
    pub repo_name: String,
}

/// 将 Path 参数填充到结构体
async fn repo_info_struct(Path(info): Path<RepoInfo>) -> String {
    format!(
        "Repository: user name: {} and repository name: {}",
        info.user_name, info.repo_name
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/user/:id", get(user_info))
        .route("/repo/:user/:repo", get(repo_info))
        .route("/repo_struct/:user_name/:repo_name", get(repo_info_struct));
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
