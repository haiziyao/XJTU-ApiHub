use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("{0}")]
    BadRequest(&'static str),
    #[error("unauthorized")]
    Unauthorized,
    #[error("{0}")]
    Conflict(&'static str),
    #[error("{0}")]
    NotFound(&'static str),
    #[error("{0}")]
    Upstream(&'static str),
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl ApiError {
    pub fn status(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Upstream(_) | Self::Database(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    pub fn into_test_response(self) -> Response {
        let status = self.status();
        tracing::warn!(%status, error = %self, "request failed");
        (
            status,
            Json(json!({
                "code": status.as_u16(),
                "data": null,
                "serve_raw": null,
                "serve_data": null
            })),
        )
            .into_response()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status();
        tracing::warn!(%status, error = %self, "request failed");
        (status, Json(json!({"code": status.as_u16(), "data": null}))).into_response()
    }
}
