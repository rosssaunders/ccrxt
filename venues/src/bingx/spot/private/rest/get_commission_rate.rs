use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult};

const COMMISSION_RATE_ENDPOINT: &str = "/openApi/spot/v1/account/commissionRate";

/// Request to get trading commission rate
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCommissionRateRequest {
    /// Trading pair, e.g., BTC-USDT (must use uppercase letters)
    pub symbol: String,

    /// Request valid time window in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Request timestamp in milliseconds
    pub timestamp: i64,
}

/// Response from getting trading commission rate
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCommissionRateResponse {
    /// Taker commission rate
    pub taker_commission_rate: f64,

    /// Maker commission rate
    pub maker_commission_rate: f64,
}

impl RestClient {
    /// Get trading commission rate
    ///
    /// Retrieves the current trading commission rate for spot trading.
    /// Rate limit: 2/s by UID
    ///
    /// # Arguments
    /// * `request` - The get commission rate request with symbol
    ///
    /// # Returns
    /// A result containing the commission rates or an error
    pub async fn get_commission_rate(
        &self,
        request: &GetCommissionRateRequest,
    ) -> RestResult<GetCommissionRateResponse> {
        self.send_get_signed_request(COMMISSION_RATE_ENDPOINT, request, EndpointType::Trading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_commission_rate_request_serialization() {
        let request = GetCommissionRateRequest {
            symbol: "BTC-USDT".to_string(),
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-USDT"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1640995200000"));
    }

    #[test]
    fn test_get_commission_rate_request_minimal() {
        let request = GetCommissionRateRequest {
            symbol: "BTC-USDT".to_string(),
            recv_window: None,
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-USDT"));
        assert!(!serialized.contains("recvWindow"));
        assert!(serialized.contains("timestamp=1640995200000"));
    }

    #[test]
    fn test_commission_rate_response_deserialization() {
        let json = r#"{
            "takerCommissionRate": 0.001,
            "makerCommissionRate": 0.0008
        }"#;

        let response: GetCommissionRateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.taker_commission_rate, 0.001);
        assert_eq!(response.maker_commission_rate, 0.0008);
    }
}
