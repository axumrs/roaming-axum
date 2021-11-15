use std::convert::Infallible;

use axum::{
    body::{Bytes, Full},
    response::IntoResponse,
};

pub struct AppError {
    pub message: String,
}

impl IntoResponse for AppError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> axum::http::Response<Self::Body> {
        self.message.into_response()
    }
}
