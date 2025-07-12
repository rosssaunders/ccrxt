use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for total balance
#[derive(Debug, Clone, Serialize, Default)]
pub struct TotalBalanceRequest {
    /// Currency filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Total balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalBalance {
    /// Details of each currency
    pub details: std::collections::HashMap<String, CurrencyBalance>,

    /// Total balance in USDT
    pub total: TotalBalanceValue,
}

/// Currency balance details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyBalance {
    /// Available balance
    pub available: String,

    /// Unrealized PnL
    pub unrealised_pnl: String,

    /// Borrowed amount
    pub borrowed: String,
}

/// Total balance value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalBalanceValue {
    /// Currency (usually USDT)
    pub currency: String,

    /// Total amount
    pub amount: String,
}

/// Request parameters for deposit address
#[derive(Debug, Clone, Serialize)]
pub struct DepositAddressRequest {
    /// Currency
    pub currency: String,
}

/// Deposit address information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositAddress {
    /// Currency
    pub currency: String,

    /// Deposit address
    pub address: String,

    /// Address name
    pub multichain_addresses: Vec<MultichainAddress>,
}

/// Multichain address information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultichainAddress {
    /// Chain name
    pub chain: String,

    /// Address
    pub address: String,

    /// Payment ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,

    /// Payment name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_name: Option<String>,

    /// Obtain failed reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obtain_failed: Option<String>,
}

/// Request parameters for deposits
#[derive(Debug, Clone, Serialize, Default)]
pub struct DepositsRequest {
    /// Currency filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Start time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Maximum number of records to return (1-100, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Deposit record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositRecord {
    /// Deposit ID
    pub id: String,

    /// Transaction ID
    pub txid: String,

    /// Currency
    pub currency: String,

    /// Chain
    pub chain: String,

    /// Amount
    pub amount: String,

    /// Address
    pub address: String,

    /// Payment ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,

    /// Status
    pub status: String,

    /// Timestamp
    pub timestamp: String,
}

/// Request parameters for withdrawals
#[derive(Debug, Clone, Serialize, Default)]
pub struct WithdrawalsRequest {
    /// Currency filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Start time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Maximum number of records to return (1-100, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Withdrawal record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalRecord {
    /// Withdrawal ID
    pub id: String,

    /// Transaction ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,

    /// Currency
    pub currency: String,

    /// Chain
    pub chain: String,

    /// Amount
    pub amount: String,

    /// Fee
    pub fee: String,

    /// Address
    pub address: String,

    /// Payment ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,

    /// Status
    pub status: String,

    /// Timestamp
    pub timestamp: String,
}

/// Request to create a transfer
#[derive(Debug, Clone, Serialize)]
pub struct CreateTransferRequest {
    /// Currency
    pub currency: String,

    /// From account (spot, margin, futures, delivery, cross_margin, options)
    pub from: String,

    /// To account (spot, margin, futures, delivery, cross_margin, options)
    pub to: String,

    /// Transfer amount
    pub amount: String,

    /// Currency pair (for margin transfers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Settle currency (for futures/delivery transfers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle: Option<String>,
}

/// Transfer record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRecord {
    /// Currency
    pub currency: String,

    /// From account
    pub from: String,

    /// To account
    pub to: String,

    /// Transfer amount
    pub amount: String,

    /// Currency pair
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Settle currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle: Option<String>,
}

/// Request parameters for withdrawal fees
#[derive(Debug, Clone, Serialize, Default)]
pub struct WithdrawalFeesRequest {
    /// Currency filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Withdrawal fee information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalFee {
    /// Currency
    pub currency: String,

    /// Minimum withdrawal amount
    pub min_amount: String,

    /// Maximum withdrawal amount
    pub max_amount: String,

    /// Fixed fee amount
    pub fixed: String,

    /// Percentage fee
    pub percent: String,
}

impl RestClient {
    /// Get total balance
    ///
    /// This endpoint returns the total balance across all accounts.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#query-account-book>
    pub async fn get_total_balance(
        &self,
        params: TotalBalanceRequest,
    ) -> crate::gateio::Result<TotalBalance> {
        self.get_with_query("/wallet/total_balance", &params).await
    }

    /// Get deposit address
    ///
    /// This endpoint returns the deposit address for a specific currency.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#generate-currency-deposit-address>
    pub async fn get_deposit_address(
        &self,
        params: DepositAddressRequest,
    ) -> crate::gateio::Result<DepositAddress> {
        self.get_with_query("/wallet/deposit_address", &params)
            .await
    }

    /// Get deposit history
    ///
    /// This endpoint returns the deposit history for the authenticated user.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#retrieve-deposit-records>
    pub async fn get_deposits(
        &self,
        params: DepositsRequest,
    ) -> crate::gateio::Result<Vec<DepositRecord>> {
        self.get_with_query("/wallet/deposits", &params).await
    }

    /// Get withdrawal history
    ///
    /// This endpoint returns the withdrawal history for the authenticated user.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#retrieve-withdrawal-records>
    pub async fn get_withdrawals(
        &self,
        params: WithdrawalsRequest,
    ) -> crate::gateio::Result<Vec<WithdrawalRecord>> {
        self.get_with_query("/wallet/withdrawals", &params).await
    }

    /// Create a transfer
    ///
    /// This endpoint creates a transfer between different accounts.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#transfer-between-trading-accounts>
    pub async fn create_transfer(
        &self,
        request: CreateTransferRequest,
    ) -> crate::gateio::Result<TransferRecord> {
        self.post("/wallet/transfers", &request).await
    }

    /// Get withdrawal fees
    ///
    /// This endpoint returns withdrawal fee information for currencies.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#retrieve-withdrawal-status>
    pub async fn get_withdrawal_fees(
        &self,
        params: WithdrawalFeesRequest,
    ) -> crate::gateio::Result<Vec<WithdrawalFee>> {
        self.get_with_query("/wallet/fee", &params).await
    }
}
