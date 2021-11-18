use axum::{routing, Router};
use redis::{AsyncCommands, Client};

const REDIS_DSN: &str = "redis://127.0.0.1:16379/";

async fn set() -> Result<&'static str, String> {
    let client = Client::open(REDIS_DSN).map_err(|err| err.to_string())?;
    let mut conn = client
        .get_async_connection()
        .await
        .map_err(|err| err.to_string())?;
    conn.set("author", "axum.rs")
        .await
        .map_err(|err| err.to_string())?;
    Ok("Successfully set")
}

async fn get() -> Result<String, String> {
    let client = Client::open(REDIS_DSN).map_err(|err| err.to_string())?;
    let mut conn = client
        .get_async_connection()
        .await
        .map_err(|err| err.to_string())?;
    let value = conn.get("author").await.map_err(|err| err.to_string())?;
    Ok(value)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/set", routing::get(set))
        .route("/get", routing::get(get));

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
