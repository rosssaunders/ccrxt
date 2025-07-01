//! Income history endpoints for Binance USDM REST API.

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::private::rest::client::RestClient;
use crate::binance::usdm::private::rest::order::OrderErrorResponse;
use crate::binance::usdm::signing::sign_query;
use chrono::Utc;
use reqwest::Method;

/// Error type for USDM income history endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum IncomeHistoryError {
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
    #[serde(other)]
    Unknown(String),
}

/// Income type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IncomeType {
    #[serde(rename = "TRANSFER")]
    Transfer,
    #[serde(rename = "WELCOME_BONUS")]
    WelcomeBonus,
    #[serde(rename = "REALIZED_PNL")]
    RealizedPnl,
    #[serde(rename = "FUNDING_FEE")]
    FundingFee,
    #[serde(rename = "COMMISSION")]
    Commission,
    #[serde(rename = "INSURANCE_CLEAR")]
    InsuranceClear,
    #[serde(rename = "REFERRAL_KICKBACK")]
    ReferralKickback,
    #[serde(rename = "COMMISSION_REBATE")]
    CommissionRebate,
    #[serde(rename = "API_REBATE")]
    ApiRebate,
    #[serde(rename = "CONTEST_REWARD")]
    ContestReward,
    #[serde(rename = "CROSS_COLLATERAL_TRANSFER")]
    CrossCollateralTransfer,
    #[serde(rename = "OPTIONS_PREMIUM_FEE")]
    OptionsPremiumFee,
    #[serde(rename = "OPTIONS_SETTLE_PROFIT")]
    OptionsSettleProfit,
    #[serde(rename = "INTERNAL_TRANSFER")]
    InternalTransfer,
    #[serde(rename = "AUTO_EXCHANGE")]
    AutoExchange,
    #[serde(rename = "DELIVERED_SETTELMENT")]
    DeliveredSettlement,
    #[serde(rename = "COIN_SWAP_DEPOSIT")]
    CoinSwapDeposit,
    #[serde(rename = "COIN_SWAP_WITHDRAW")]
    CoinSwapWithdraw,
    #[serde(rename = "POSITION_LIMIT_INCREASE_FEE")]
    PositionLimitIncreaseFee,
}

/// Request for getting income history.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIncomeHistoryRequest {
    /// Trading symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Income type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_type: Option<IncomeType>,
    /// Start time in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Number of records to return (default 100, max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    /// Timestamp in milliseconds.
    pub timestamp: u64,
    /// Signature for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Income history record
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomeHistoryResponse {
    /// Trading symbol
    pub symbol: String,
    /// Income type
    pub income_type: IncomeType,
    /// Income amount
    pub income: String,
    /// Asset
    pub asset: String,
    /// Income info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    /// Time when income was recorded
    pub time: u64,
    /// Transaction ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tran_id: Option<u64>,
    /// Trade ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
}

impl RestClient {
    /// Get income history.
    pub async fn get_income_history(
        &self,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
        symbol: Option<String>,
        income_type: Option<IncomeType>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u16>,
    ) -> Result<Vec<IncomeHistoryResponse>, IncomeHistoryError> {
        // Rate limiting for private endpoints (30 weight)
        self.rate_limiter
            .acquire_request(30)
            .await
            .map_err(|e| IncomeHistoryError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = GetIncomeHistoryRequest {
            symbol,
            income_type,
            start_time,
            end_time,
            limit,
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| IncomeHistoryError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(Method::GET, &format!("{}/fapi/v1/income", self.base_url))
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| IncomeHistoryError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let income_response: Vec<IncomeHistoryResponse> = response
                .json()
                .await
                .map_err(|e| IncomeHistoryError::Unknown(e.to_string()))?;
            Ok(income_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| IncomeHistoryError::Unknown(e.to_string()))?;
            Err(IncomeHistoryError::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_income_history_response_deserialization() {
        let json = r#"
        [
            {
                "symbol": "BTCUSDT",
                "incomeType": "REALIZED_PNL",
                "income": "-1.37500000",
                "asset": "USDT",
                "info": "BTCUSDT",
                "time": 1570608000000,
                "tranId": 9689322392,
                "tradeId": "2059192"
            }
        ]
        "#;

        let response: Vec<IncomeHistoryResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].symbol, "BTCUSDT");
        assert_eq!(response[0].income, "-1.37500000");
        assert_eq!(response[0].asset, "USDT");
        assert_eq!(response[0].time, 1570608000000);
    }
}
