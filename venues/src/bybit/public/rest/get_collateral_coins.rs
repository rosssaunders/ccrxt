use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult};

const COLLATERAL_COINS_ENDPOINT: &str = "/v5/crypto-loan/collateral-data";

#[derive(Debug, Clone, Serialize)]
pub struct GetCollateralCoinsRequest;

#[derive(Debug, Clone, Deserialize)]
pub struct CollateralCoinInfo {
    #[serde(rename = "collateralCoin")]
    pub collateral_coin: String,
    #[serde(rename = "maxCollateralAmount")]
    pub max_collateral_amount: String,
    #[serde(rename = "collateralRatio")]
    pub collateral_ratio: String,
    #[serde(rename = "liquidationThreshold")]
    pub liquidation_threshold: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetCollateralCoinsData {
    pub list: Vec<CollateralCoinInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetCollateralCoinsResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetCollateralCoinsData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    pub async fn get_collateral_coins(&self) -> RestResult<GetCollateralCoinsResponse> {
        self.send_public_request(
            COLLATERAL_COINS_ENDPOINT,
            None::<&GetCollateralCoinsRequest>,
            EndpointType::Market,
        )
        .await
    }
}
