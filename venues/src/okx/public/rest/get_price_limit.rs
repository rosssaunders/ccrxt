use serde::{Deserialize, Serialize};
use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Request parameters for getting price limit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPriceLimitRequest {
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,
}

/// Response for getting price limit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPriceLimitResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Price limit data
    pub data: Vec<PriceLimitData>,
}

/// Price limit data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceLimitData {
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Highest buy limit (return "" when enabled is false)
    #[serde(rename = "buyLmt")]
    pub buy_lmt: String,
    /// Lowest sell limit (return "" when enabled is false)
    #[serde(rename = "sellLmt")]
    pub sell_lmt: String,
    /// Data return time, Unix timestamp format in milliseconds
    pub ts: String,
    /// Whether price limit is effective
    pub enabled: bool,
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
    /// Response containing the price limit data
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
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_price_limit_request_structure() {
        let request = GetPriceLimitRequest {
            inst_id: "BTC-USDT-SWAP".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instId").and_then(|v| v.as_str()), Some("BTC-USDT-SWAP"));
    }

    #[test]
    fn test_price_limit_data_structure() {
        let price_limit_json = json!({
            "instType": "SWAP",
            "instId": "BTC-USDT-SWAP",
            "buyLmt": "52000",
            "sellLmt": "48000",
            "ts": "1597026383085",
            "enabled": true
        });

        let price_limit_data: PriceLimitData = serde_json::from_value(price_limit_json).unwrap();
        assert_eq!(price_limit_data.inst_type, "SWAP");
        assert_eq!(price_limit_data.inst_id, "BTC-USDT-SWAP");
        assert_eq!(price_limit_data.buy_lmt, "52000");
        assert_eq!(price_limit_data.sell_lmt, "48000");
        assert!(price_limit_data.enabled);
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
                    "buyLmt": "52000",
                    "sellLmt": "48000",
                    "ts": "1597026383085",
                    "enabled": true
                }
            ]
        });

        let response: GetPriceLimitResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().inst_id, "BTC-USDT-SWAP");
    }
}