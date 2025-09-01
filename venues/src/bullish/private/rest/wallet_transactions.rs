//! Wallet transactions endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use crate::bullish::{EndpointType, PaginatedResult, PrivateRestClient as RestClient, RestResult};

/// Endpoint URL path for wallet transactions
const WALLET_TRANSACTIONS_ENDPOINT: &str = "/v1/wallets/transactions";

/// Transaction status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

/// Transaction type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
    Trade,
    Fee,
    Interest,
    Rebate,
    Liquidation,
}

/// Wallet transaction details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletTransaction {
    /// Unique transaction ID
    #[serde(rename = "transactionId")]
    pub transaction_id: String,

    /// Asset symbol
    pub symbol: String,

    /// Transaction amount (positive for credits, negative for debits)
    pub amount: String,

    /// Running balance after this transaction
    pub balance: String,

    /// Transaction type
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,

    /// Transaction status
    pub status: TransactionStatus,

    /// Transaction description
    pub description: String,

    /// Reference ID (e.g., order ID, trade ID)
    #[serde(rename = "referenceId")]
    pub reference_id: Option<String>,

    /// Transaction fee
    pub fee: Option<String>,

    /// Transaction timestamp
    #[serde(rename = "createdAtTimestamp")]
    pub created_at_timestamp: u64,

    /// Transaction datetime
    #[serde(rename = "createdAtDatetime")]
    pub created_at_datetime: String,

    /// Network confirmation details (for crypto transactions)
    #[serde(rename = "networkConfirmations")]
    pub network_confirmations: Option<i32>,

    /// Required confirmations (for crypto transactions)
    #[serde(rename = "requiredConfirmations")]
    pub required_confirmations: Option<i32>,

    /// Transaction hash (for crypto transactions)
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
}

/// Parameters for querying wallet transactions
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletTransactionsParams {
    /// Trading account ID (required)
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,

    /// Asset symbol filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Transaction type filter
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<TransactionType>,

    /// Transaction status filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TransactionStatus>,

    /// Start time filter (timestamp)
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time filter (timestamp)
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Page size (default: 100, max: 1000)
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,

    /// Page token for pagination
    #[serde(rename = "pageToken", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// Response for wallet transactions query
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletTransactionsResponse {
    /// List of transactions
    pub data: Vec<WalletTransaction>,

    /// Next page token for pagination
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

impl RestClient {
    /// Get wallet transactions with optional filters
    ///
    /// Retrieve a list of wallet transactions for a trading account.
    /// Supports pagination and various filtering options.
    ///
    /// # Arguments
    /// * `params` - Query parameters for filtering transactions
    ///
    /// # Returns
    /// Paginated list of wallet transactions
    pub async fn get_wallet_transactions(
        &mut self,
        params: GetWalletTransactionsParams,
    ) -> RestResult<PaginatedResult<WalletTransaction>> {
        let wire: WalletTransactionsResponse = self
            .send_get_authenticated_request(
                WALLET_TRANSACTIONS_ENDPOINT,
                params,
                EndpointType::PrivateCustody,
            )
            .await?;

        Ok(PaginatedResult::Token {
            data: wire.data,
            next_page_token: wire.next_page_token,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_type_serialization() {
        assert_eq!(
            serde_json::to_string(&TransactionType::Deposit).unwrap(),
            "\"DEPOSIT\""
        );
        assert_eq!(
            serde_json::to_string(&TransactionType::Withdrawal).unwrap(),
            "\"WITHDRAWAL\""
        );
        assert_eq!(
            serde_json::to_string(&TransactionType::Trade).unwrap(),
            "\"TRADE\""
        );
    }

    #[test]
    fn test_transaction_status_serialization() {
        assert_eq!(
            serde_json::to_string(&TransactionStatus::Pending).unwrap(),
            "\"PENDING\""
        );
        assert_eq!(
            serde_json::to_string(&TransactionStatus::Completed).unwrap(),
            "\"COMPLETED\""
        );
        assert_eq!(
            serde_json::to_string(&TransactionStatus::Failed).unwrap(),
            "\"FAILED\""
        );
    }

    #[test]
    fn test_get_wallet_transactions_params_default() {
        let params = GetWalletTransactionsParams::default();
        assert!(params.trading_account_id.is_empty());
        assert!(params.symbol.is_none());
        assert!(params.transaction_type.is_none());
        assert!(params.status.is_none());
        assert!(params.start_time.is_none());
        assert!(params.end_time.is_none());
        assert!(params.page_size.is_none());
        assert!(params.page_token.is_none());
    }

    #[test]
    fn test_get_wallet_transactions_params_serialization() {
        let params = GetWalletTransactionsParams {
            trading_account_id: "acct".to_string(),
            symbol: Some("BTC".to_string()),
            transaction_type: Some(TransactionType::Deposit),
            status: Some(TransactionStatus::Completed),
            start_time: Some(1),
            end_time: Some(2),
            page_size: Some(100),
            page_token: Some("token".to_string()),
        };

        let qs = serde_urlencoded::to_string(&params).unwrap();
        assert!(qs.contains("tradingAccountId=acct"));
        assert!(qs.contains("symbol=BTC"));
        assert!(qs.contains("type=DEPOSIT"));
        assert!(qs.contains("status=COMPLETED"));
        assert!(qs.contains("startTime=1"));
        assert!(qs.contains("endTime=2"));
        assert!(qs.contains("pageSize=100"));
        assert!(qs.contains("pageToken=token"));
    }
}
