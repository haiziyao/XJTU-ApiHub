use std::time::Duration;

use serde_json::Value;

use crate::error::ApiError;

pub struct CampusProfile {
    pub student_no: String,
    pub name: String,
    pub class_name: String,
    pub grade: String,
    pub sex: Option<String>,
    pub raw: Value,
}

pub async fn fetch(
    client: &reqwest::Client,
    url: &str,
    token: &str,
) -> Result<CampusProfile, ApiError> {
    let response = client
        .get(url)
        .header("Referer", "http://hello.xjtu.edu.cn/yingxin-pc/")
        .header("access-token", token)
        .header("access_token", token)
        .header("systemtype", "yingxin_student_pc")
        .header("synAccessSource", "pc")
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
             (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        )
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .map_err(|error| {
            tracing::warn!(%error, "campus profile request failed");
            ApiError::Upstream("campus profile request failed")
        })?;

    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|_| ApiError::Upstream("campus response read failed"))?;
    let raw: Value = serde_json::from_str(&body)
        .map_err(|_| ApiError::Upstream("campus returned non-JSON"))?;

    if !status.is_success() || !state_ok(&raw) {
        return Err(ApiError::Unauthorized);
    }

    let student = raw.pointer("/data/studentBean").and_then(Value::as_object);
    let field = |name: &str| {
        student
            .and_then(|student| student.get(name))
            .map(text)
            .unwrap_or_default()
    };

    let sno = field("sno");
    let student_no = if sno.is_empty() {
        field("account")
    } else {
        sno
    };
    if student_no.is_empty() {
        return Err(ApiError::Unauthorized);
    }

    let sex = field("sex");
    Ok(CampusProfile {
        student_no,
        name: field("name"),
        class_name: field("className"),
        grade: field("grade"),
        sex: if sex.is_empty() { None } else { Some(sex) },
        raw,
    })
}

fn state_ok(raw: &Value) -> bool {
    match raw.get("state") {
        Some(Value::Number(number)) => number.as_u64() == Some(200),
        Some(Value::String(value)) => value == "200",
        _ => false,
    }
}

fn text(value: &Value) -> String {
    match value {
        Value::String(value) => value.trim().to_string(),
        Value::Number(value) => value.to_string(),
        _ => String::new(),
    }
}
