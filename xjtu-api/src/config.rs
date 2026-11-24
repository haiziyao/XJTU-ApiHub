use std::env;

use anyhow::Context;

#[derive(Clone, Debug)]
pub struct Config {
    pub bind_addr: String,
    pub database_url: String,
    pub campus_profile_url: String,
    pub token_ttl_days: i64,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            bind_addr: env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
            database_url: env::var("DATABASE_URL").context("DATABASE_URL is required")?,
            campus_profile_url: env::var("CAMPUS_PROFILE_URL").unwrap_or_else(|_| {
                "http://hello.xjtu.edu.cn/yingxin/user/afterLogin?synAccessSource=pc".to_string()
            }),
            token_ttl_days: env::var("TOKEN_TTL_DAYS")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(90),
        })
    }
}
