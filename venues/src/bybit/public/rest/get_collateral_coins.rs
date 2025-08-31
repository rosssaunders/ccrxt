use serde::{Deserialize, Serialize};

use crate::bybit::{EndpointType, PublicRestClient, RestResult};

const COLLATERAL_COINS_ENDPOINT: &str = "/v5/crypto-loan/collateral-data";

/// Request parameters for getting collateral coins.
///
/// This is an empty request as the endpoint doesn't require any parameters.
#[derive(Debug, Clone, Serialize)]
pub struct GetCollateralCoinsRequest;

/// Information about a coin that can be used as collateral.
#[derive(Debug, Clone, Deserialize)]
pub struct CollateralCoinInfo {
    /// Symbol of the coin that can be used as collateral (e.g., "BTC", "ETH")
    #[serde(rename = "collateralCoin")]
    pub collateral_coin: String,

    /// Maximum amount of this coin that can be used as collateral
    #[serde(rename = "maxCollateralAmount")]
    pub max_collateral_amount: String,

    /// The collateral ratio for this coin (as a decimal, e.g., "0.85" = 85%)
    #[serde(rename = "collateralRatio")]
    pub collateral_ratio: String,

    /// The liquidation threshold for this collateral (as a decimal, e.g., "0.93" = 93%)
    #[serde(rename = "liquidationThreshold")]
    pub liquidation_threshold: String,
}

/// Container for the list of collateral coins.
#[derive(Debug, Clone, Deserialize)]
pub struct GetCollateralCoinsData {
    /// List of all collateral coins with their parameters
    pub list: Vec<CollateralCoinInfo>,
}

/// Response from the get collateral coins API endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetCollateralCoinsResponse {
    /// Return code (0 indicates success)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Return message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Collateral coins data
    pub result: GetCollateralCoinsData,

    /// Extended information (varies by endpoint)
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Response timestamp in milliseconds
    pub time: u64,
}

impl PublicRestClient {
    /// Get collateral coins
    ///
    /// Query the list of coins that can be used as collateral for crypto loans including
    /// their collateral ratios and liquidation thresholds.
    ///
    /// [docs](https://bybit-exchange.github.io/docs/v5/crypto-loan/collateral-coin)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Returns
    /// A result containing the list of collateral coins with ratios and thresholds
    pub async fn get_collateral_coins(&self) -> RestResult<GetCollateralCoinsResponse> {
        self.send_public_request(
            COLLATERAL_COINS_ENDPOINT,
            None::<&GetCollateralCoinsRequest>,
            EndpointType::Market,
        )
        .await
    }
}
