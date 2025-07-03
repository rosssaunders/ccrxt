use crate::bitget::{BitgetRestClient, RestResult};
use reqwest::Method;
use rest::BitgetRequest;
use serde::{Deserialize, Serialize};

/// VIP fee rate level information
#[derive(Debug, Clone, Deserialize)]
pub struct VipFeeRate {
    /// VIP level
    pub level: String,

    /// Total trading volume of the last 30 days, USDT
    #[serde(rename = "dealAmount")]
    pub deal_amount: String,

    /// Total assets, USDT
    #[serde(rename = "assetAmount")]
    pub asset_amount: String,

    /// Taker rate, '0.000425' means 4.25 with four decimal places ahead
    #[serde(rename = "takerFeeRate")]
    pub taker_fee_rate: String,

    /// Maker rate, '0.00006' means 0.6 with four decimal places ahead
    #[serde(rename = "makerFeeRate")]
    pub maker_fee_rate: String,

    /// 24-hour withdrawal limit (BTC)
    #[serde(rename = "btcWithdrawAmount")]
    pub btc_withdraw_amount: String,

    /// 24-hour withdrawal limit (USDT)
    #[serde(rename = "usdtWithdrawAmount")]
    pub usdt_withdraw_amount: String,
}

/// Request for getting VIP fee rates
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetVipFeeRateRequest;

/// Response for getting VIP fee rates
#[derive(Debug, Clone, Deserialize)]
pub struct GetVipFeeRateResponse {
    /// List of VIP fee rate levels
    pub data: Vec<VipFeeRate>,
}

impl BitgetRequest for GetVipFeeRateRequest {
    type Response = GetVipFeeRateResponse;

    fn path(&self) -> String {
        "/api/v2/mix/market/vip-fee-rate".to_string()
    }

    fn method(&self) -> String {
        "GET".to_string()
    }

    fn need_signature(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vip_fee_rate_request() {
        let request = GetVipFeeRateRequest::default();
        assert_eq!(request.path(), "/api/v2/mix/market/vip-fee-rate");
        assert_eq!(request.method(), "GET".to_string());
        assert!(!request.need_signature());
    }
}
