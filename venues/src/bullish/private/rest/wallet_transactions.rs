//! Wallet transactions endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

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
#[derive(Debug, Clone, Default)]
pub struct GetWalletTransactionsParams {
    /// Trading account ID (required)
    pub trading_account_id: String,
    /// Asset symbol filter
    pub symbol: Option<String>,
    /// Transaction type filter
    pub transaction_type: Option<TransactionType>,
    /// Transaction status filter
    pub status: Option<TransactionStatus>,
    /// Start time filter (timestamp)
    pub start_time: Option<u64>,
    /// End time filter (timestamp)
    pub end_time: Option<u64>,
    /// Page size (default: 100, max: 1000)
    pub page_size: Option<u32>,
    /// Page token for pagination
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
    ) -> RestResult<WalletTransactionsResponse> {
        let mut query_params = vec![("tradingAccountId", params.trading_account_id)];

        if let Some(symbol) = params.symbol {
            query_params.push(("symbol", symbol));
        }
        if let Some(transaction_type) = params.transaction_type {
            query_params.push(("type", format!("{:?}", transaction_type).to_uppercase()));
        }
        if let Some(status) = params.status {
            query_params.push(("status", format!("{:?}", status).to_uppercase()));
        }
        if let Some(start_time) = params.start_time {
            query_params.push(("startTime", start_time.to_string()));
        }
        if let Some(end_time) = params.end_time {
            query_params.push(("endTime", end_time.to_string()));
        }
        if let Some(page_size) = params.page_size {
            query_params.push(("pageSize", page_size.to_string()));
        }
        if let Some(page_token) = params.page_token {
            query_params.push(("pageToken", page_token));
        }

        let mut url = WALLET_TRANSACTIONS_ENDPOINT.to_string();
        if !query_params.is_empty() {
            url.push('?');
            let query_string: Vec<String> = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            url.push_str(&query_string.join("&"));
        }

        self.send_authenticated_request(
            &url,
            reqwest::Method::GET,
            None::<&()>,
            EndpointType::PrivateWalletTransactions,
        )
        .await
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
}
