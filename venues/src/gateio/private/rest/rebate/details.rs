use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const DETAILS_ENDPOINT: &str = "/rebate/details";

/// Query rebate details
#[derive(Debug, Clone, Serialize, Default)]
pub struct DetailsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Rebate detail record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebateDetail {
    pub currency: String,

    pub amount: String,

    pub timestamp: i64,
}

impl RestClient {
    /// List rebate details
    pub async fn list_rebate_details(&self, query: DetailsQuery) -> RestResult<Vec<RebateDetail>> {
        self.send_get_request(DETAILS_ENDPOINT, Some(&query)).await
    }
}
