use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, InstrumentType, RestResult};

/// Request to get trade fee
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradeFeeRequest {
    /// Instrument type
    pub inst_type: InstrumentType,

    /// Instrument ID, e.g. "BTC-USDT"
    /// Optional for SPOT, required for others
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,

    /// Underlying, e.g. "BTC-USD"
    /// Only applicable to FUTURES/SWAP/OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,

    /// Instrument family, e.g. "BTC-USD"
    /// Only applicable to FUTURES/SWAP/OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// Trade fee details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeFee {
    /// Category
    pub category: String,

    /// Delivery
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<String>,

    /// Exercise
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exercise: Option<String>,

    /// Instrument family
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,

    /// Instrument ID
    pub inst_id: String,

    /// Instrument type
    pub inst_type: String,

    /// Level
    pub level: String,

    /// Maker fee rate
    pub maker: String,

    /// Taker fee rate
    pub taker: String,

    /// Timestamp
    pub ts: String,
}

impl RestClient {
    /// Get trade fee
    ///
    /// # Arguments
    /// * `request` - The get trade fee request
    ///
    /// # Returns
    /// A result containing the trade fee or an error
    pub async fn get_trade_fee(&self, request: &GetTradeFeeRequest) -> RestResult<OkxApiResponse<TradeFee>> {
        self.send_request(
            "api/v5/account/trade-fee",
            reqwest::Method::GET,
            Some(request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_trade_fee_request_serialization() {
        let request = GetTradeFeeRequest {
            inst_type: InstrumentType::Spot,
            inst_id: Some("BTC-USDT".to_string()),
            uly: None,
            inst_family: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instType=SPOT"));
        assert!(serialized.contains("instId=BTC-USDT"));
    }

    #[test]
    fn test_get_trade_fee_request_futures() {
        let request = GetTradeFeeRequest {
            inst_type: InstrumentType::Futures,
            inst_id: None,
            uly: Some("BTC-USD".to_string()),
            inst_family: Some("BTC-USD".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instType=FUTURES"));
        assert!(serialized.contains("uly=BTC-USD"));
        assert!(serialized.contains("instFamily=BTC-USD"));
    }

    #[test]
    fn test_trade_fee_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "category": "1",
                    "delivery": "",
                    "exercise": "",
                    "instFamily": "",
                    "instId": "BTC-USDT",
                    "instType": "SPOT",
                    "level": "Lv1",
                    "maker": "-0.0008",
                    "taker": "-0.001",
                    "ts": "1597026383085"
                }
            ]
        }"#;

        let response: OkxApiResponse<TradeFee> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let fee = response.data.first();
        assert!(fee.is_some(), "Expected at least one fee in response");
        let fee = fee.unwrap();
        assert_eq!(fee.inst_id, "BTC-USDT");
        assert_eq!(fee.inst_type, "SPOT");
        assert_eq!(fee.level, "Lv1");
        assert_eq!(fee.maker, "-0.0008");
        assert_eq!(fee.taker, "-0.001");
    }
}
