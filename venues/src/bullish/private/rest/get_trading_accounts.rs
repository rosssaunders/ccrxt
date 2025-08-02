//! Get trading accounts endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

/// Endpoint URL path for trading accounts
const TRADING_ACCOUNTS_ENDPOINT: &str = "/v1/accounts/trading-accounts";

/// Endpoint URL path for single trading account (with parameter)
const SINGLE_TRADING_ACCOUNT_ENDPOINT: &str = "/v1/accounts/trading-accounts/{}";

/// Trading account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingAccount {
    /// Unique trading account identifier
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,

    /// Total collateral across all assets in USD
    #[serde(rename = "totalCollateralUSD")]
    pub total_collateral_usd: String,

    /// Total borrowed across all assets in USD
    #[serde(rename = "totalBorrowedUSD")]
    pub total_borrowed_usd: String,

    /// The value of margin when this account will be moved into defaulted state
    #[serde(rename = "defaultedMarginUSD")]
    pub defaulted_margin_usd: String,

    /// Maximum allowed borrowing for this account in USD
    #[serde(rename = "riskLimitUSD")]
    pub risk_limit_usd: String,

    /// Total liabilities for this account in USD
    #[serde(rename = "totalLiabilitiesUSD")]
    pub total_liabilities_usd: String,

    /// Whether this is the primary trading account
    #[serde(rename = "isPrimaryAccount")]
    pub is_primary_account: bool,

    /// Whether this account is borrowing any asset
    #[serde(rename = "isBorrowing")]
    pub is_borrowing: bool,

    /// Whether this account has any open loan offers
    #[serde(rename = "isLending")]
    pub is_lending: bool,

    /// Whether this account is in a defaulted state
    #[serde(rename = "isDefaulted")]
    pub is_defaulted: bool,

    /// Rate limit token for higher rate limits
    #[serde(rename = "rateLimitToken")]
    pub rate_limit_token: Option<String>,

    /// Trading fee rates for this account
    #[serde(rename = "tradeFeeRate")]
    pub trade_fee_rate: Option<TradeFeeRate>,
}

/// Trading fee rate information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeFeeRate {
    /// Maker fee rate
    #[serde(rename = "makerFeeRate")]
    pub maker_fee_rate: String,

    /// Taker fee rate
    #[serde(rename = "takerFeeRate")]
    pub taker_fee_rate: String,
}

/// Response for get trading accounts endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingAccountsResponse {
    /// Array of trading account data
    pub data: Vec<TradingAccount>,
}

impl RestClient {
    /// Get all trading accounts
    ///
    /// Gets details for all trading accounts accessible by the API key used in the request.
    /// This endpoint provides balance and account status information equivalent to a
    /// "get balances" endpoint.
    ///
    /// See: Bullish API documentation for /v1/accounts/trading-accounts
    ///
    /// # Returns
    /// Trading accounts information including balances, borrowing status, and fee rates
    pub async fn get_trading_accounts(&mut self) -> RestResult<TradingAccountsResponse> {
        self.send_get_authenticated_request(TRADING_ACCOUNTS_ENDPOINT, (),
            EndpointType::PrivateTradingAccounts,
        )
        .await
    }

    /// Get specific trading account by ID
    ///
    /// Gets details for a specific trading account by trading account ID.
    ///
    /// # Arguments
    /// * `trading_account_id` - The trading account ID to retrieve
    ///
    /// # Returns
    /// Trading account information for the specified account
    pub async fn get_trading_account(
        &mut self,
        trading_account_id: &str,
    ) -> RestResult<TradingAccount> {
        let endpoint = SINGLE_TRADING_ACCOUNT_ENDPOINT.replace("{}", trading_account_id);

        self.send_get_authenticated_request(&endpoint, (),
            EndpointType::PrivateTradingAccounts,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_trading_account_structure() {
        let account_json = json!({
            "tradingAccountId": "111234567890",
            "totalCollateralUSD": "10000.50",
            "totalBorrowedUSD": "5000.25",
            "defaultedMarginUSD": "1000.00",
            "riskLimitUSD": "100000.00",
            "totalLiabilitiesUSD": "5500.75",
            "isPrimaryAccount": true,
            "isBorrowing": true,
            "isLending": false,
            "isDefaulted": false,
            "rateLimitToken": "abcd1234efgh5678",
            "tradeFeeRate": {
                "makerFeeRate": "0.001",
                "takerFeeRate": "0.002"
            }
        });

        let account: TradingAccount = serde_json::from_value(account_json).unwrap();

        assert_eq!(account.trading_account_id, "111234567890");
        assert_eq!(account.total_collateral_usd, "10000.50");
        assert_eq!(account.total_borrowed_usd, "5000.25");
        assert!(account.is_primary_account);
        assert!(account.is_borrowing);
        assert!(!account.is_lending);
        assert!(!account.is_defaulted);
        assert_eq!(
            account.rate_limit_token,
            Some("abcd1234efgh5678".to_string())
        );

        let fee_rate = account.trade_fee_rate.unwrap();
        assert_eq!(fee_rate.maker_fee_rate, "0.001");
        assert_eq!(fee_rate.taker_fee_rate, "0.002");
    }

    #[test]
    fn test_trading_account_minimal_structure() {
        let account_json = json!({
            "tradingAccountId": "111234567890",
            "totalCollateralUSD": "10000.50",
            "totalBorrowedUSD": "0.00",
            "defaultedMarginUSD": "1000.00",
            "riskLimitUSD": "100000.00",
            "totalLiabilitiesUSD": "0.00",
            "isPrimaryAccount": false,
            "isBorrowing": false,
            "isLending": false,
            "isDefaulted": false
        });

        let account: TradingAccount = serde_json::from_value(account_json).unwrap();

        assert_eq!(account.trading_account_id, "111234567890");
        assert!(!account.is_primary_account);
        assert!(!account.is_borrowing);
        assert!(!account.is_lending);
        assert!(!account.is_defaulted);
        assert_eq!(account.rate_limit_token, None);
        assert!(account.trade_fee_rate.is_none());
    }

    #[test]
    fn test_trading_accounts_response_structure() {
        let response_json = json!({
            "data": [
                {
                    "tradingAccountId": "111234567890",
                    "totalCollateralUSD": "10000.50",
                    "totalBorrowedUSD": "0.00",
                    "defaultedMarginUSD": "1000.00",
                    "riskLimitUSD": "100000.00",
                    "totalLiabilitiesUSD": "0.00",
                    "isPrimaryAccount": true,
                    "isBorrowing": false,
                    "isLending": false,
                    "isDefaulted": false
                },
                {
                    "tradingAccountId": "111234567891",
                    "totalCollateralUSD": "5000.25",
                    "totalBorrowedUSD": "1000.00",
                    "defaultedMarginUSD": "500.00",
                    "riskLimitUSD": "50000.00",
                    "totalLiabilitiesUSD": "1100.00",
                    "isPrimaryAccount": false,
                    "isBorrowing": true,
                    "isLending": false,
                    "isDefaulted": false
                }
            ]
        });

        let response: TradingAccountsResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.data.len(), 2);

        let first_account = &response.data[0];
        assert_eq!(first_account.trading_account_id, "111234567890");
        assert!(first_account.is_primary_account);
        assert!(!first_account.is_borrowing);

        let second_account = &response.data[1];
        assert_eq!(second_account.trading_account_id, "111234567891");
        assert!(!second_account.is_primary_account);
        assert!(second_account.is_borrowing);
    }

    #[test]
    fn test_trade_fee_rate_structure() {
        let fee_rate_json = json!({
            "makerFeeRate": "0.0015",
            "takerFeeRate": "0.0025"
        });

        let fee_rate: TradeFeeRate = serde_json::from_value(fee_rate_json).unwrap();

        assert_eq!(fee_rate.maker_fee_rate, "0.0015");
        assert_eq!(fee_rate.taker_fee_rate, "0.0025");
    }

    #[test]
    fn test_trading_account_serialization_roundtrip() {
        let original_account = TradingAccount {
            trading_account_id: "test123".to_string(),
            total_collateral_usd: "1000.00".to_string(),
            total_borrowed_usd: "100.00".to_string(),
            defaulted_margin_usd: "50.00".to_string(),
            risk_limit_usd: "10000.00".to_string(),
            total_liabilities_usd: "150.00".to_string(),
            is_primary_account: true,
            is_borrowing: true,
            is_lending: false,
            is_defaulted: false,
            rate_limit_token: Some("token123".to_string()),
            trade_fee_rate: Some(TradeFeeRate {
                maker_fee_rate: "0.001".to_string(),
                taker_fee_rate: "0.002".to_string(),
            }),
        };

        let json_str = serde_json::to_string(&original_account).unwrap();
        let deserialized_account: TradingAccount = serde_json::from_str(&json_str).unwrap();

        assert_eq!(
            original_account.trading_account_id,
            deserialized_account.trading_account_id
        );
        assert_eq!(
            original_account.is_primary_account,
            deserialized_account.is_primary_account
        );
        assert_eq!(
            original_account.rate_limit_token,
            deserialized_account.rate_limit_token
        );
    }
}
