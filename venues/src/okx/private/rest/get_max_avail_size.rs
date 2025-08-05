use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

const ACCOUNT_MAX_AVAIL_SIZE_ENDPOINT: &str = "api/v5/account/max-avail-size";

/// Request to get max available size
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxAvailSizeRequest {
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

    /// Whether to reduce position only
    /// true or false, the default is false
    /// Only applicable to MARGIN orders, and FUTURES/SWAP orders in net mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Whether to borrow (only applicable to Multi-currency margin and Portfolio margin)
    /// true: borrow, false: not borrow, the default is false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unSpotOffset")]
    pub un_spot_offset: Option<bool>,

    /// Quick Margin type. Only applicable to Quick Margin Mode of isolated margin
    /// "manual", "auto_borrow", "auto_repay"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_mgn_type: Option<String>,
}

/// Max available size details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxAvailSize {
    /// Instrument ID
    pub inst_id: String,

    /// Available size for buy
    pub avail_buy: String,

    /// Available size for sell
    pub avail_sell: String,
}

impl RestClient {
    /// Get max available size
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-maximum-available-tradable-amount
    ///
    /// # Arguments
    /// * `request` - The get max available size request
    ///
    /// # Returns
    /// A result containing the max available size or an error
    pub async fn get_max_avail_size(
        &self,
        request: &GetMaxAvailSizeRequest,
    ) -> RestResult<MaxAvailSize> {
        self.send_get_request(
            ACCOUNT_MAX_AVAIL_SIZE_ENDPOINT,
            Some(request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okx::response::OkxApiResponse;

    #[test]
    fn test_get_max_avail_size_request_serialization() {
        let request = GetMaxAvailSizeRequest {
            inst_id: "BTC-USDT".to_string(),
            td_mode: "cross".to_string(),
            ccy: Some("USDT".to_string()),
            reduce_only: Some(false),
            un_spot_offset: Some(true),
            quick_mgn_type: Some("auto_borrow".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instId=BTC-USDT"));
        assert!(serialized.contains("tdMode=cross"));
        assert!(serialized.contains("ccy=USDT"));
        assert!(serialized.contains("reduceOnly=false"));
        assert!(serialized.contains("unSpotOffset=true"));
        assert!(serialized.contains("quickMgnType=auto_borrow"));
    }

    #[test]
    fn test_get_max_avail_size_minimal_request() {
        let request = GetMaxAvailSizeRequest {
            inst_id: "BTC-USDT".to_string(),
            td_mode: "cash".to_string(),
            ccy: None,
            reduce_only: None,
            un_spot_offset: None,
            quick_mgn_type: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instId=BTC-USDT"));
        assert!(serialized.contains("tdMode=cash"));
        assert!(!serialized.contains("ccy="));
        assert!(!serialized.contains("reduceOnly="));
        assert!(!serialized.contains("unSpotOffset="));
        assert!(!serialized.contains("quickMgnType="));
    }

    #[test]
    fn test_max_avail_size_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USDT",
                    "availBuy": "0.5",
                    "availSell": "0.3"
                }
            ]
        }"#;

        let response: OkxApiResponse<MaxAvailSize> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let max_avail = &response.data[0];
        assert_eq!(max_avail.inst_id, "BTC-USDT");
        assert_eq!(max_avail.avail_buy, "0.5");
        assert_eq!(max_avail.avail_sell, "0.3");
    }
}
