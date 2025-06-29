use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::kucoin::{ResponseHeaders, RestResponse, Result};

use super::RestClient;

/// Request for getting deposit addresses
#[derive(Debug, Clone, Serialize)]
pub struct GetDepositAddressRequest {
    /// Currency code
    pub currency: String,
    /// Chain name (optional, e.g., "eth", "bsc")
    pub chain: Option<String>,
}

/// Deposit address information
#[derive(Debug, Clone, Deserialize)]
pub struct DepositAddress {
    /// Deposit address
    pub address: String,
    /// Address memo/tag (for some currencies)
    pub memo: Option<String>,
    /// Chain name
    pub chain: String,
    /// Contract address (for tokens)
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<String>,
}

/// Request for getting deposit history
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetDepositsRequest {
    /// Currency filter (optional)
    pub currency: Option<String>,
    /// Start time filter (optional, milliseconds)
    #[serde(rename = "startAt")]
    pub start_time: Option<i64>,
    /// End time filter (optional, milliseconds)
    #[serde(rename = "endAt")]
    pub end_time: Option<i64>,
    /// Status filter (optional): PROCESSING, SUCCESS, FAILURE
    pub status: Option<String>,
}

/// Deposit record
#[derive(Debug, Clone, Deserialize)]
pub struct Deposit {
    /// Deposit address
    pub address: String,
    /// Address memo/tag
    pub memo: Option<String>,
    /// Amount
    pub amount: String,
    /// Fee
    pub fee: String,
    /// Currency
    pub currency: String,
    /// Chain
    pub chain: String,
    /// Wallet transaction ID
    #[serde(rename = "walletTxId")]
    pub wallet_tx_id: String,
    /// Is internal transfer
    #[serde(rename = "isInner")]
    pub is_inner: bool,
    /// Status
    pub status: String,
    /// Remark
    pub remark: Option<String>,
    /// Creation time
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    /// Update time
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

/// Response wrapper for deposits
#[derive(Debug, Clone, Deserialize)]
pub struct DepositsResponse {
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
    /// Deposit items
    pub items: Vec<Deposit>,
}

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

/// Request for getting withdrawal history
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetWithdrawalsRequest {
    /// Currency filter (optional)
    pub currency: Option<String>,
    /// Start time filter (optional, milliseconds)
    #[serde(rename = "startAt")]
    pub start_time: Option<i64>,
    /// End time filter (optional, milliseconds)
    #[serde(rename = "endAt")]
    pub end_time: Option<i64>,
    /// Status filter (optional): PROCESSING, WALLET_PROCESSING, SUCCESS, FAILURE
    pub status: Option<String>,
}

/// Withdrawal record
#[derive(Debug, Clone, Deserialize)]
pub struct Withdrawal {
    /// Withdrawal ID
    pub id: String,
    /// Withdrawal address
    pub address: String,
    /// Address memo/tag
    pub memo: Option<String>,
    /// Currency
    pub currency: String,
    /// Chain
    pub chain: String,
    /// Amount
    pub amount: String,
    /// Fee
    pub fee: String,
    /// Wallet transaction ID
    #[serde(rename = "walletTxId")]
    pub wallet_tx_id: Option<String>,
    /// Is internal transfer
    #[serde(rename = "isInner")]
    pub is_inner: bool,
    /// Status
    pub status: String,
    /// Remark
    pub remark: Option<String>,
    /// Creation time
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    /// Update time
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

/// Response wrapper for withdrawals
#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawalsResponse {
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
    /// Withdrawal items
    pub items: Vec<Withdrawal>,
}

/// Request for getting withdrawal quotas
#[derive(Debug, Clone, Serialize)]
pub struct GetWithdrawalQuotasRequest {
    /// Currency code
    pub currency: String,
    /// Chain name (optional)
    pub chain: Option<String>,
}

/// Withdrawal quota information
#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawalQuota {
    /// Currency
    pub currency: String,
    /// Chain
    pub chain: String,
    /// Available amount for withdrawal
    #[serde(rename = "availableAmount")]
    pub available_amount: String,
    /// Remaining daily quota
    #[serde(rename = "remainAmount")]
    pub remain_amount: String,
    /// Withdrawal minimum amount
    #[serde(rename = "withdrawMinSize")]
    pub withdraw_min_size: String,
    /// Limitation of amount
    #[serde(rename = "limitBTCAmount")]
    pub limit_btc_amount: String,
    /// Inner transfer minimum fee
    #[serde(rename = "innerWithdrawMinFee")]
    pub inner_withdraw_min_fee: String,
    /// KuCoin withdrawal fee
    #[serde(rename = "withdrawMinFee")]
    pub withdraw_min_fee: String,
    /// Is withdrawal enabled
    #[serde(rename = "isWithdrawEnabled")]
    pub is_withdraw_enabled: bool,
    /// Withdrawal precision
    pub precision: i32,
}

impl RestClient {
    /// Get deposit address for a currency
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, GetDepositAddressRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_with_credentials("api_key", "api_secret", "api_passphrase");
    ///     let request = GetDepositAddressRequest {
    ///         currency: "BTC".to_string(),
    ///         chain: None,
    ///     };
    ///     let (address, _headers) = client.get_deposit_address(request).await?;
    ///     println!("Deposit address: {}", address.address);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_deposit_address(
        &self,
        request: GetDepositAddressRequest,
    ) -> Result<(DepositAddress, ResponseHeaders)> {
        let mut params = HashMap::new();
        params.insert("currency".to_string(), request.currency);
        
        if let Some(chain) = request.chain {
            params.insert("chain".to_string(), chain);
        }

        let (response, headers): (RestResponse<DepositAddress>, ResponseHeaders) =
            self.get("/api/v2/deposit-addresses", Some(params)).await?;

        Ok((response.data, headers))
    }

    /// Get deposit history
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, GetDepositsRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_with_credentials("api_key", "api_secret", "api_passphrase");
    ///     let request = GetDepositsRequest {
    ///         currency: Some("BTC".to_string()),
    ///         status: Some("SUCCESS".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let (deposits, _headers) = client.get_deposits(request).await?;
    ///     println!("Found {} deposits", deposits.items.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_deposits(
        &self,
        request: GetDepositsRequest,
    ) -> Result<(DepositsResponse, ResponseHeaders)> {
        let mut params = HashMap::new();
        
        if let Some(currency) = request.currency {
            params.insert("currency".to_string(), currency);
        }
        if let Some(start_time) = request.start_time {
            params.insert("startAt".to_string(), start_time.to_string());
        }
        if let Some(end_time) = request.end_time {
            params.insert("endAt".to_string(), end_time.to_string());
        }
        if let Some(status) = request.status {
            params.insert("status".to_string(), status);
        }

        let (response, headers): (RestResponse<DepositsResponse>, ResponseHeaders) =
            self.get("/api/v1/deposits", Some(params)).await?;

        Ok((response.data, headers))
    }

    /// Create a withdrawal
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, CreateWithdrawalRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_with_credentials("api_key", "api_secret", "api_passphrase");
    ///     let request = CreateWithdrawalRequest {
    ///         currency: "BTC".to_string(),
    ///         address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
    ///         amount: "0.001".to_string(),
    ///         memo: None,
    ///         is_inner: Some(false),
    ///         remark: Some("Test withdrawal".to_string()),
    ///         chain: None,
    ///         fee_deduct_type: None,
    ///     };
    ///     let (withdrawal, _headers) = client.create_withdrawal(request).await?;
    ///     println!("Withdrawal ID: {}", withdrawal.withdrawal_id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_withdrawal(
        &self,
        request: CreateWithdrawalRequest,
    ) -> Result<(WithdrawalResponse, ResponseHeaders)> {
        let body = serde_json::to_string(&request)
            .map_err(|e| crate::kucoin::ApiError::JsonParsing(format!("Failed to serialize request: {}", e)))?;

        let (response, headers): (RestResponse<WithdrawalResponse>, ResponseHeaders) =
            self.post("/api/v1/withdrawals", &body).await?;

        Ok((response.data, headers))
    }

    /// Get withdrawal history
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, GetWithdrawalsRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_with_credentials("api_key", "api_secret", "api_passphrase");
    ///     let request = GetWithdrawalsRequest {
    ///         currency: Some("BTC".to_string()),
    ///         status: Some("SUCCESS".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let (withdrawals, _headers) = client.get_withdrawals(request).await?;
    ///     println!("Found {} withdrawals", withdrawals.items.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_withdrawals(
        &self,
        request: GetWithdrawalsRequest,
    ) -> Result<(WithdrawalsResponse, ResponseHeaders)> {
        let mut params = HashMap::new();
        
        if let Some(currency) = request.currency {
            params.insert("currency".to_string(), currency);
        }
        if let Some(start_time) = request.start_time {
            params.insert("startAt".to_string(), start_time.to_string());
        }
        if let Some(end_time) = request.end_time {
            params.insert("endAt".to_string(), end_time.to_string());
        }
        if let Some(status) = request.status {
            params.insert("status".to_string(), status);
        }

        let (response, headers): (RestResponse<WithdrawalsResponse>, ResponseHeaders) =
            self.get("/api/v1/withdrawals", Some(params)).await?;

        Ok((response.data, headers))
    }

    /// Get withdrawal quotas for a currency
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, GetWithdrawalQuotasRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_with_credentials("api_key", "api_secret", "api_passphrase");
    ///     let request = GetWithdrawalQuotasRequest {
    ///         currency: "BTC".to_string(),
    ///         chain: None,
    ///     };
    ///     let (quota, _headers) = client.get_withdrawal_quotas(request).await?;
    ///     println!("Available amount: {}", quota.available_amount);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_withdrawal_quotas(
        &self,
        request: GetWithdrawalQuotasRequest,
    ) -> Result<(WithdrawalQuota, ResponseHeaders)> {
        let mut params = HashMap::new();
        params.insert("currency".to_string(), request.currency);
        
        if let Some(chain) = request.chain {
            params.insert("chain".to_string(), chain);
        }

        let (response, headers): (RestResponse<WithdrawalQuota>, ResponseHeaders) =
            self.get("/api/v1/withdrawals/quotas", Some(params)).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit_address_request_creation() {
        let request = GetDepositAddressRequest {
            currency: "BTC".to_string(),
            chain: Some("btc".to_string()),
        };
        assert_eq!(request.currency, "BTC");
        assert_eq!(request.chain, Some("btc".to_string()));
    }

    #[test]
    fn test_deposits_request_default() {
        let request = GetDepositsRequest::default();
        assert!(request.currency.is_none());
        assert!(request.status.is_none());
    }

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
