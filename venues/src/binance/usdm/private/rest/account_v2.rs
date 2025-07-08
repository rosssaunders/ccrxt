//! Account information V2 endpoints for Binance USDM REST API.

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{
    enums::PositionSide,
    private::rest::{client::RestClient, order::OrderErrorResponse},
    signing::sign_query,
};

/// Error type for USDM account V2 endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
pub enum AccountV2Error {
    #[error("Invalid API key or signature: {0}")]
    InvalidApiKeyOrSignature(String),

    #[error("Too many requests: {0}")]
    TooManyRequests(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for USDM account V2 operations.
pub type AccountV2Result<T> = Result<T, AccountV2Error>;

/// Request for getting account information V2.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountV2Request {
    /// Request timestamp in milliseconds.
    pub timestamp: u64,
    /// Request signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl Default for GetAccountV2Request {
    fn default() -> Self {
        Self {
            timestamp: Utc::now().timestamp_millis() as u64,
            signature: None,
        }
    }
}

/// Asset information in account V2.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetV2 {
    /// Asset name.
    pub asset: String,
    /// Wallet balance.
    pub wallet_balance: String,
    /// Unrealized PnL.
    pub unrealized_profit: String,
    /// Margin balance.
    pub margin_balance: String,
    /// Maintenance margin.
    pub maint_margin: String,
    /// Initial margin.
    pub initial_margin: String,
    /// Position initial margin.
    pub position_initial_margin: String,
    /// Open order initial margin.
    pub open_order_initial_margin: String,
    /// Cross wallet balance.
    pub cross_wallet_balance: String,
    /// Cross unrealized PnL.
    pub cross_un_pnl: String,
    /// Available balance.
    pub available_balance: String,
    /// Maximum amount for transfer out.
    pub max_withdraw_amount: String,
    /// Whether the asset can be used as margin in Multi-Assets mode.
    pub margin_available: bool,
    /// Last update time.
    pub update_time: u64,
}

/// Position information in account V2.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionV2 {
    /// Trading symbol.
    pub symbol: String,
    /// Initial margin.
    pub initial_margin: String,
    /// Maintenance margin.
    pub maint_margin: String,
    /// Unrealized PnL.
    pub unrealized_profit: String,
    /// Position initial margin.
    pub position_initial_margin: String,
    /// Open order initial margin.
    pub open_order_initial_margin: String,
    /// Current leverage.
    pub leverage: String,
    /// Isolated position value.
    pub isolated: bool,
    /// Entry price.
    pub entry_price: String,
    /// Break-even price.
    pub break_even_price: String,
    /// Maximum notional value.
    pub max_notional: String,
    /// Bid notional.
    pub bid_notional: String,
    /// Ask notional.
    pub ask_notional: String,
    /// Position side.
    pub position_side: PositionSide,
    /// Position amount.
    pub position_amt: String,
    /// Last update time.
    pub update_time: u64,
}

/// Account information V2 response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountV2Response {
    /// Fee tier.
    pub fee_tier: u32,
    /// Whether the account can trade.
    pub can_trade: bool,
    /// Whether the account can deposit.
    pub can_deposit: bool,
    /// Whether the account can withdraw.
    pub can_withdraw: bool,
    /// Fee burn option for spot trading.
    pub fee_burn: bool,
    /// Last update time.
    pub update_time: u64,
    /// Multi-assets margin.
    pub multi_assets_margin: bool,
    /// Trade group ID.
    pub trade_group_id: Option<i64>,
    /// Total initial margin.
    pub total_initial_margin: String,
    /// Total maintenance margin.
    pub total_maint_margin: String,
    /// Total wallet balance.
    pub total_wallet_balance: String,
    /// Total unrealized PnL.
    pub total_unrealized_profit: String,
    /// Total margin balance.
    pub total_margin_balance: String,
    /// Total position initial margin.
    pub total_position_initial_margin: String,
    /// Total open order initial margin.
    pub total_open_order_initial_margin: String,
    /// Total cross wallet balance.
    pub total_cross_wallet_balance: String,
    /// Total cross unrealized PnL.
    pub total_cross_un_pnl: String,
    /// Available balance.
    pub available_balance: String,
    /// Maximum amount for transfer out.
    pub max_withdraw_amount: String,
    /// Assets information.
    pub assets: Vec<AssetV2>,
    /// Positions information.
    pub positions: Vec<PositionV2>,
}

impl RestClient {
    /// Get account information V2.
    ///
    /// Retrieves comprehensive account information including balances, positions,
    /// and trading permissions with detailed asset and position information.
    ///
    /// Weight: 5
    ///
    /// # Arguments
    ///
    /// * `api_key` - API key for authentication
    /// * `api_secret` - API secret for signing requests
    ///
    /// # Returns
    ///
    /// Returns `AccountV2Response` containing comprehensive account information.
    ///
    /// # Errors
    ///
    /// Returns `AccountV2Error` if:
    /// - Authentication fails
    /// - Rate limits are exceeded
    /// - API error occurs
    pub async fn get_account_v2(
        &self,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
    ) -> AccountV2Result<AccountV2Response> {
        // Acquire rate limit permit
        self.rate_limiter
            .acquire_request(5)
            .await
            .map_err(|e| AccountV2Error::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = GetAccountV2Request {
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| AccountV2Error::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(Method::GET, format!("{}/fapi/v2/account", self.base_url))
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| AccountV2Error::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let account_response: AccountV2Response = response
                .json()
                .await
                .map_err(|e| AccountV2Error::Unknown(e.to_string()))?;
            Ok(account_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| AccountV2Error::Unknown(e.to_string()))?;
            Err(AccountV2Error::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_account_v2_request_serialization() {
        let request = GetAccountV2Request {
            timestamp: 1625097600000,
            signature: Some("test_signature".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("signature=test_signature"));
    }

    #[test]
    fn test_account_v2_response_deserialization() {
        let json = r#"{
            "feeTier": 0,
            "canTrade": true,
            "canDeposit": true,
            "canWithdraw": true,
            "feeBurn": false,
            "updateTime": 1625097600000,
            "multiAssetsMargin": false,
            "totalInitialMargin": "0.00000000",
            "totalMaintMargin": "0.00000000",
            "totalWalletBalance": "1000.00000000",
            "totalUnrealizedProfit": "0.00000000",
            "totalMarginBalance": "1000.00000000",
            "totalPositionInitialMargin": "0.00000000",
            "totalOpenOrderInitialMargin": "0.00000000",
            "totalCrossWalletBalance": "1000.00000000",
            "totalCrossUnPnl": "0.00000000",
            "availableBalance": "1000.00000000",
            "maxWithdrawAmount": "1000.00000000",
            "assets": [],
            "positions": []
        }"#;

        let response: AccountV2Response = serde_json::from_str(json).unwrap();
        assert_eq!(response.fee_tier, 0);
        assert!(response.can_trade);
        assert_eq!(response.total_wallet_balance, "1000.00000000");
    }
}
