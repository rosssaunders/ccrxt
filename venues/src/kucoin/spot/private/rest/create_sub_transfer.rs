use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const CREATE_SUB_TRANSFER_ENDPOINT: &str = "/api/v2/accounts/sub-transfer";

/// Request for sub-account transfer (main account only)
#[derive(Debug, Clone, Serialize)]
pub struct CreateSubTransferRequest {
    /// Client order ID (optional, max 40 characters)
    #[serde(rename = "clientOid")]
    pub client_order_id: Option<String>,

    /// Currency to transfer
    pub currency: String,

    /// Transfer amount
    pub amount: String,

    /// Direction: OUT (from main to sub), IN (from sub to main)
    pub direction: String,

    /// Account type (main, trade, etc.)
    #[serde(rename = "accountType")]
    pub account_type: Option<String>,

    /// Sub-account user ID (required for direction OUT)
    #[serde(rename = "subUserId")]
    pub sub_user_id: Option<String>,

    /// Sub-account type (optional)
    #[serde(rename = "subAccountType")]
    pub sub_account_type: Option<String>,
}

/// Sub-account transfer response
#[derive(Debug, Clone, Deserialize)]
pub struct SubTransferResponse {
    /// Transfer order ID
    #[serde(rename = "orderId")]
    pub order_id: String,
}

impl RestClient {
    /// Create a sub-account transfer (main account only)
    ///
    /// Reference: https://docs.kucoin.com/#sub-account-transfer
    pub async fn create_sub_transfer(
        &self,
        request: CreateSubTransferRequest,
    ) -> Result<(SubTransferResponse, ResponseHeaders)> {
        let body = serde_json::to_string(&request).map_err(|e| {
            crate::kucoin::spot::ApiError::JsonParsing(format!(
                "Failed to serialize request: {}",
                e
            ))
        })?;

        let (response, headers): (RestResponse<SubTransferResponse>, ResponseHeaders) =
            self.post(CREATE_SUB_TRANSFER_ENDPOINT, &body).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_transfer_request_creation() {
        let request = CreateSubTransferRequest {
            client_order_id: Some("sub_001".to_string()),
            currency: "BTC".to_string(),
            amount: "0.01".to_string(),
            direction: "OUT".to_string(),
            account_type: Some("main".to_string()),
            sub_user_id: Some("sub_123".to_string()),
            sub_account_type: Some("trade".to_string()),
        };
        assert_eq!(request.currency, "BTC");
        assert_eq!(request.direction, "OUT");
        assert_eq!(request.sub_user_id, Some("sub_123".to_string()));
    }

    #[test]
    fn test_sub_transfer_request_minimal() {
        let request = CreateSubTransferRequest {
            client_order_id: None,
            currency: "USDT".to_string(),
            amount: "100.0".to_string(),
            direction: "IN".to_string(),
            account_type: None,
            sub_user_id: None,
            sub_account_type: None,
        };
        assert_eq!(request.currency, "USDT");
        assert_eq!(request.direction, "IN");
        assert!(request.client_order_id.is_none());
    }
}
