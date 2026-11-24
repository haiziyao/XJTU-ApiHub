pub mod response;

use std::sync::Arc;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::MySqlPool,
    pub config: Arc<Config>,
    pub http: reqwest::Client,
}
