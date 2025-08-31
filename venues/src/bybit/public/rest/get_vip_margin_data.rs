use serde::{Deserialize, Serialize};

use crate::bybit::{EndpointType, PublicRestClient, RestResult};

const VIP_MARGIN_DATA_ENDPOINT: &str = "/v5/spot-margin-trade/data";

/// Request parameters for getting VIP margin data
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetVipMarginDataRequest {
    /// VIP level
    #[serde(rename = "vipLevel", skip_serializing_if = "Option::is_none")]
    pub vip_level: Option<String>,

    /// Currency name. If not passed, return all currencies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// VIP margin information
#[derive(Debug, Clone, Deserialize)]
pub struct VipMarginInfo {
    /// VIP level
    #[serde(rename = "vipLevel")]
    pub vip_level: String,

    /// Currency name
    pub currency: String,

    /// Whether the currency is borrowable (1: true, 0: false)
    #[serde(rename = "borrowable")]
    pub borrowable: String,

    /// Collateral ratio
    #[serde(rename = "collateralRatio")]
    pub collateral_ratio: String,

    /// Margin call ratio
    #[serde(rename = "marginCallRatio")]
    pub margin_call_ratio: String,

    /// Liquidation ratio
    #[serde(rename = "liquidationRatio")]
    pub liquidation_ratio: String,

    /// Daily interest rate
    #[serde(rename = "interestRate")]
    pub interest_rate: String,

    /// Hourly borrow rate
    #[serde(rename = "hourlyBorrowRate")]
    pub hourly_borrow_rate: String,
}

/// VIP margin data container
#[derive(Debug, Clone, Deserialize)]
pub struct GetVipMarginDataData {
    /// Array of VIP margin info
    pub list: Vec<VipMarginInfo>,
}

/// Response from the VIP margin data endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetVipMarginDataResponse {
    /// Success/Error code (0: success, 1: error)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Success/Error message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Business data result
    pub result: GetVipMarginDataData,

    /// Extended information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Current timestamp in milliseconds
    pub time: u64,
}

impl PublicRestClient {
    /// Get VIP margin data
    ///
    /// Query for Bybit VIP margin data including borrowing rates, collateral ratios, and liquidation ratios.
    ///
    /// [docs](https://bybit-exchange.github.io/docs/v5/spot-margin-uta/vip-margin)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - Optional request parameters:
    ///   - `vip_level`: Optional VIP level filter
    ///   - `currency`: Optional currency filter. If not provided, returns all currencies
    ///
    /// # Returns
    /// A result containing the VIP margin data response with margin info or an error
    pub async fn get_vip_margin_data(
        &self,
        request: Option<GetVipMarginDataRequest>,
    ) -> RestResult<GetVipMarginDataResponse> {
        self.send_public_request(
            VIP_MARGIN_DATA_ENDPOINT,
            request.as_ref(),
            EndpointType::Market,
        )
        .await
    }
}
