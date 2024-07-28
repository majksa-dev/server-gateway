pub mod basic;
pub mod endpoint;
pub mod jwt;

use super::quota::Quota;
use gateway::cors;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Auth {
    pub token: String,
    pub quota: Option<Quota>,
}

impl From<&Auth> for cors::config::Auth {
    fn from(value: &Auth) -> Self {
        Self::new(value.token.clone(), None)
    }
}
