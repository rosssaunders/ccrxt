use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult};

const COLLATERAL_RATIO_ENDPOINT: &str = "/v5/spot-margin-trade/collateral";

#[derive(Debug, Clone, Serialize)]
pub struct GetCollateralRatioRequest;

#[derive(Debug, Clone, Deserialize)]
pub struct CollateralRatioInfo {
    pub currency: String,
    #[serde(rename = "hourlyBorrowRate")]
    pub hourly_borrow_rate: String,
    #[serde(rename = "maxBorrowingAmount")]
    pub max_borrowing_amount: String,
    #[serde(rename = "freeBorrowingAmount")]
    pub free_borrowing_amount: String,
    #[serde(rename = "freeBorrowAmount")]
    pub free_borrow_amount: String,
    #[serde(rename = "maxBorrowAmount")]
    pub max_borrow_amount: String,
    #[serde(rename = "borrowUsageRate")]
    pub borrow_usage_rate: String,
    #[serde(rename = "marginCollateralRatio")]
    pub margin_collateral_ratio: String,
    #[serde(rename = "liquidationCollateralRatio")]
    pub liquidation_collateral_ratio: String,
    #[serde(rename = "collateralSwitch")]
    pub collateral_switch: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetCollateralRatioData {
    pub list: Vec<CollateralRatioInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetCollateralRatioResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetCollateralRatioData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    pub async fn get_collateral_ratio(&self) -> RestResult<GetCollateralRatioResponse> {
        self.send_public_request(
            COLLATERAL_RATIO_ENDPOINT,
            None::<&GetCollateralRatioRequest>,
            EndpointType::Market,
        )
        .await
    }
}
