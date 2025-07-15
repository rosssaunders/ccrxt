use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult};

const COLLATERAL_RATIO_ENDPOINT: &str = "/v5/spot-margin-trade/collateral";

/// Request parameters for getting collateral ratio information.
///
/// This is an empty request as the endpoint doesn't require any parameters.
#[derive(Debug, Clone, Serialize)]
pub struct GetCollateralRatioRequest;

/// Collateral ratio information for a currency.
#[derive(Debug, Clone, Deserialize)]
pub struct CollateralRatioInfo {
    /// Currency symbol (e.g., "BTC", "USDT")
    pub currency: String,

    /// Hourly borrow rate for this currency (as a decimal, e.g., "0.0001" = 0.01%)
    #[serde(rename = "hourlyBorrowRate")]
    pub hourly_borrow_rate: String,

    /// Maximum amount that can be borrowed for this currency
    #[serde(rename = "maxBorrowingAmount")]
    pub max_borrowing_amount: String,

    /// Free borrowing amount available
    #[serde(rename = "freeBorrowingAmount")]
    pub free_borrowing_amount: String,

    /// Free borrow amount (alternative field)
    #[serde(rename = "freeBorrowAmount")]
    pub free_borrow_amount: String,

    /// Maximum borrow amount (alternative field)
    #[serde(rename = "maxBorrowAmount")]
    pub max_borrow_amount: String,

    /// Current borrow usage rate (as a decimal, e.g., "0.5" = 50%)
    #[serde(rename = "borrowUsageRate")]
    pub borrow_usage_rate: String,

    /// Margin collateral ratio for this currency (as a decimal, e.g., "0.85" = 85%)
    #[serde(rename = "marginCollateralRatio")]
    pub margin_collateral_ratio: String,

    /// Liquidation collateral ratio for this currency (as a decimal, e.g., "0.93" = 93%)
    #[serde(rename = "liquidationCollateralRatio")]
    pub liquidation_collateral_ratio: String,

    /// Whether this currency can be used as collateral ("1" = enabled, "0" = disabled)
    #[serde(rename = "collateralSwitch")]
    pub collateral_switch: String,
}

/// Container for the list of collateral ratio information.
#[derive(Debug, Clone, Deserialize)]
pub struct GetCollateralRatioData {
    /// List of all currencies with their collateral ratio information
    pub list: Vec<CollateralRatioInfo>,
}

/// Response from the get collateral ratio API endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetCollateralRatioResponse {
    /// Return code (0 indicates success)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Return message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Collateral ratio data
    pub result: GetCollateralRatioData,

    /// Extended information (varies by endpoint)
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Response timestamp in milliseconds
    pub time: u64,
}

impl RestClient {
    /// Get collateral ratio information
    ///
    /// Query the collateral ratios and borrowing information for various currencies
    /// in spot margin trading.
    ///
    /// [API Documentation](https://bybit-exchange.github.io/docs/v5/spot-margin-trade/collateral-info)
    ///
    /// Rate limit: 50 requests per second
    ///
    /// # Returns
    /// A result containing collateral ratios and borrowing limits for all supported currencies
    pub async fn get_collateral_ratio(&self) -> RestResult<GetCollateralRatioResponse> {
        self.send_public_request(
            COLLATERAL_RATIO_ENDPOINT,
            None::<&GetCollateralRatioRequest>,
            EndpointType::Market,
        )
        .await
    }
}
