use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const CREATE_INNER_TRANSFER_ENDPOINT: &str = "/api/v2/accounts/inner-transfer";

/// Request for creating an inner transfer between accounts
#[derive(Debug, Clone, Serialize)]
pub struct CreateInnerTransferRequest {
    /// Client order ID for the transfer (optional, max 40 characters)
    #[serde(rename = "clientOid")]
    pub client_order_id: Option<String>,

    /// Currency to transfer
    pub currency: String,

    /// Transfer from account type
    pub from: String,

    /// Transfer to account type  
    pub to: String,

    /// Transfer amount
    pub amount: String,

    /// Transfer from account ID (optional, for sub-accounts)
    #[serde(rename = "fromAccountId")]
    pub from_account_id: Option<String>,

    /// Transfer to account ID (optional, for sub-accounts)
    #[serde(rename = "toAccountId")]
    pub to_account_id: Option<String>,
}

/// Inner transfer response
#[derive(Debug, Clone, Deserialize)]
pub struct InnerTransferResponse {
    /// Transfer order ID
    #[serde(rename = "orderId")]
    pub order_id: String,
}

impl RestClient {
    /// Create an inner transfer between accounts
    ///
    /// [docs](https://docs.kucoin.com/#inner-transfer)
    pub async fn create_inner_transfer(
        &self,
        request: CreateInnerTransferRequest,
    ) -> Result<(InnerTransferResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<InnerTransferResponse>, ResponseHeaders) = self
            .post_with_request(CREATE_INNER_TRANSFER_ENDPOINT, &request)
            .await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inner_transfer_request_creation() {
        let request = CreateInnerTransferRequest {
            client_order_id: Some("test_transfer".to_string()),
            currency: "USDT".to_string(),
            from: "main".to_string(),
            to: "trade".to_string(),
            amount: "100.0".to_string(),
            from_account_id: None,
            to_account_id: None,
        };
        assert_eq!(request.currency, "USDT");
        assert_eq!(request.from, "main");
        assert_eq!(request.to, "trade");
        assert_eq!(request.amount, "100.0");
    }

    #[test]
    fn test_inner_transfer_request_minimal() {
        let request = CreateInnerTransferRequest {
            client_order_id: None,
            currency: "BTC".to_string(),
            from: "main".to_string(),
            to: "trade".to_string(),
            amount: "0.001".to_string(),
            from_account_id: None,
            to_account_id: None,
        };
        assert_eq!(request.currency, "BTC");
        assert!(request.client_order_id.is_none());
    }
}
