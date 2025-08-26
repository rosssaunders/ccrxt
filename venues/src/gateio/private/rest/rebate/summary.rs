use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const SUMMARY_ENDPOINT: &str = "/rebate/summary";

/// Query rebate summary
#[derive(Debug, Clone, Serialize, Default)]
pub struct SummaryQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// Rebate summary item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebateSummary {
    pub currency: String,

    pub amount: String,
}

impl RestClient {
    /// Get rebate summary
    pub async fn get_rebate_summary(&self, query: SummaryQuery) -> RestResult<Vec<RebateSummary>> {
        self.send_get_request(SUMMARY_ENDPOINT, Some(&query)).await
    }
}
