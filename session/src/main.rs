use axum::{
    extract::{Extension, Form, Query},
    http::{HeaderMap, StatusCode},
    response::Html,
    routing, AddExtensionLayer, Router,
};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const SESSION_ID_COOKIE_NAME: &str = "axum_rs_session_id";
const SESSION_KEY_PREFIX: &str = "axum_rs_session:";
const REDIS_DSN: &str = "redis://127.0.0.1:16379/";

/// 用户Session
#[derive(Serialize, Deserialize, Debug)]
pub struct UserSession {
    pub username: String,
    pub level: u8,
}

/// 用户登录表单
#[derive(Deserialize)]
pub struct UserLoginForm {
    pub username: String,
    pub password: String,
}

/// 用户登录界面的提示信息
#[derive(Deserialize)]
pub struct LoginMessage {
    pub msg: Option<String>,
}

/// 将 Session ID 保存到 Cookie
fn save_session_id_to_cookie(session_id: &str, headers: &mut HeaderMap) {
    let cookie = format!("{}={}", SESSION_ID_COOKIE_NAME, session_id);
    headers.insert(
        axum::http::header::SET_COOKIE,
        cookie.as_str().parse().unwrap(),
    );
}

/// 从 cookie 中获取session id
fn get_session_from_cookie(headers: &HeaderMap) -> Option<String> {
    let cookies = headers
        .get(axum::http::header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    if cookies.is_empty() {
        return None;
    }
    let mut session_id: Option<String> = None;
    let cookies: Vec<&str> = cookies.split(';').collect();
    for cookie in cookies {
        let cookie_pair: Vec<&str> = cookie.split('=').collect();
        let cookie_name = cookie_pair[0].trim();
        let cookie_value = cookie_pair[1].trim();
        if cookie_name == SESSION_ID_COOKIE_NAME && !cookie_value.is_empty() {
            session_id = Some(cookie_value.to_string());
            break;
        }
    }
    session_id
}

/// 登录界面
async fn login(Query(login_msg): Query<LoginMessage>) -> Html<String> {
    let msg = match login_msg.msg {
        None => "".to_string(),
        Some(msg) => format!(r#"<div style="color:red">{}</div>"#, msg),
    };
    let html = format!(
        r#"
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
            <h1>用户登录</h1>
            {}
            <form action="/login" method="post">
                <div>
                    <label>用户名:<input type="text" name="username"></label>
                </div>
                <div>
                    <label>密码:<input type="password" name="password"></label>
                </div>
                <div><button type="submit">登录</button></div>
            </form>
          </body>
          </html>"#,
        msg
    );
    Html(html)
}

/// 登录操作
async fn logout_action(
    Extension(rdc): Extension<redis::Client>,
    Form(frm): Form<UserLoginForm>,
) -> Result<(StatusCode, HeaderMap, ()), String> {
    let mut headers = HeaderMap::new();
    let url;
    if !(&frm.username == "axum.rs" && &frm.password == "axum.rs") {
        url = "/login?msg=用户名或密码错误"
    } else {
        // 生成 session ID
        let session_id = Uuid::new_v4().to_simple().to_string();
        // 将 session ID 保存到 Cookie
        save_session_id_to_cookie(&session_id, &mut headers);

        let user_session = UserSession {
            username: frm.username,
            level: 1,
        };
        let user_session = serde_json::json!(user_session).to_string();

        // 将 session 保存到 redis
        let redis_key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
        let mut conn = rdc
            .get_async_connection()
            .await
            .map_err(|err| err.to_string())?;
        // session 将在20分钟后自动过期
        conn.set_ex(redis_key, user_session, 1200)
            .await
            .map_err(|err| err.to_string())?;
        url = "/"
    }
    headers.insert(axum::http::header::LOCATION, url.parse().unwrap());
    Ok((StatusCode::FOUND, headers, ()))
}

/// 退出登录
async fn logout(
    Extension(rdc): Extension<redis::Client>,
    headers: HeaderMap,
) -> Result<(StatusCode, HeaderMap, ()), String> {
    let session_id = get_session_from_cookie(&headers);
    let mut headers = HeaderMap::new();
    if let Some(session_id) = session_id {
        // 从 redis 删除 Session
        let redis_key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
        let mut conn = rdc
            .get_async_connection()
            .await
            .map_err(|err| err.to_string())?;
        conn.del(redis_key).await.map_err(|err| err.to_string())?;
        // 清空Cookie
        save_session_id_to_cookie(&session_id, &mut headers);
    }
    headers.insert(axum::http::header::LOCATION, "/login".parse().unwrap());
    Ok((StatusCode::FOUND, headers, ()))
}

/// 首页
async fn index(
    Extension(rdc): Extension<redis::Client>,
    headers: HeaderMap,
) -> Result<Html<String>, String> {
    let session_id = get_session_from_cookie(&headers);
    let mut session: Option<UserSession> = None;
    if let Some(session_id) = session_id {
        // 从 redis 读取 Session
        let redis_key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
        let mut conn = rdc
            .get_async_connection()
            .await
            .map_err(|err| err.to_string())?;
        let session_str: Option<String> =
            conn.get(redis_key).await.map_err(|err| err.to_string())?;
        if let Some(session_str) = session_str {
            let user_session: UserSession =
                serde_json::from_str(&session_str).map_err(|err| err.to_string())?;
            session = Some(user_session);
        }
    }
    match session {
        Some(session) => {
            let html = format!(
                r#"
        <!DOCTYPE html>
        <html lang="zh-Hans">
          <head>
            <meta charset="utf-8" />
            <meta name="author" content="axum.rs (team@axum.rs)" />
            <title>
              用户首页-AXUM中文网
            </title>
          </head>
          <body>
            <div>欢迎 {} ! 你的等级是 {}。</div>
            <div><a href="/logout">退出登录</a></div>
          </body>
          </html>"#,
                session.username, session.level
            );
            Ok(Html(html))
        }
        None => Err("Please login via /login page".to_string()),
    }
}

#[tokio::main]
async fn main() {
    let rdc = redis::Client::open(REDIS_DSN).unwrap();
    let app = Router::new()
        .route("/", routing::get(index))
        .route("/login", routing::get(login).post(logout_action))
        .route("/logout", routing::get(logout))
        .layer(AddExtensionLayer::new(rdc));

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
