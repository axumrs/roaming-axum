use axum::{
    extract::{Form, Path},
    http::{HeaderMap, StatusCode},
    response::Html,
    routing::get,
    Router,
};
use serde::Deserialize;

/// 通过表单提交数据
#[derive(Deserialize)]
pub struct EditUser {
    pub id: i32,
    pub username: String,
    pub email: String,
}

/// 对应数据库的模型
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub email: String,
}

/// 显示要修改的用户
async fn edit_user(Path(id): Path<i32>) -> Html<String> {
    let model = UserModel {
        id,
        username: "AXUM.RS".to_string(),
        email: "team@axum.rs".to_string(),
    };
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="zh-Hans">
          <head>
            <meta charset="utf-8" />
            <meta name="author" content="axum.rs (team@axum.rs)" />
            <title>
              修改用户-AXUM中文网
            </title>
          </head>
          <body>
          <form method="post" action="/edit_user/{}">
          <input type="hidden" name="id" value="{}">
          <div>
            <label>用户名</label>
            <input type="text" name="username" value="{}">
          </div>
          <div>
            <label>Email</label>
            <input type="email" name="email" value="{}">
          </div>
          <div>
            <button type="submit">提交</button>
          </div>
          </form>
          </body>
          </html>
        "#,
        model.id, model.id, model.username, model.email
    );
    Html(html)
}

/// 对用户进行修改
async fn edit_user_action(Form(frm): Form<EditUser>) -> Html<String> {
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="zh-Hans">
          <head>
            <meta charset="utf-8" />
            <meta name="author" content="axum.rs (team@axum.rs)" />
            <title>
              修改用户-AXUM中文网
            </title>
          </head>
          <body>
            <h1>修改成功！</h1>
            <p>修改后的用户资料：</p>
            <div>ID: {} </div>
            <div>用户名: {} </div>
            <div>Email: {} </div>
          </body>
          </html>"#,
        frm.id, frm.username, frm.email
    );
    Html(html)
}

/// 新闻首页
/// 访问路径：/news
async fn news_index() -> &'static str {
    "new index"
}

/// 新闻详情
/// 访问路径：/news/detail/123
async fn news_detail(Path(id): Path<i32>) -> String {
    format!("new detail {}", id)
}

/// 新闻评论
/// 访问路径：/news/comments/456
async fn news_comments(Path(id): Path<i32>) -> String {
    format!("new comments {}", id)
}

/// 重定向
async fn redirect() -> (StatusCode, HeaderMap, ()) {
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::LOCATION,
        "https://axum.rs".parse().unwrap(),
    );
    (StatusCode::FOUND, headers, ())
}

#[tokio::main]
async fn main() {
    // 新闻的子路由
    let news_router = Router::new()
        .route("/", get(news_index))
        .route("/detail/:id", get(news_detail))
        .route("/comments/:id", get(news_comments));

    let app = Router::new()
        .route("/edit_user/:id", get(edit_user).post(edit_user_action)) // 链式路由
        .nest("/news", news_router) // 新闻的嵌套路由
        .route("/go", get(redirect)); // 重定向
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
