use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for funding accounts
#[derive(Debug, Clone, Serialize, Default)]
pub struct FundingAccountsRequest {
    /// Currency filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Funding account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingAccount {
    /// Currency
    pub currency: String,

    /// Available balance
    pub available: String,

    /// Locked balance
    pub locked: String,

    /// Lent amount
    pub lent: String,

    /// Total lending balance
    pub total_lent: String,
}

/// Request parameters for transferable amount
#[derive(Debug, Clone, Serialize)]
pub struct TransferableRequest {
    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,
}

/// Transferable amount information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferableAmount {
    /// Currency
    pub currency: String,

    /// Available amount for transfer
    pub amount: String,
}

/// Request parameters for borrowable amount
#[derive(Debug, Clone, Serialize)]
pub struct BorrowableRequest {
    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,
}

/// Borrowable amount information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowableAmount {
    /// Currency
    pub currency: String,

    /// Amount available for borrowing
    pub amount: String,
}

/// Request parameters for auto repay settings
#[derive(Debug, Clone, Serialize, Default)]
pub struct AutoRepayRequest {
    /// Status (on/off)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Auto repay settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoRepaySetting {
    /// Auto repay status
    pub status: String,
}

impl RestClient {
    /// Get funding accounts
    ///
    /// This endpoint returns funding account balances for margin trading.
    /// Funding accounts hold assets that can be lent out for margin trading.
    pub async fn get_funding_accounts(
        &self,
        params: FundingAccountsRequest,
    ) -> crate::gateio::Result<Vec<FundingAccount>> {
        self.get_with_query("/margin/funding_accounts", &params)
            .await
    }

    /// Get transferable amount
    ///
    /// This endpoint returns the amount that can be transferred for a specific
    /// currency and currency pair in margin trading.
    pub async fn get_transferable(
        &self,
        params: TransferableRequest,
    ) -> crate::gateio::Result<TransferableAmount> {
        self.get_with_query("/margin/transferable", &params).await
    }

    /// Get borrowable amount
    ///
    /// This endpoint returns the amount that can be borrowed for a specific
    /// currency and currency pair in margin trading.
    pub async fn get_borrowable(
        &self,
        params: BorrowableRequest,
    ) -> crate::gateio::Result<BorrowableAmount> {
        self.get_with_query("/margin/borrowable", &params).await
    }

    /// Get auto repay settings
    ///
    /// This endpoint returns the current auto repay settings for margin trading.
    pub async fn get_auto_repay(&self) -> crate::gateio::Result<AutoRepaySetting> {
        self.get("/margin/auto_repay").await
    }

    /// Update auto repay settings
    ///
    /// This endpoint updates the auto repay settings for margin trading.
    pub async fn update_auto_repay(
        &self,
        params: AutoRepayRequest,
    ) -> crate::gateio::Result<AutoRepaySetting> {
        self.post("/margin/auto_repay", &params).await
    }
}
