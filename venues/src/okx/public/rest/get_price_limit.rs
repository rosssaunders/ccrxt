use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{EndpointType, InstrumentType, RestResult};

/// Request parameters for getting price limit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPriceLimitRequest {
    /// Instrument ID (required)
    #[serde(rename = "instId")]
    pub inst_id: String,
}

/// Price limit data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceLimit {
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Highest buy limit (empty string when enabled is false)
    #[serde(rename = "buyLmt")]
    pub buy_lmt: String,
    /// Lowest sell limit (empty string when enabled is false)
    #[serde(rename = "sellLmt")]
    pub sell_lmt: String,
    /// Data return time, Unix timestamp format in milliseconds
    pub ts: String,
    /// Whether price limit is effective
    pub enabled: bool,
}

/// Response for getting price limit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPriceLimitResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Price limit data
    pub data: Vec<PriceLimit>,
}

impl RestClient {
    /// Get limit price
    ///
    /// Retrieve the highest buy limit and lowest sell limit of the instrument.
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-public-data-get-limit-price
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The price limit request parameters
    ///
    /// # Returns
    /// Response containing the price limit information
    pub async fn get_price_limit(
        &self,
        request: GetPriceLimitRequest,
    ) -> RestResult<GetPriceLimitResponse> {
        self.send_request(
            "api/v5/public/price-limit",
            reqwest::Method::GET,
            Some(&request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_get_price_limit_request_structure() {
        let request = GetPriceLimitRequest {
            inst_id: "BTC-USDT-SWAP".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USDT-SWAP")
        );
    }

    #[test]
    fn test_price_limit_structure() {
        let price_limit_json = json!({
            "instType": "SWAP",
            "instId": "BTC-USDT-SWAP",
            "buyLmt": "35000.0",
            "sellLmt": "25000.0",
            "ts": "1597026383085",
            "enabled": true
        });

        let price_limit: PriceLimit = serde_json::from_value(price_limit_json).unwrap();
        assert_eq!(price_limit.inst_type, InstrumentType::Swap);
        assert_eq!(price_limit.inst_id, "BTC-USDT-SWAP");
        assert_eq!(price_limit.buy_lmt, "35000.0");
        assert_eq!(price_limit.sell_lmt, "25000.0");
        assert_eq!(price_limit.ts, "1597026383085");
        assert!(price_limit.enabled);
    }

    #[test]
    fn test_price_limit_structure_when_disabled() {
        let price_limit_json = json!({
            "instType": "SPOT",
            "instId": "BTC-USDT",
            "buyLmt": "",
            "sellLmt": "",
            "ts": "1597026383085",
            "enabled": false
        });

        let price_limit: PriceLimit = serde_json::from_value(price_limit_json).unwrap();
        assert_eq!(price_limit.inst_type, InstrumentType::Spot);
        assert_eq!(price_limit.inst_id, "BTC-USDT");
        assert_eq!(price_limit.buy_lmt, "");
        assert_eq!(price_limit.sell_lmt, "");
        assert_eq!(price_limit.ts, "1597026383085");
        assert!(!price_limit.enabled);
    }

    #[test]
    fn test_get_price_limit_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "SWAP",
                    "instId": "BTC-USDT-SWAP",
                    "buyLmt": "35000.0",
                    "sellLmt": "25000.0",
                    "ts": "1597026383085",
                    "enabled": true
                }
            ]
        });

        let response: GetPriceLimitResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().inst_id, "BTC-USDT-SWAP");
        assert!(response.data.first().unwrap().enabled);
    }

    #[test]
    fn test_price_limit_serialization_roundtrip() {
        let original = GetPriceLimitRequest {
            inst_id: "ETH-USD-SWAP".to_string(),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetPriceLimitRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_id, deserialized.inst_id);
    }

    #[test]
    fn test_price_limit_with_different_instrument_types() {
        // Test with FUTURES
        let futures_json = json!({
            "instType": "FUTURES",
            "instId": "BTC-USD-240329",
            "buyLmt": "45000.0",
            "sellLmt": "35000.0",
            "ts": "1597026383085",
            "enabled": true
        });

        let futures_limit: PriceLimit = serde_json::from_value(futures_json).unwrap();
        assert_eq!(futures_limit.inst_type, InstrumentType::Futures);
        assert_eq!(futures_limit.inst_id, "BTC-USD-240329");

        // Test with OPTION
        let option_json = json!({
            "instType": "OPTION",
            "instId": "BTC-USD-240329-40000-C",
            "buyLmt": "1500.0",
            "sellLmt": "1000.0",
            "ts": "1597026383085",
            "enabled": true
        });

        let option_limit: PriceLimit = serde_json::from_value(option_json).unwrap();
        assert_eq!(option_limit.inst_type, InstrumentType::Option);
        assert_eq!(option_limit.inst_id, "BTC-USD-240329-40000-C");
    }
}
