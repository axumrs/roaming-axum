use axum::{extract::Extension, routing, AddExtensionLayer, Router};

#[derive(Clone)]
pub struct UserInfo {
    pub username: String,
}
async fn show_user_info(Extension(info): Extension<UserInfo>) -> String {
    format!("Sigined User: {}", info.username)
}
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/user", routing::get(show_user_info))
        .layer(AddExtensionLayer::new(UserInfo {
            username: "axum.rs".to_string(),
        }));
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
