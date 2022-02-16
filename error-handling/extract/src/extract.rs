use std::borrow::Cow;

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, RequestParts},
    http::StatusCode,
    BoxError,
};
use serde::de::DeserializeOwned;
use serde_json::json;
pub struct Json<T>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for Json<T>
where
    B: axum::body::HttpBody + Send,
    T: DeserializeOwned,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req).await {
            Ok(value) => Ok(Self(value.0)),
            Err(err) => {
                let body: Cow<'_, str> = match err {
                    JsonRejection::InvalidJsonBody(err) => {
                        format!("缺少所需的字段：{}", err).into()
                    }
                    JsonRejection::MissingJsonContentType(err) => {
                        format!("请使用JSON请求：{}", err).into()
                    }
                    err => format!("发生错误：{}", err).into(),
                };
                Err((
                    StatusCode::BAD_REQUEST,
                    axum::Json(json!({ "error": body })),
                ))
            }
        }
    }
}
