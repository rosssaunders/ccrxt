use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const PARTNER_SUB_LIST_ENDPOINT: &str = "/rebate/partner/sub_list";

/// Partner subordinate list request parameters
#[derive(Debug, Clone, Serialize, Default)]
pub struct PartnerSubListRequest {
    /// User ID filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// Maximum number of records to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// List offset for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Partner subordinate list response
#[derive(Debug, Clone, Deserialize)]
pub struct PartnerSubListResponse {
    /// Total number of subordinates
    pub total: i64,

    /// List of subordinate records
    pub list: Vec<PartnerSubordinate>,
}

/// Individual subordinate record
#[derive(Debug, Clone, Deserialize)]
pub struct PartnerSubordinate {
    /// User ID
    pub user_id: i64,

    /// Subordinate type (sub-agent, direct customer, indirect customer)
    #[serde(rename = "type")]
    pub subordinate_type: String,

    /// Registration time
    pub create_time: i64,

    /// Last active time
    pub last_active_time: Option<i64>,

    /// Total trading volume
    pub total_volume: Option<String>,

    /// Total commission generated
    pub total_commission: Option<String>,
}

impl RestClient {
    /// Partner Subordinate List
    ///
    /// Including sub-agents, direct customers, and indirect customers
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#partner-subordinate-list)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Subordinate list request parameters with optional filters
    ///
    /// # Returns
    /// Subordinate list response with total count and subordinate details
    pub async fn get_partner_sub_list(
        &self,
        req: Option<PartnerSubListRequest>,
    ) -> RestResult<PartnerSubListResponse> {
        self.send_get_request(PARTNER_SUB_LIST_ENDPOINT, req.as_ref())
            .await
    }
}
