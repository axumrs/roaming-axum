use askama::Template;
use axum::{response::Html, routing, Router};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub name: String,
}

async fn index() -> Result<Html<String>, String> {
    let name = String::from("axum中文网");
    let tpl = IndexTemplate { name };
    let html = tpl.render().map_err(|err| err.to_string())?;
    Ok(Html(html))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", routing::get(index));

    // 绑定到配置文件设置的地址
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
