use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const TRANSFERABLE_ENDPOINT: &str = "/api/v1/accounts/transferable";

/// Request for getting transferable balance
#[derive(Debug, Clone, Serialize)]
pub struct GetTransferableRequest {
    /// Currency code
    pub currency: String,

    /// Account type
    #[serde(rename = "type")]
    pub account_type: String,

    /// Transfer tag (optional)
    pub tag: Option<String>,
}

/// Transferable balance information
#[derive(Debug, Clone, Deserialize)]
pub struct TransferableBalance {
    /// Currency
    pub currency: String,

    /// Available balance for transfer
    pub balance: String,

    /// Available amount
    pub available: String,

    /// Held amount
    pub holds: String,

    /// Transferable amount
    pub transferable: String,
}

impl RestClient {
    /// Get transferable balance for a currency and account type
    ///
    /// Reference: https://docs.kucoin.com/#get-transferable-balance
    pub async fn get_transferable(
        &self,
        request: GetTransferableRequest,
    ) -> Result<(TransferableBalance, ResponseHeaders)> {
        let mut params = HashMap::new();
        params.insert("currency".to_string(), request.currency);
        params.insert("type".to_string(), request.account_type);

        if let Some(tag) = request.tag {
            params.insert("tag".to_string(), tag);
        }

        let (response, headers): (RestResponse<TransferableBalance>, ResponseHeaders) =
            self.get(TRANSFERABLE_ENDPOINT, Some(params)).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transferable_request_creation() {
        let request = GetTransferableRequest {
            currency: "ETH".to_string(),
            account_type: "trade".to_string(),
            tag: Some("test".to_string()),
        };
        assert_eq!(request.currency, "ETH");
        assert_eq!(request.account_type, "trade");
        assert_eq!(request.tag, Some("test".to_string()));
    }

    #[test]
    fn test_transferable_request_minimal() {
        let request = GetTransferableRequest {
            currency: "BTC".to_string(),
            account_type: "main".to_string(),
            tag: None,
        };
        assert_eq!(request.currency, "BTC");
        assert_eq!(request.account_type, "main");
        assert!(request.tag.is_none());
    }
}
