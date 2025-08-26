use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const PARTNER_COMMISSION_HISTORY_ENDPOINT: &str = "/rebate/partner/commission_history";

/// Partner rebate records request parameters
#[derive(Debug, Clone, Serialize, Default)]
pub struct PartnerCommissionHistoryRequest {
    /// Trading pair filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// User ID filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// Start time (defaults to 7 days prior if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time (defaults to current time if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// Partner commission history response
#[derive(Debug, Clone, Deserialize)]
pub struct PartnerCommissionHistoryResponse {
    /// Total number of records
    pub total: i64,

    /// List of commission records
    pub list: Vec<PartnerCommission>,
}

/// Individual partner commission record
#[derive(Debug, Clone, Deserialize)]
pub struct PartnerCommission {
    /// Commission ID
    pub id: String,

    /// User ID
    pub user_id: i64,

    /// Trading pair
    pub currency_pair: String,

    /// Commission amount
    pub commission: String,

    /// Currency of the commission
    pub currency: String,

    /// Commission rate
    pub rate: String,

    /// Transaction volume
    pub volume: String,

    /// Commission time
    pub create_time: i64,
}

impl RestClient {
    /// Partner Obtains Rebate Records of Recommended Users
    ///
    /// Query time range is limited to 30 days
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#partner-obtains-rebate-records-of-recommended-users)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Commission history request parameters with optional filters
    ///
    /// # Returns
    /// Commission history response with total count and commission list
    pub async fn get_partner_commission_history(
        &self,
        req: Option<PartnerCommissionHistoryRequest>,
    ) -> RestResult<PartnerCommissionHistoryResponse> {
        self.send_get_request(PARTNER_COMMISSION_HISTORY_ENDPOINT, req.as_ref())
            .await
    }
}
