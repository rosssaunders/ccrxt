use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult};

const VIP_MARGIN_DATA_ENDPOINT: &str = "/v5/spot-margin-trade/data";

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetVipMarginDataRequest {
    #[serde(rename = "vipLevel", skip_serializing_if = "Option::is_none")]
    pub vip_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VipMarginInfo {
    #[serde(rename = "vipLevel")]
    pub vip_level: String,
    pub currency: String,
    #[serde(rename = "borrowable")]
    pub borrowable: String,
    #[serde(rename = "collateralRatio")]
    pub collateral_ratio: String,
    #[serde(rename = "marginCallRatio")]
    pub margin_call_ratio: String,
    #[serde(rename = "liquidationRatio")]
    pub liquidation_ratio: String,
    #[serde(rename = "interestRate")]
    pub interest_rate: String,
    #[serde(rename = "hourlyBorrowRate")]
    pub hourly_borrow_rate: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetVipMarginDataData {
    pub list: Vec<VipMarginInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetVipMarginDataResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetVipMarginDataData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
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
