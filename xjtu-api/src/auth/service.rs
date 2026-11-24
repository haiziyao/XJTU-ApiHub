use chrono::{Duration, Utc};
use rand::RngCore;
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::common::AppState;
use crate::error::ApiError;

use super::campus::{self, CampusProfile};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LookupRequest {
    pub student_no: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub student_no: String,
    pub campus_credential: String,
    pub privacy_accepted: bool,
    pub nickname: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub student_no: String,
    pub campus_credential: String,
}

pub struct RegisterResult {
    pub student_no: String,
    pub nickname: String,
    pub serve_raw: Value,
    pub serve_data: Value,
}

pub struct LoginResult {
    pub access_token: String,
    pub student_no: String,
    pub nickname: String,
    pub serve_raw: Value,
    pub serve_data: Value,
}

pub async fn lookup(state: &AppState, student_no: &str) -> Result<Option<String>, ApiError> {
    let student_no = validate_student_no(student_no)?;
    let nickname = sqlx::query_scalar::<_, String>(
        "SELECT nick_name FROM `user` WHERE stu_id = ? AND is_deleted = 0",
    )
    .bind(student_no)
    .fetch_optional(&state.pool)
    .await?;
    Ok(nickname)
}

pub async fn register(state: &AppState, input: RegisterRequest) -> Result<RegisterResult, ApiError> {
    let student_no = validate_student_no(&input.student_no)?;
    if !input.privacy_accepted {
        return Err(ApiError::BadRequest("privacy acceptance required"));
    }
    if input.campus_credential.trim().is_empty() {
        return Err(ApiError::BadRequest("campusCredential is required"));
    }

    let campus = campus::fetch(
        &state.http,
        &state.config.campus_profile_url,
        input.campus_credential.trim(),
    )
    .await?;
    if campus.student_no != student_no {
        return Err(ApiError::Conflict("campus student number mismatch"));
    }

    let profile = ProfileFields::from_campus(&campus);
    let nickname = match input
        .nickname
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        Some(value) => validate_nickname(value)?,
        None => {
            let real_name = campus.name.trim();
            if real_name.is_empty() {
                return Err(ApiError::BadRequest("nickname is required"));
            }
            validate_nickname(real_name)?
        }
    };

    let existing: Option<(String, i8)> =
        sqlx::query_as("SELECT nick_name, is_deleted FROM `user` WHERE stu_id = ?")
            .bind(student_no)
            .fetch_optional(&state.pool)
            .await?;
    if existing.is_some() {
        return Err(ApiError::Conflict("account already registered"));
    }

    let now = Utc::now().naive_utc();
    let inserted = sqlx::query(
        "INSERT IGNORE INTO `user` (stu_id, nick_name, real_name, grade_name, \
         province_code, gender, is_deleted, degree_type, enrollment_year, \
         privacy_agreed_at, last_login_at, created_at, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?, 0, ?, ?, ?, NULL, ?, ?)",
    )
    .bind(student_no)
    .bind(&nickname)
    .bind(profile.real_name.as_deref())
    .bind(profile.grade_name.as_deref())
    .bind(profile.province_code.as_deref())
    .bind(profile.gender)
    .bind(profile.degree_type)
    .bind(profile.enrollment_year)
    .bind(now)
    .bind(now)
    .bind(now)
    .execute(&state.pool)
    .await?;
    if inserted.rows_affected() == 0 {
        return Err(ApiError::Conflict("account already registered"));
    }

    let serve_data = json!({
        "studentNo": student_no,
        "name": &campus.name,
        "className": &campus.class_name,
        "grade": &campus.grade,
        "nickname": &nickname,
        "gender": profile.gender,
        "provinceCode": profile.province_code,
        "degreeType": profile.degree_type,
        "enrollmentYear": profile.enrollment_year,
    });

    Ok(RegisterResult {
        student_no: student_no.to_string(),
        nickname,
        serve_raw: campus.raw,
        serve_data,
    })
}

pub async fn login(state: &AppState, input: LoginRequest) -> Result<LoginResult, ApiError> {
    let student_no = validate_student_no(&input.student_no)?;
    if input.campus_credential.trim().is_empty() {
        return Err(ApiError::BadRequest("campusCredential is required"));
    }

    let campus = campus::fetch(
        &state.http,
        &state.config.campus_profile_url,
        input.campus_credential.trim(),
    )
    .await?;
    if campus.student_no != student_no {
        return Err(ApiError::Conflict("campus student number mismatch"));
    }

    let nickname: Option<String> =
        sqlx::query_scalar("SELECT nick_name FROM `user` WHERE stu_id = ? AND is_deleted = 0")
            .bind(student_no)
            .fetch_optional(&state.pool)
            .await?;
    let nickname = nickname.ok_or(ApiError::NotFound("account not registered"))?;

    let profile = ProfileFields::from_campus(&campus);
    let now = Utc::now().naive_utc();
    sqlx::query(
        "UPDATE `user` SET real_name = ?, grade_name = ?, province_code = ?, \
         gender = ?, degree_type = ?, enrollment_year = ?, last_login_at = ?, \
         updated_at = ? WHERE stu_id = ?",
    )
    .bind(profile.real_name.as_deref())
    .bind(profile.grade_name.as_deref())
    .bind(profile.province_code.as_deref())
    .bind(profile.gender)
    .bind(profile.degree_type)
    .bind(profile.enrollment_year)
    .bind(now)
    .bind(now)
    .bind(student_no)
    .execute(&state.pool)
    .await?;

    let access_token = generate_token();
    let token_hash = sha256_hex(&access_token);
    let expires_at = now + Duration::days(state.config.token_ttl_days);
    sqlx::query(
        "INSERT INTO `user_token` (token_hash, stu_id, created_at, expires_at) \
         VALUES (?, ?, ?, ?)",
    )
    .bind(&token_hash)
    .bind(student_no)
    .bind(now)
    .bind(expires_at)
    .execute(&state.pool)
    .await?;

    let serve_data = json!({
        "studentNo": student_no,
        "nickname": &nickname,
    });

    Ok(LoginResult {
        access_token,
        student_no: student_no.to_string(),
        nickname,
        serve_raw: campus.raw,
        serve_data,
    })
}

pub async fn authenticate(state: &AppState, token: &str) -> Result<(String, String), ApiError> {
    if token.len() != 64 || !token.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(ApiError::Unauthorized);
    }
    let token_hash = sha256_hex(token);
    let now = Utc::now().naive_utc();
    let row: Option<(String, String)> = sqlx::query_as(
        "SELECT u.stu_id, u.nick_name FROM `user_token` t \
         JOIN `user` u ON u.stu_id = t.stu_id \
         WHERE t.token_hash = ? AND t.revoked_at IS NULL AND t.expires_at > ? \
         AND u.is_deleted = 0",
    )
    .bind(&token_hash)
    .bind(now)
    .fetch_optional(&state.pool)
    .await?;
    let row = row.ok_or(ApiError::Unauthorized)?;
    let _ = sqlx::query("UPDATE `user_token` SET last_used_at = ? WHERE token_hash = ?")
        .bind(now)
        .bind(&token_hash)
        .execute(&state.pool)
        .await;
    Ok(row)
}

struct ProfileFields {
    real_name: Option<String>,
    grade_name: Option<String>,
    province_code: Option<String>,
    gender: Option<i8>,
    degree_type: Option<i8>,
    enrollment_year: Option<i16>,
}

impl ProfileFields {
    fn from_campus(campus: &CampusProfile) -> Self {
        let (degree_type, enrollment_year, province_code) = derive_fields(&campus.student_no);
        let gender = match campus.sex.as_deref() {
            Some("1") => Some(1i8),
            Some("2") => Some(2i8),
            _ => None,
        };
        Self {
            real_name: non_empty(&campus.name),
            grade_name: non_empty(&campus.class_name).or_else(|| non_empty(&campus.grade)),
            province_code,
            gender,
            degree_type,
            enrollment_year,
        }
    }
}

fn validate_student_no(value: &str) -> Result<&str, ApiError> {
    let value = value.trim();
    if value.is_empty() || value.len() > 16 || value.chars().any(char::is_whitespace) {
        return Err(ApiError::BadRequest("invalid studentNo"));
    }
    Ok(value)
}

fn validate_nickname(value: &str) -> Result<String, ApiError> {
    let value = value.trim();
    if value.is_empty() || value.chars().count() > 32 {
        return Err(ApiError::BadRequest("invalid nickname"));
    }
    Ok(value.to_string())
}

fn non_empty(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn derive_fields(stu_id: &str) -> (Option<i8>, Option<i16>, Option<String>) {
    if !stu_id.bytes().all(|byte| byte.is_ascii_digit()) {
        return (None, None, None);
    }
    let degree_type = stu_id
        .chars()
        .next()
        .and_then(|value| value.to_digit(10))
        .and_then(|value| match value {
            2 => Some(2i8),
            3 => Some(3i8),
            _ => None,
        });
    let enrollment_year = stu_id
        .get(1..3)
        .and_then(|value| value.parse::<i16>().ok())
        .map(|year| 2000 + year);
    let province_code = stu_id.get(3..5).map(ToOwned::to_owned);
    (degree_type, enrollment_year, province_code)
}

fn generate_token() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}

fn sha256_hex(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    hex::encode(hasher.finalize())
}
