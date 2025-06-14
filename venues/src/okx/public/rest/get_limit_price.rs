use serde::{Deserialize, Serialize};
use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Request parameters for getting limit price
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLimitPriceRequest {
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,
}

/// Limit price information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitPrice {
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Highest buy limit (returns "" when enabled is false)
    #[serde(rename = "buyLmt")]
    pub buy_lmt: String,
    /// Lowest sell limit (returns "" when enabled is false)
    #[serde(rename = "sellLmt")]
    pub sell_lmt: String,
    /// Data return time (Unix timestamp in milliseconds)
    pub ts: String,
    /// Whether price limit is effective
    pub enabled: bool,
}

/// Response for getting limit price
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLimitPriceResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Limit price data
    pub data: Vec<LimitPrice>,
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
    /// * `request` - The limit price request parameters
    ///
    /// # Returns
    /// Response containing the limit price information
    pub async fn get_limit_price(
        &self,
        request: GetLimitPriceRequest,
    ) -> RestResult<GetLimitPriceResponse> {
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
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_limit_price_request_structure() {
        let request = GetLimitPriceRequest {
            inst_id: "BTC-USDT-SWAP".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instId").and_then(|v| v.as_str()), Some("BTC-USDT-SWAP"));
    }

    #[test]
    fn test_limit_price_structure() {
        let limit_price_json = json!({
            "instType": "SWAP",
            "instId": "BTC-USDT-SWAP",
            "buyLmt": "32000.0",
            "sellLmt": "30000.0",
            "ts": "1597026383085",
            "enabled": true
        });

        let limit_price: LimitPrice = serde_json::from_value(limit_price_json).unwrap();
        assert_eq!(limit_price.inst_type, "SWAP");
        assert_eq!(limit_price.inst_id, "BTC-USDT-SWAP");
        assert_eq!(limit_price.buy_lmt, "32000.0");
        assert_eq!(limit_price.sell_lmt, "30000.0");
        assert_eq!(limit_price.enabled, true);
    }

    #[test]
    fn test_get_limit_price_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "SWAP",
                    "instId": "BTC-USDT-SWAP",
                    "buyLmt": "32000.0",
                    "sellLmt": "30000.0",
                    "ts": "1597026383085",
                    "enabled": true
                }
            ]
        });

        let response: GetLimitPriceResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        let limit_price = response.data.first().unwrap();
        assert_eq!(limit_price.inst_id, "BTC-USDT-SWAP");
        assert_eq!(limit_price.buy_lmt, "32000.0");
    }
}