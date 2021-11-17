use axum::{
    extract::Form,
    http::{HeaderMap, StatusCode},
    response::Html,
    routing, Router,
};
use serde::Deserialize;

const COOKIE_NAME: &'static str = "username";

#[derive(Deserialize)]
pub struct UserLoginForm {
    pub username: String,
    pub password: String,
}

/// 用户中心首页
async fn user_center(headers: HeaderMap) -> Result<Html<String>, &'static str> {
    let cookies = headers
        .get(axum::http::header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
        .unwrap_or("".to_string()); // 从请求头获取所有COOKIE
    if cookies.is_empty() {
        return Err("NO COOKIE SETTED"); // 没有 Cookie
    }
    let mut logined_username: Option<String> = None;
    let cookies: Vec<&str> = cookies.split(';').collect(); // 多个cookie用;分割
    for cookie in cookies {
        let cookie_pair: Vec<&str> = cookie.split('=').collect(); // 每个cookie都是用=分割的键值对
        let cookie_name = cookie_pair[0].trim();
        let cookie_value = cookie_pair[1].trim();
        // 如果 cookie 的名称是我们希望的，并且值不为空
        if cookie_name == COOKIE_NAME && !cookie_value.is_empty() {
            logined_username = Some(String::from(cookie_value)); // 设置已登录用户的用户名
            break;
        }
    }
    if logined_username.is_none() {
        return Err("COOKIE IS EMPTY"); // 没有我们需要的cookie
    }
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="zh-Hans">
          <head>
            <meta charset="utf-8" />
            <meta name="author" content="axum.rs (team@axum.rs)" />
            <title>
              用户中心-AXUM中文网
            </title>
          </head>
          <body>
          <p>你好，<strong>{}</strong>！你已成功登录。[<a href="/logout">退出登录</a>]
          </body>
          </html>
        "#,
        logined_username.unwrap()
    );
    Ok(Html(html))
}
/// 用户登录表单
async fn user_login() -> Html<String> {
    let html = r#"
        <!DOCTYPE html>
        <html lang="zh-Hans">
          <head>
            <meta charset="utf-8" />
            <meta name="author" content="axum.rs (team@axum.rs)" />
            <title>
              用户登录-AXUM中文网
            </title>
          </head>
          <body>
          <form method="post" action="/login">
          <div>
            <label>用户名</label>
            <input type="text" name="username">
          </div>
          <div>
            <label>密码</label>
            <input type="password" name="password">
          </div>
          <div>
            <button type="submit">提交</button>
          </div>
          </form>
          </body>
          </html>
        "#
    .to_string();

    Html(html)
}
/// 用户登录
async fn user_login_action(Form(frm): Form<UserLoginForm>) -> (StatusCode, HeaderMap, ()) {
    let mut headers = HeaderMap::new();
    if !(&frm.username == "axum.rs" && &frm.password == "axum.rs") {
        headers.insert(
            axum::http::header::LOCATION,
            "/login?msg=用户名或密码错误".parse().unwrap(),
        ); // 跳转到登录页面
    } else {
        let cookie = format!("{}={}", COOKIE_NAME, frm.username);
        headers.insert(
            axum::http::header::SET_COOKIE,
            cookie.as_str().parse().unwrap(),
        ); // 设置Cookie
        headers.insert(axum::http::header::LOCATION, "/".parse().unwrap()); // 跳转到用户中心首页
    }
    (StatusCode::FOUND, headers, ())
}
/// 退出登录
async fn user_logout() -> (StatusCode, HeaderMap, ()) {
    let cookie = format!("{}=", COOKIE_NAME);
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::SET_COOKIE,
        cookie.as_str().parse().unwrap(),
    ); // 清空Cookie
    headers.insert(axum::http::header::LOCATION, "/login".parse().unwrap()); // 跳转到登录页面
    (StatusCode::FOUND, headers, ())
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", routing::get(user_center))
        .route("/login", routing::get(user_login).post(user_login_action))
        .route("/logout", routing::get(user_logout));
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
