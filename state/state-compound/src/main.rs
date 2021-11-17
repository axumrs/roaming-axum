use axum::{extract::Extension, routing, AddExtensionLayer, Router};

use std::sync::Arc;

#[derive(Clone)]
pub struct DatabaseClient {
    pub dsn: String,
}
pub struct RedisClient {
    pub host: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseClient,
    pub rdb: Arc<RedisClient>,
}

async fn status(Extension(state): Extension<AppState>) -> String {
    format!(
        "database dsn: {}, redis host: {}",
        state.db.dsn, state.rdb.host
    )
}
#[tokio::main]
async fn main() {
    let db_client = DatabaseClient {
        dsn: "host=pg.axum.rs port=5432 user=axum_rs password=axum.rs sslmode=disable".to_string(),
    };
    let redis_client = Arc::new(RedisClient {
        host: "redis.axum.rs".to_string(),
    });

    let app = Router::new()
        .route("/status", routing::get(status))
        .layer(AddExtensionLayer::new(AppState {
            db: db_client,
            rdb: redis_client,
        }));
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
