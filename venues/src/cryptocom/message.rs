//! Common message structures for Crypto.com REST API responses

use serde::Deserialize;

/// Generic API result structure for Crypto.com REST API responses
#[derive(Debug, Clone, Deserialize)]
pub struct ApiResult<T> {
    /// Response code (0 = success)
    #[serde(rename = "code")]
    pub code: i64,

    /// Result data containing the actual response payload
    #[serde(rename = "result")]
    pub result: T,

    /// Response ID (may be -1)
    #[serde(rename = "id")]
    pub id: i64,

    /// Method name (optional, present in some responses)
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}
