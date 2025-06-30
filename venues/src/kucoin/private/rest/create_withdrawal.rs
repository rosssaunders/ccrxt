use serde::{Deserialize, Serialize};

use crate::kucoin::{ResponseHeaders, RestResponse, Result};

use super::RestClient;

/// Request for creating a withdrawal
#[derive(Debug, Clone, Serialize)]
pub struct CreateWithdrawalRequest {
    /// Currency code
    pub currency: String,

    /// Withdrawal address
    pub address: String,

    /// Amount to withdraw
    pub amount: String,

    /// Address memo/tag (optional)
    pub memo: Option<String>,

    /// Is internal transfer flag (optional)
    #[serde(rename = "isInner")]
    pub is_inner: Option<bool>,

    /// Remark (optional)
    pub remark: Option<String>,

    /// Chain name (optional)
    pub chain: Option<String>,

    /// Fee deduction type (optional): INTERNAL, EXTERNAL
    #[serde(rename = "feeDeductType")]
    pub fee_deduct_type: Option<String>,
}

/// Withdrawal response
#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawalResponse {
    /// Withdrawal ID
    #[serde(rename = "withdrawalId")]
    pub withdrawal_id: String,
}

impl RestClient {
    /// Create a withdrawal
    ///
    /// Reference: https://docs.kucoin.com/#apply-withdraw-v1
    pub async fn create_withdrawal(
        &self,
        request: CreateWithdrawalRequest,
    ) -> Result<(WithdrawalResponse, ResponseHeaders)> {
        let body = serde_json::to_string(&request).map_err(|e| {
            crate::kucoin::ApiError::JsonParsing(format!("Failed to serialize request: {}", e))
        })?;

        let (response, headers): (RestResponse<WithdrawalResponse>, ResponseHeaders) =
            self.post("/api/v1/withdrawals", &body).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_withdrawal_request_creation() {
        let request = CreateWithdrawalRequest {
            currency: "BTC".to_string(),
            address: "test_address".to_string(),
            amount: "0.001".to_string(),
            memo: None,
            is_inner: Some(false),
            remark: Some("Test".to_string()),
            chain: None,
            fee_deduct_type: None,
        };
        assert_eq!(request.currency, "BTC");
        assert_eq!(request.amount, "0.001");
        assert_eq!(request.is_inner, Some(false));
    }
}
