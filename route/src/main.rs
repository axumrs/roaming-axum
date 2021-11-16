use axum::{
    extract::{Form, Path},
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
#[tokio::main]
async fn main() {
    let app = Router::new().route("/edit_user/:id", get(edit_user).post(edit_user_action));
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
