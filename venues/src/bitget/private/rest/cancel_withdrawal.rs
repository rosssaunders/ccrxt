use crate::bitget::{
    BitgetRestClient,
};
use reqwest::Method;
use rest::BitgetRequest;
use serde::{Deserialize, Serialize};

/// Request for canceling withdrawal
#[derive(Debug, Clone, Serialize)]
pub struct CancelWithdrawalRequest {
    /// Withdraw order ID
    #[serde(rename = "orderId")]
    pub order_id: String,
}

/// Response for canceling withdrawal
#[derive(Debug, Clone, Deserialize)]
pub struct CancelWithdrawalResponse {
    /// Result of the cancellation (success/fail)
    pub data: String,
}

impl CancelWithdrawalRequest {
    /// Create a new request
    pub fn new(order_id: impl Into<String>) -> Self {
        Self {
            order_id: order_id.into(),
        }
    }
}

impl BitgetRequest for CancelWithdrawalRequest {
    type Response = CancelWithdrawalResponse;

    fn path(&self) -> String {
        "/api/v2/spot/wallet/cancel-withdrawal".to_string()
    }

    fn method(&self) -> String {
        "POST".to_string()
    }

    fn need_signature(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_creation() {
        let request = CancelWithdrawalRequest::new("1231231312312");
        assert_eq!(request.order_id, "1231231312312");
    }

    #[test]
    fn test_serialization() {
        let request = CancelWithdrawalRequest::new("1231231312312");
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("orderId"));
        assert!(json.contains("1231231312312"));
    }
}
