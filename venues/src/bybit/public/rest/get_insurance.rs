use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult};

const INSURANCE_ENDPOINT: &str = "/v5/market/insurance";

/// Request parameters for getting insurance pool data
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetInsuranceRequest {
    /// Coin name (e.g., "BTC", "ETH"). If not passed, return all coins
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

/// Insurance pool information for a specific coin
#[derive(Debug, Clone, Deserialize)]
pub struct InsuranceInfo {
    /// Coin name
    pub coin: String,

    /// Insurance pool balance
    pub balance: String,

    /// USD value
    pub value: String,
}

/// Insurance pool data container
#[derive(Debug, Clone, Deserialize)]
pub struct GetInsuranceData {
    /// Data updated timestamp in milliseconds
    #[serde(rename = "updatedTime")]
    pub updated_time: String,

    /// Array of insurance pool info
    pub list: Vec<InsuranceInfo>,
}

/// Response from the insurance pool endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetInsuranceResponse {
    /// Success/Error code (0: success, 1: error)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Success/Error message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Business data result
    pub result: GetInsuranceData,

    /// Extended information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Current timestamp in milliseconds
    pub time: u64,
}

impl RestClient {
    /// Get insurance pool data
    ///
    /// Query for Bybit insurance pool data. The insurance pool is used to cover losses from
    /// liquidated positions to ensure counterparty risks are minimized.
    ///
    /// [API Documentation](https://bybit-exchange.github.io/docs/v5/market/insurance)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - Optional request parameters:
    ///   - `coin`: Optional coin name filter. If not provided, returns all coins
    ///
    /// # Returns
    /// A result containing the insurance pool response with balance and USD value data or an error
    pub async fn get_insurance(
        &self,
        request: Option<GetInsuranceRequest>,
    ) -> RestResult<GetInsuranceResponse> {
        self.send_public_request(INSURANCE_ENDPOINT, request.as_ref(), EndpointType::Market)
            .await
    }
}
