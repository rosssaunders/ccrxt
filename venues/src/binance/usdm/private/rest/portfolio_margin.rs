//! Portfolio margin endpoints for Binance USDM REST API.

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::enums::*;
use crate::binance::usdm::private::rest::client::RestClient;
use crate::binance::usdm::private::rest::order::OrderErrorResponse;
use crate::binance::usdm::signing::sign_query;
use chrono::Utc;
use reqwest::Method;

/// Error type for USDM portfolio margin endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum PortfolioMarginError {
    /// Invalid API key or signature.
    #[error("Invalid API key or signature: {0}")]
    #[serde(rename = "-1022")]
    InvalidSignature(String),
    /// Timestamp for this request is outside of the recv window.
    #[error("Timestamp for this request is outside of the recv window: {0}")]
    #[serde(rename = "-1021")]
    TimestampOutOfRecvWindow(String),
    /// Invalid API key format.
    #[error("Invalid API key format: {0}")]
    #[serde(rename = "-2014")]
    BadApiKeyFmt(String),
    /// Invalid API key, IP, or permissions for action.
    #[error("Invalid API key, IP, or permissions for action: {0}")]
    #[serde(rename = "-2015")]
    RejectedMbxKey(String),
    /// Unknown error.
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Request for getting portfolio margin account info.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPortfolioMarginAccountRequest {
    /// Asset (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    /// Timestamp in milliseconds.
    pub timestamp: u64,
    /// Signature for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Portfolio margin asset information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioMarginAsset {
    /// Asset name
    pub asset: String,
    /// Cross wallet balance
    pub cross_wallet_balance: String,
    /// Cross un pnl
    pub cross_un_pnl: String,
    /// Available balance
    pub available_balance: String,
    /// Max withdraw amount
    pub max_withdraw_amount: String,
    /// Margin available
    pub margin_available: bool,
    /// Update time
    pub update_time: u64,
}

/// Portfolio margin position information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioMarginPosition {
    /// Symbol
    pub symbol: String,
    /// Initial margin
    pub initial_margin: String,
    /// Maintenance margin
    pub maint_margin: String,
    /// Unrealized profit
    pub unrealized_profit: String,
    /// Position initial margin
    pub position_initial_margin: String,
    /// Open order initial margin
    pub open_order_initial_margin: String,
    /// Leverage
    pub leverage: String,
    /// Isolated
    pub isolated: bool,
    /// Entry price
    pub entry_price: String,
    /// Breakeven price
    pub breakeven_price: String,
    /// Max notional
    pub max_notional: String,
    /// Bid notional
    pub bid_notional: String,
    /// Ask notional
    pub ask_notional: String,
    /// Position side
    pub position_side: PositionSide,
    /// Position amount
    pub position_amt: String,
    /// Update time
    pub update_time: u64,
}

/// Response from portfolio margin account endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioMarginAccountResponse {
    /// Total wallet balance
    pub total_wallet_balance: String,
    /// Total unrealized profit
    pub total_unrealized_profit: String,
    /// Total margin balance
    pub total_margin_balance: String,
    /// Total position initial margin
    pub total_position_initial_margin: String,
    /// Total open order initial margin
    pub total_open_order_initial_margin: String,
    /// Total cross wallet balance
    pub total_cross_wallet_balance: String,
    /// Total cross un pnl
    pub total_cross_un_pnl: String,
    /// Available balance
    pub available_balance: String,
    /// Max withdraw amount
    pub max_withdraw_amount: String,
    /// Assets
    pub assets: Vec<PortfolioMarginAsset>,
    /// Positions
    pub positions: Vec<PortfolioMarginPosition>,
}

impl RestClient {
    /// Get portfolio margin account information.
    pub async fn get_portfolio_margin_account(
        &self,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
        asset: Option<String>,
    ) -> Result<PortfolioMarginAccountResponse, PortfolioMarginError> {
        // Rate limiting for private endpoints (5 weight)
        self.rate_limiter
            .acquire_request(5)
            .await
            .map_err(|e| PortfolioMarginError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = GetPortfolioMarginAccountRequest {
            asset,
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| PortfolioMarginError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(Method::GET, &format!("{}/fapi/v1/pmAccountInfo", self.base_url))
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| PortfolioMarginError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let account_response: PortfolioMarginAccountResponse = response
                .json()
                .await
                .map_err(|e| PortfolioMarginError::Unknown(e.to_string()))?;
            Ok(account_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| PortfolioMarginError::Unknown(e.to_string()))?;
            Err(PortfolioMarginError::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_portfolio_margin_account_response_deserialization() {
        let json = r#"
        {
            "totalWalletBalance": "126.72469206",
            "totalUnrealizedProfit": "0.00000000",
            "totalMarginBalance": "126.72469206",
            "totalPositionInitialMargin": "0.00000000",
            "totalOpenOrderInitialMargin": "0.00000000",
            "totalCrossWalletBalance": "126.72469206",
            "totalCrossUnPnl": "0.00000000",
            "availableBalance": "126.72469206",
            "maxWithdrawAmount": "126.72469206",
            "assets": [
                {
                    "asset": "USDT",
                    "crossWalletBalance": "126.72469206",
                    "crossUnPnl": "0.00000000",
                    "availableBalance": "126.72469206",
                    "maxWithdrawAmount": "126.72469206",
                    "marginAvailable": true,
                    "updateTime": 1617939110373
                }
            ],
            "positions": [
                {
                    "symbol": "BTCUSDT",
                    "initialMargin": "0",
                    "maintMargin": "0",
                    "unrealizedProfit": "0.00000000",
                    "positionInitialMargin": "0",
                    "openOrderInitialMargin": "0",
                    "leverage": "100",
                    "isolated": false,
                    "entryPrice": "0.00000",
                    "breakevenPrice": "0.0",
                    "maxNotional": "25000",
                    "bidNotional": "0",
                    "askNotional": "0",
                    "positionSide": "BOTH",
                    "positionAmt": "0",
                    "updateTime": 0
                }
            ]
        }
        "#;

        let response: PortfolioMarginAccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.total_wallet_balance, "126.72469206");
        assert_eq!(response.assets.len(), 1);
        assert_eq!(response.assets[0].asset, "USDT");
        assert_eq!(response.positions.len(), 1);
        assert_eq!(response.positions[0].symbol, "BTCUSDT");
        assert_eq!(response.positions[0].position_side, PositionSide::Both);
    }
}
