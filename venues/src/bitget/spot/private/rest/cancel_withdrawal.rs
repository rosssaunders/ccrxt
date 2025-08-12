use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::spot::RestResult;

/// Request for canceling withdrawal
#[derive(Debug, Clone, Serialize, Default)]
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

impl RestClient {
    /// Cancel a withdrawal request
    ///
    /// Cancels a withdrawal by order ID.
    ///
    /// [docs](https://www.bitget.com/api-doc/spot/wallet/Cancel-Withdrawal)
    ///
    /// Rate limit: 5 req/sec/UID
    ///
    /// Returns a `RestResult<CancelWithdrawalResponse>` containing the result or an error.
    pub async fn cancel_withdrawal(
        &self,
        params: CancelWithdrawalRequest,
    ) -> RestResult<CancelWithdrawalResponse> {
        let endpoint = "/api/v2/spot/wallet/cancel-withdrawal";
        self.send_post_signed_request(endpoint, params, 5, false, None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_creation() {
        let request = CancelWithdrawalRequest {
            order_id: "1231231312312".into(),
        };
        assert_eq!(request.order_id, "1231231312312");
    }

    #[test]
    fn test_serialization() {
        let request = CancelWithdrawalRequest {
            order_id: "1231231312312".into(),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("orderId"));
        assert!(json.contains("1231231312312"));
    }
}
