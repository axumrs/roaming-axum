use axum::{extract::Extension, routing, AddExtensionLayer, Router};
use dotenv::dotenv;
use serde::Deserialize;

/// Web配置
#[derive(Deserialize)]
pub struct WebConfig {
    /// Web服务监听地址
    pub addr: String,
}

/// Redis 配置
#[derive(Deserialize)]
pub struct RedisConfig {
    /// 连接字符串
    pub dsn: String,
}

/// 项目配置
#[derive(Deserialize)]
pub struct Config {
    pub web: WebConfig,
    pub redis: RedisConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    /// 从环境变量中初始化配置
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        // 尝试合并环境变量设置
        cfg.merge(config::Environment::new())?;
        // 转换成我们自己的Config对象
        cfg.try_into()
    }
}

#[derive(Clone)]
pub struct AppState {
    pub pool: deadpool_postgres::Pool,
    pub rdc: redis::Client,
}

/// 尝试获取 Postgres Client
async fn try_pg(Extension(state): Extension<AppState>) -> Result<&'static str, String> {
    let _client: deadpool_postgres::Client =
        state.pool.get().await.map_err(|err| err.to_string())?;
    Ok("Successfully got database client from postgresql pool in AppState")
}

/// 尝试获取 Redis 异步连接
async fn try_redis(Extension(state): Extension<AppState>) -> Result<&'static str, String> {
    let _conn = state
        .rdc
        .get_async_connection()
        .await
        .map_err(|err| err.to_string())?;
    Ok("Successfully got async connection via redis client in AppState")
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // 解析 .env 文件
    let cfg = Config::from_env().expect("初始化项目配置失败");

    let pool = cfg
        .pg
        .create_pool(tokio_postgres::NoTls)
        .expect("创建Postgres连接池失败");
    let rdc = redis::Client::open(cfg.redis.dsn).expect("创建redis连接失败");

    let app = Router::new()
        .route("/pg", routing::get(try_pg))
        .route("/rds", routing::get(try_redis))
        .layer(AddExtensionLayer::new(AppState { pool, rdc }));

    // 绑定到配置文件设置的地址
    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
