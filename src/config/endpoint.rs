use serde::Deserialize;

use super::{auth::Auth, cache::Cache, method::Method, quota::Quota};

#[derive(Debug, Clone, Deserialize)]
pub struct Endpoint {
    pub path: String,
    pub id: String,
    pub quota: Option<Quota>,
    pub cache: Option<Cache>,
    pub method: Method,
    pub auth: Option<Vec<Auth>>,
}
