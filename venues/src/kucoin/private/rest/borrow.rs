use super::RestClient;
use crate::kucoin::{ResponseHeaders, RestResponse, Result};
use serde::{Deserialize, Serialize};

const BORROW_ENDPOINT: &str = "/api/v3/margin/borrow";

/// Request for borrowing margin
#[derive(Debug, Clone, Serialize)]
pub struct BorrowRequest {
    /// Currency to borrow
    pub currency: String,
    /// Borrow amount
    pub size: String,
    /// Time in force: IOC or FOK
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// Symbol, mandatory for isolated margin account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// true-isolated, false-cross; default is false
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    pub is_isolated: Option<bool>,
    /// true: high frequency borrowing, false: low frequency borrowing; default false
    #[serde(rename = "isHf", skip_serializing_if = "Option::is_none")]
    pub is_hf: Option<bool>,
}

/// Response data for borrow operation
#[derive(Debug, Clone, Deserialize)]
pub struct BorrowResponse {
    /// Borrow Order ID
    #[serde(rename = "orderNo")]
    pub order_no: String,
    /// Actual borrowed amount
    #[serde(rename = "actualSize")]
    pub actual_size: String,
}

/// Time in force for borrow orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeInForce {
    #[serde(rename = "IOC")]
    ImmediateOrCancel,
    #[serde(rename = "FOK")]
    FillOrKill,
}

impl RestClient {
    /// Borrow margin
    ///
    /// This API endpoint is used to initiate an application for cross or isolated margin borrowing.
    pub async fn borrow(
        &self,
        request: BorrowRequest,
    ) -> Result<(BorrowResponse, ResponseHeaders)> {
        let body = serde_json::to_string(&request).map_err(|e| {
            crate::kucoin::ApiError::JsonParsing(format!("Failed to serialize request: {}", e))
        })?;
        let (response, headers): (RestResponse<BorrowResponse>, ResponseHeaders) =
            self.post(BORROW_ENDPOINT, &body).await?;
        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrow_request_creation() {
        let request = BorrowRequest {
            currency: "USDT".to_string(),
            size: "100.0".to_string(),
            time_in_force: TimeInForce::ImmediateOrCancel,
            symbol: Some("BTC-USDT".to_string()),
            is_isolated: Some(true),
            is_hf: Some(false),
        };

        assert_eq!(request.currency, "USDT");
        assert_eq!(request.size, "100.0");
        assert!(matches!(
            request.time_in_force,
            TimeInForce::ImmediateOrCancel
        ));
        assert_eq!(request.symbol, Some("BTC-USDT".to_string()));
        assert_eq!(request.is_isolated, Some(true));
        assert_eq!(request.is_hf, Some(false));
    }

    #[test]
    fn test_time_in_force_serialization() {
        assert_eq!(
            serde_json::to_string(&TimeInForce::ImmediateOrCancel).unwrap(),
            "\"IOC\""
        );
        assert_eq!(
            serde_json::to_string(&TimeInForce::FillOrKill).unwrap(),
            "\"FOK\""
        );
    }
}
