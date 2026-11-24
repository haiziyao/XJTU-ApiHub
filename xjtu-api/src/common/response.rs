use axum::Json;
use serde_json::{json, Value};

pub fn ok(data: Value) -> Json<Value> {
    Json(json!({"code": 200, "data": data}))
}

pub fn ok_test(data: Value, serve_raw: Value, serve_data: Value) -> Json<Value> {
    Json(json!({
        "code": 200,
        "data": data,
        "serve_raw": serve_raw,
        "serve_data": serve_data
    }))
}
