use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};
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

#[derive(Deserialize)]
pub struct SubjectArgs {
    pub page: i32,
    pub keyword: String,
}
/// 将 Query 参数填充到结构体
async fn subject(Query(args): Query<SubjectArgs>) -> String {
    format!("Page {}, keyword: {} of subjects", args.page, args.keyword)
}

/// 把 Query 参数变成Option
async fn subject_opt(args: Option<Query<SubjectArgs>>) -> String {
    if let Some(args) = args {
        let args = args.0;
        return format!("Page {}, keyword: {} of subjects", args.page, args.keyword);
    }
    "Page 0, no keyword of subjects".to_string()
}

#[derive(Deserialize)]
pub struct SubjectArgsOpt {
    pub page: Option<i32>,
    pub keyword: Option<String>,
}
/// 将结构体的字段变成Option，并将参数填充到结构体
async fn subject_opt_done(Query(args): Query<SubjectArgsOpt>) -> String {
    let page = args.page.unwrap_or(0);
    let keyword = args.keyword.unwrap_or("".to_string());

    format!("Page {}, keyword: {} of subjects", page, keyword)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/user/:id", get(user_info))
        .route("/repo/:user/:repo", get(repo_info))
        .route("/repo_struct/:user_name/:repo_name", get(repo_info_struct))
        .route("/subject", get(subject))
        .route("/subject_opt", get(subject_opt))
        .route("/subject_opt_done", get(subject_opt_done));
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
