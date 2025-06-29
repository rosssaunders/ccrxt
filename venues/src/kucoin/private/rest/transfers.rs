use serde::{Deserialize, Serialize};

use crate::kucoin::{ResponseHeaders, RestResponse, Result};

use super::RestClient;

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

/// Request for getting inner transfer history
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetInnerTransfersRequest {
    /// Currency filter (optional)
    pub currency: Option<String>,
    /// Transfer from account type filter (optional)
    pub from: Option<String>,
    /// Transfer to account type filter (optional)
    pub to: Option<String>,
    /// Order ID filter (optional)
    #[serde(rename = "orderId")]
    pub order_id: Option<String>,
    /// Start time filter (optional, milliseconds)
    #[serde(rename = "startAt")]
    pub start_time: Option<i64>,
    /// End time filter (optional, milliseconds)
    #[serde(rename = "endAt")]
    pub end_time: Option<i64>,
}

/// Inner transfer record
#[derive(Debug, Clone, Deserialize)]
pub struct InnerTransfer {
    /// Transfer order ID
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// Currency
    pub currency: String,
    /// Transfer amount
    pub amount: String,
    /// Transfer from account type
    pub from: String,
    /// Transfer to account type
    pub to: String,
    /// Transfer from account ID
    #[serde(rename = "fromAccountId")]
    pub from_account_id: Option<String>,
    /// Transfer to account ID
    #[serde(rename = "toAccountId")]
    pub to_account_id: Option<String>,
    /// Transfer status
    pub status: String,
    /// Creation time
    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

/// Response wrapper for inner transfers
#[derive(Debug, Clone, Deserialize)]
pub struct InnerTransfersResponse {
    /// Current page
    #[serde(rename = "currentPage")]
    pub current_page: i32,
    /// Page size
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    /// Total number of records
    #[serde(rename = "totalNum")]
    pub total_num: i32,
    /// Total pages
    #[serde(rename = "totalPage")]
    pub total_page: i32,
    /// Transfer items
    pub items: Vec<InnerTransfer>,
}

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
    /// Create an inner transfer between accounts
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, CreateInnerTransferRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_with_credentials("api_key", "api_secret", "api_passphrase");
    ///     let request = CreateInnerTransferRequest {
    ///         client_order_id: Some("transfer_001".to_string()),
    ///         currency: "USDT".to_string(),
    ///         from: "main".to_string(),
    ///         to: "trade".to_string(),
    ///         amount: "100.0".to_string(),
    ///         from_account_id: None,
    ///         to_account_id: None,
    ///     };
    ///     let (transfer, _headers) = client.create_inner_transfer(request).await?;
    ///     println!("Transfer order ID: {}", transfer.order_id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_inner_transfer(
        &self,
        request: CreateInnerTransferRequest,
    ) -> Result<(InnerTransferResponse, ResponseHeaders)> {
        let body = serde_json::to_string(&request)
            .map_err(|e| crate::kucoin::ApiError::JsonParsing(format!("Failed to serialize request: {}", e)))?;

        let (response, headers): (RestResponse<InnerTransferResponse>, ResponseHeaders) =
            self.post("/api/v2/accounts/inner-transfer", &body).await?;

        Ok((response.data, headers))
    }

    /// Get inner transfer history
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, GetInnerTransfersRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_with_credentials("api_key", "api_secret", "api_passphrase");
    ///     let request = GetInnerTransfersRequest {
    ///         currency: Some("USDT".to_string()),
    ///         from: Some("main".to_string()),
    ///         to: Some("trade".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let (transfers, _headers) = client.get_inner_transfers(request).await?;
    ///     println!("Found {} transfers", transfers.items.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_inner_transfers(
        &self,
        request: GetInnerTransfersRequest,
    ) -> Result<(InnerTransfersResponse, ResponseHeaders)> {
        let mut params = std::collections::HashMap::new();
        
        if let Some(currency) = request.currency {
            params.insert("currency".to_string(), currency);
        }
        if let Some(from) = request.from {
            params.insert("from".to_string(), from);
        }
        if let Some(to) = request.to {
            params.insert("to".to_string(), to);
        }
        if let Some(order_id) = request.order_id {
            params.insert("orderId".to_string(), order_id);
        }
        if let Some(start_time) = request.start_time {
            params.insert("startAt".to_string(), start_time.to_string());
        }
        if let Some(end_time) = request.end_time {
            params.insert("endAt".to_string(), end_time.to_string());
        }

        let (response, headers): (RestResponse<InnerTransfersResponse>, ResponseHeaders) =
            self.get("/api/v1/accounts/transferable", Some(params)).await?;

        Ok((response.data, headers))
    }

    /// Create a sub-account transfer (main account only)
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, CreateSubTransferRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_with_credentials("api_key", "api_secret", "api_passphrase");
    ///     let request = CreateSubTransferRequest {
    ///         client_order_id: Some("sub_transfer_001".to_string()),
    ///         currency: "USDT".to_string(),
    ///         amount: "50.0".to_string(),
    ///         direction: "OUT".to_string(),
    ///         account_type: Some("main".to_string()),
    ///         sub_user_id: Some("sub_account_id".to_string()),
    ///         sub_account_type: Some("trade".to_string()),
    ///     };
    ///     let (transfer, _headers) = client.create_sub_transfer(request).await?;
    ///     println!("Sub-transfer order ID: {}", transfer.order_id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_sub_transfer(
        &self,
        request: CreateSubTransferRequest,
    ) -> Result<(SubTransferResponse, ResponseHeaders)> {
        let body = serde_json::to_string(&request)
            .map_err(|e| crate::kucoin::ApiError::JsonParsing(format!("Failed to serialize request: {}", e)))?;

        let (response, headers): (RestResponse<SubTransferResponse>, ResponseHeaders) =
            self.post("/api/v2/accounts/sub-transfer", &body).await?;

        Ok((response.data, headers))
    }

    /// Get transferable balance for a currency and account type
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, GetTransferableRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_with_credentials("api_key", "api_secret", "api_passphrase");
    ///     let request = GetTransferableRequest {
    ///         currency: "USDT".to_string(),
    ///         account_type: "main".to_string(),
    ///         tag: None,
    ///     };
    ///     let (balance, _headers) = client.get_transferable(request).await?;
    ///     println!("Transferable amount: {}", balance.transferable);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_transferable(
        &self,
        request: GetTransferableRequest,
    ) -> Result<(TransferableBalance, ResponseHeaders)> {
        let mut params = std::collections::HashMap::new();
        params.insert("currency".to_string(), request.currency);
        params.insert("type".to_string(), request.account_type);
        
        if let Some(tag) = request.tag {
            params.insert("tag".to_string(), tag);
        }

        let (response, headers): (RestResponse<TransferableBalance>, ResponseHeaders) =
            self.get("/api/v1/accounts/transferable", Some(params)).await?;

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
    fn test_inner_transfers_request_default() {
        let request = GetInnerTransfersRequest::default();
        assert!(request.currency.is_none());
        assert!(request.from.is_none());
        assert!(request.to.is_none());
    }

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
}
