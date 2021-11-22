use std::fmt::Display;

use axum::{extract::TypedHeader, routing, Json, Router};
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

// 快速说明
//
// - 获取一个授权令牌:
//
// curl -X POST -H 'content-type:application/json' -d '{"client_id":"axum.rs","client_secret":"team@axum.rs"}' 127.0.0.1:9527/authorize
//
// - 使用授权令牌访问被保护的内容
//
// curl -H 'content-type:application/json' -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ0ZWFtQGF4dW0ucnMiLCJjb21wYW55IjoiQVhVTS5SUyIsImV4cCI6MTAwMDAwMDAwMDB9.2jPYCuK6_nDrFdXS3HLAm43YvbFvrBBLYS6YkZ_z6zM' 127.0.0.1:9527/protected
//
// - 尝试使用非法的令牌使用授权令牌访问被保护的内容
//
// curl -H 'content-type:application/json' -H 'Authorization: Bearer foobar' 127.0.0.1:9527/protected

const JWT_SECRET: &str = "https://AXUM.RS";

pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey<'static>,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret).into_static(),
        }
    }
    pub fn global() -> Self {
        Self::new(JWT_SECRET.as_bytes())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

#[derive(Serialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: String::from("Bearer"),
        }
    }
}

#[derive(Deserialize)]
pub struct AuthPayload {
    pub client_id: String,
    pub client_secret: String,
}

async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, Json<String>> {
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(Json(String::from("Missing Credentials")));
    }

    if payload.client_id != "axum.rs" || payload.client_secret != "team@axum.rs" {
        return Err(Json(String::from("Wrong Credentials")));
    }

    let claims = Claims {
        sub: "team@axum.rs".to_string(),
        company: "AXUM.RS".to_string(),
        exp: 10000000000,
    };

    let token = encode(&Header::default(), &claims, &Keys::global().encoding)
        .map_err(|err| Json(err.to_string()))?;

    Ok(Json(AuthBody::new(token)))
}
async fn protected(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<String, String> {
    let token_data = decode::<Claims>(
        bearer.token(),
        &Keys::global().decoding,
        &Validation::default(),
    )
    .map_err(|err| format!("Invalid Token: {}", err.to_string()))?;
    let claims = token_data.claims;
    Ok(format!(
        "Welcome, your email {}, company: {}",
        claims.sub, claims.company
    ))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/authorize", routing::post(authorize))
        .route("/protected", routing::get(protected));

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
