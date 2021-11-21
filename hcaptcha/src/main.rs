use askama::Template;
use axum::{
    extract::{Extension, Form},
    response::Html,
    routing, AddExtensionLayer, Router,
};
use serde::Deserialize;

/// 验证HCaptcha子模块
mod hcaptcha_verify {
    use serde::{Deserialize, Serialize};

    /// 提交验证的请求
    #[derive(Serialize)]
    pub struct VerifyRequest {
        pub secret: String,
        pub response: String,
    }

    /// 验证结果的响应
    #[derive(Deserialize)]
    pub struct VerifyResponse {
        pub success: bool,
    }

    /// 将用户的人机验证操作结果提交到服务器验证
    pub async fn verify(response: String, secret: String) -> Result<bool, String> {
        let req = VerifyRequest { secret, response };
        let client = reqwest::Client::new();
        let res = client
            .post("https://hcaptcha.com/siteverify")
            .form(&req)
            .send()
            .await
            .map_err(|err| err.to_string())?;
        let res = res.text().await.map_err(|err| err.to_string())?;
        let res: VerifyResponse = serde_json::from_str(&res).map_err(|err| err.to_string())?;
        Ok(res.success)
    }
}
/// 你的 HCAPTCHA 的 Site Key
/// 你可以使用 10000000-ffff-ffff-ffff-000000000001 作为本地开发测试
const HCAPTCHA_SITE_KEY: &str = "10000000-ffff-ffff-ffff-000000000001";
/// 你的 HCAPTCHA 的 Secret
/// 你可以使用 0x0000000000000000000000000000000000000000 作为本地开发测试
const HCAPTCHA_SECRET_KEY: &str = "0x0000000000000000000000000000000000000000";

/// 反馈表单模板
#[derive(Template)]
#[template(path = "feed.html")]
pub struct FeedTemplate {
    pub site_key: String,
}

/// 反馈信息模板
#[derive(Template)]
#[template(path = "feed_action.html")]
pub struct FeedActionTemplate {
    pub feed: Feed,
}

/// 反馈信息模型
pub struct Feed {
    pub nickname: String,
    pub email: String,
    pub message: String,
}

/// 反馈信息表单
#[derive(Deserialize)]
pub struct SubmitFeed {
    pub nickname: String,
    pub email: String,
    pub message: String,
    pub hcaptcha_response: String,
}

/// hCaptcha 配置
#[derive(Clone)]
pub struct HCaptchaConfig {
    pub site_key: String,
    pub secret: String,
}

/// 反馈表单页面
async fn feed(Extension(cfg): Extension<HCaptchaConfig>) -> Result<Html<String>, String> {
    let tpl = FeedTemplate {
        site_key: cfg.site_key,
    };
    let html = tpl.render().map_err(|err| err.to_string())?;
    Ok(Html(html))
}

/// 反馈信息处理
async fn feed_action(
    Extension(cfg): Extension<HCaptchaConfig>,
    Form(frm): Form<SubmitFeed>,
) -> Result<Html<String>, String> {
    // 人机验证
    let result = hcaptcha_verify::verify(frm.hcaptcha_response, cfg.secret).await?;
    if !result {
        return Err("Please verify your hcaptcha".to_string());
    };

    let tpl = FeedActionTemplate {
        feed: Feed {
            nickname: frm.nickname,
            email: frm.email,
            message: frm.message,
        },
    };
    let html = tpl.render().map_err(|err| err.to_string())?;
    Ok(Html(html))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/feed", routing::get(feed).post(feed_action))
        .layer(AddExtensionLayer::new(HCaptchaConfig {
            site_key: String::from(HCAPTCHA_SITE_KEY),
            secret: String::from(HCAPTCHA_SECRET_KEY),
        }));

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
