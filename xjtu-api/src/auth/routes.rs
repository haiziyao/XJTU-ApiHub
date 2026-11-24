use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json, Value};

use crate::common::{response, AppState};
use crate::error::ApiError;

use super::service::{self, LoginRequest, LookupRequest, RegisterRequest};

pub async fn lookup(
    State(state): State<AppState>,
    Json(input): Json<LookupRequest>,
) -> Result<Json<Value>, ApiError> {
    let nickname = service::lookup(&state, &input.student_no).await?;
    Ok(response::ok(json!({ "nickname": nickname })))
}

pub async fn lookup_test(
    State(state): State<AppState>,
    Json(input): Json<LookupRequest>,
) -> Response {
    match service::lookup(&state, &input.student_no).await {
        Ok(nickname) => {
            response::ok_test(json!({ "nickname": nickname }), Value::Null, Value::Null)
                .into_response()
        }
        Err(error) => error.into_test_response(),
    }
}

pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterRequest>,
) -> Result<Json<Value>, ApiError> {
    let result = service::register(&state, input).await?;
    Ok(response::ok(json!({
        "studentNo": result.student_no,
        "nickname": result.nickname
    })))
}

pub async fn register_test(
    State(state): State<AppState>,
    Json(input): Json<RegisterRequest>,
) -> Response {
    match service::register(&state, input).await {
        Ok(result) => response::ok_test(
            json!({
                "studentNo": result.student_no,
                "nickname": result.nickname
            }),
            result.serve_raw,
            result.serve_data,
        )
        .into_response(),
        Err(error) => error.into_test_response(),
    }
}

pub async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginRequest>,
) -> Result<Json<Value>, ApiError> {
    let result = service::login(&state, input).await?;
    Ok(response::ok(json!({
        "accessToken": result.access_token,
        "studentNo": result.student_no,
        "nickname": result.nickname
    })))
}

pub async fn login_test(
    State(state): State<AppState>,
    Json(input): Json<LoginRequest>,
) -> Response {
    match service::login(&state, input).await {
        Ok(result) => response::ok_test(
            json!({
                "accessToken": result.access_token,
                "studentNo": result.student_no,
                "nickname": result.nickname
            }),
            result.serve_raw,
            result.serve_data,
        )
        .into_response(),
        Err(error) => error.into_test_response(),
    }
}

pub async fn me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, ApiError> {
    let token = bearer_token(&headers)?;
    let (student_no, nickname) = service::authenticate(&state, token).await?;
    Ok(response::ok(
        json!({ "studentNo": student_no, "nickname": nickname }),
    ))
}

fn bearer_token(headers: &HeaderMap) -> Result<&str, ApiError> {
    let value = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(ApiError::Unauthorized)?;
    let token = value
        .strip_prefix("Bearer ")
        .ok_or(ApiError::Unauthorized)?;
    if token.len() != 64 || !token.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(ApiError::Unauthorized);
    }
    Ok(token)
}
