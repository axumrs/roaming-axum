use axum::{routing, Router};

async fn axum_rs_txt() -> String {
    std::fs::read_to_string("static/axum-rs.txt").unwrap()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/static/axum-rs.txt", routing::get(axum_rs_txt));
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
