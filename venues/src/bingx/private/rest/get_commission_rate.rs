use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::{EndpointType, RestResult};

/// Request to get trading commission rate
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCommissionRateRequest {
    /// Trading pair, e.g., BTC-USDT (must use uppercase letters)
    pub symbol: String,

    /// Request valid time window in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
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
    ///
    /// # Example
    /// ```rust,no_run
    /// use venues::bingx::{PrivateRestClient, GetCommissionRateRequest};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client: PrivateRestClient = unimplemented!();
    ///     let request = GetCommissionRateRequest {
    ///         symbol: "BTC-USDT".to_string(),
    ///         recv_window: None,
    ///     };
    ///     let commission_rate = client.get_commission_rate(&request).await?;
    ///     println!("Taker rate: {}, Maker rate: {}",
    ///         commission_rate.taker_commission_rate,
    ///         commission_rate.maker_commission_rate);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_commission_rate(&self, request: &GetCommissionRateRequest) -> RestResult<GetCommissionRateResponse> {
        self.send_request(
            "/openApi/spot/v1/user/commissionRate",
            reqwest::Method::GET,
            Some(request),
            EndpointType::Trading,
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
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-USDT"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_get_commission_rate_request_minimal() {
        let request = GetCommissionRateRequest {
            symbol: "BTC-USDT".to_string(),
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-USDT"));
        assert!(!serialized.contains("recvWindow"));
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
