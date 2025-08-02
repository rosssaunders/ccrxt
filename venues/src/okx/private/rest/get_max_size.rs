use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

const ACCOUNT_MAX_SIZE_ENDPOINT: &str = "api/v5/account/max-size";
/// Request to get max size
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxSizeRequest {
    /// Instrument ID, e.g. "BTC-USDT"
    pub inst_id: String,

    /// Trade mode
    /// Margin mode: "cross", "isolated"
    /// Non-Margin mode: "cash"
    pub td_mode: String,

    /// Currency used for margin
    /// Only applicable to cross MARGIN orders in Futures and Swap mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,

    /// Order price
    /// When the price is not specified, it will be calculated according to the last traded price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,

    /// Leverage
    /// Only applicable to MARGIN/FUTURES/SWAP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lever: Option<String>,

    /// Whether to borrow (only applicable to Multi-currency margin and Portfolio margin)
    /// true: borrow, false: not borrow, the default is false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unSpotOffset")]
    pub un_spot_offset: Option<bool>,
}

/// Max size details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxSize {
    /// Instrument ID
    pub inst_id: String,

    /// Currency
    pub ccy: String,

    /// Max buy
    pub max_buy: String,

    /// Max sell
    pub max_sell: String,
}

impl RestClient {
    /// Get max size
    ///
    /// # Arguments
    /// * `request` - The get max size request
    ///
    /// # Returns
    /// A result containing the max size or an error
    pub async fn get_max_size(
        &self,
        request: &GetMaxSizeRequest,
    ) -> RestResult<OkxApiResponse<MaxSize>> {
        self.send_request(
            ACCOUNT_MAX_SIZE_ENDPOINT,
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
    fn test_get_max_size_request_serialization() {
        let request = GetMaxSizeRequest {
            inst_id: "BTC-USDT".to_string(),
            td_mode: "cross".to_string(),
            ccy: Some("USDT".to_string()),
            px: Some("50000".to_string()),
            lever: Some("10".to_string()),
            un_spot_offset: Some(false),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instId=BTC-USDT"));
        assert!(serialized.contains("tdMode=cross"));
        assert!(serialized.contains("ccy=USDT"));
        assert!(serialized.contains("px=50000"));
        assert!(serialized.contains("lever=10"));
        assert!(serialized.contains("unSpotOffset=false"));
    }

    #[test]
    fn test_get_max_size_minimal_request() {
        let request = GetMaxSizeRequest {
            inst_id: "BTC-USDT".to_string(),
            td_mode: "cash".to_string(),
            ccy: None,
            px: None,
            lever: None,
            un_spot_offset: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instId=BTC-USDT"));
        assert!(serialized.contains("tdMode=cash"));
        assert!(!serialized.contains("ccy="));
        assert!(!serialized.contains("px="));
        assert!(!serialized.contains("lever="));
        assert!(!serialized.contains("unSpotOffset="));
    }

    #[test]
    fn test_max_size_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USDT",
                    "ccy": "USDT",
                    "maxBuy": "0.5",
                    "maxSell": "0.3"
                }
            ]
        }"#;

        let response: OkxApiResponse<MaxSize> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let max_size = &response.data[0];
        assert_eq!(max_size.inst_id, "BTC-USDT");
        assert_eq!(max_size.ccy, "USDT");
        assert_eq!(max_size.max_buy, "0.5");
        assert_eq!(max_size.max_sell, "0.3");
    }
}
