use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

/// Endpoint URL path for trading accounts
const TRADING_ACCOUNTS_ENDPOINT: &str = "/v1/accounts/trading-accounts";

/// Trading account information
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingAccount {
    /// Unique trading account identifier
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,

    /// Name of the trading account
    #[serde(rename = "tradingAccountName", skip_serializing_if = "Option::is_none")]
    pub trading_account_name: Option<String>,

    /// Description of the trading account
    #[serde(
        rename = "tradingAccountDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_account_description: Option<String>,

    /// Whether this is the primary trading account
    #[serde(rename = "isPrimaryAccount")]
    pub is_primary_account: bool,

    /// Whether the trading account is borrowing
    #[serde(rename = "isBorrowing")]
    pub is_borrowing: bool,

    /// Whether the trading account is lending
    #[serde(rename = "isLending")]
    pub is_lending: bool,

    /// Max initial leverage
    #[serde(rename = "maxInitialLeverage", skip_serializing_if = "Option::is_none")]
    pub max_initial_leverage: Option<String>,

    /// Unique rate limit token of the trading account
    #[serde(rename = "rateLimitToken", skip_serializing_if = "Option::is_none")]
    pub rate_limit_token: Option<String>,

    /// Whether the trading account is defaulted
    #[serde(rename = "isDefaulted")]
    pub is_defaulted: bool,

    /// Trade fees per feeGroupId for this trading account
    #[serde(rename = "tradeFeeRate", skip_serializing_if = "Option::is_none")]
    pub trade_fee_rate: Option<Vec<TradeFeeTier>>,

    /// The maximum allowed borrowing for this trading account (USD)
    #[serde(rename = "riskLimitUSD")]
    pub risk_limit_usd: String,

    /// The total liabilities for this trading account (USD)
    #[serde(rename = "totalLiabilitiesUSD")]
    pub total_liabilities_usd: String,

    /// Total borrowed across all assets displayed in USD
    #[serde(rename = "totalBorrowedUSD")]
    pub total_borrowed_usd: String,

    /// Total collateral across all assets displayed in USD
    #[serde(rename = "totalCollateralUSD")]
    pub total_collateral_usd: String,

    /// The minimum margin to increase risk (USD)
    #[serde(rename = "initialMarginUSD", skip_serializing_if = "Option::is_none")]
    pub initial_margin_usd: Option<String>,

    /// Warning margin threshold (USD)
    #[serde(rename = "warningMarginUSD", skip_serializing_if = "Option::is_none")]
    pub warning_margin_usd: Option<String>,

    /// Liquidation margin threshold (USD)
    #[serde(
        rename = "liquidationMarginUSD",
        skip_serializing_if = "Option::is_none"
    )]
    pub liquidation_margin_usd: Option<String>,

    /// Full liquidation margin threshold (USD)
    #[serde(
        rename = "fullLiquidationMarginUSD",
        skip_serializing_if = "Option::is_none"
    )]
    pub full_liquidation_margin_usd: Option<String>,

    /// Defaulted margin threshold (USD)
    #[serde(rename = "defaultedMarginUSD")]
    pub defaulted_margin_usd: String,

    /// End customer id used for self trade prevention
    #[serde(rename = "endCustomerId", skip_serializing_if = "Option::is_none")]
    pub end_customer_id: Option<String>,

    /// Whether concentration risk checks are enforced
    #[serde(
        rename = "isConcentrationRiskEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_concentration_risk_enabled: Option<bool>,

    /// Expected market impact in a liquidation event (USD)
    #[serde(rename = "liquidityAddonUSD", skip_serializing_if = "Option::is_none")]
    pub liquidity_addon_usd: Option<String>,

    /// Worst possible loss on the portfolio (USD)
    #[serde(rename = "marketRiskUSD", skip_serializing_if = "Option::is_none")]
    pub market_risk_usd: Option<String>,

    /// Margin profile multipliers
    #[serde(rename = "marginProfile", skip_serializing_if = "Option::is_none")]
    pub margin_profile: Option<MarginProfile>,
}

/// Trade fee tier information for an account's fee group
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeFeeTier {
    /// Identifier for this particular fee tier
    #[serde(rename = "feeGroupId")]
    pub fee_group_id: i64,

    /// Maker fee in basis points (bps)
    #[serde(rename = "makerFee")]
    pub maker_fee: String,

    /// Taker fee in basis points (bps)
    #[serde(rename = "takerFee")]
    pub taker_fee: String,
}

/// Margin profile multipliers used to derive margin requirement values
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MarginProfile {
    /// Multiplier for initial margin requirement
    #[serde(
        rename = "initialMarketRiskMultiplierPct",
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_market_risk_multiplier_pct: Option<String>,

    /// Multiplier for warning margin requirement
    #[serde(
        rename = "warningMarketRiskMultiplierPct",
        skip_serializing_if = "Option::is_none"
    )]
    pub warning_market_risk_multiplier_pct: Option<String>,

    /// Multiplier for liquidation margin requirement
    #[serde(
        rename = "liquidationMarketRiskMultiplierPct",
        skip_serializing_if = "Option::is_none"
    )]
    pub liquidation_market_risk_multiplier_pct: Option<String>,

    /// Multiplier for full liquidation margin requirement
    #[serde(
        rename = "fullLiquidationMarketRiskMultiplierPct",
        skip_serializing_if = "Option::is_none"
    )]
    pub full_liquidation_market_risk_multiplier_pct: Option<String>,

    /// Multiplier for defaulted margin requirement
    #[serde(
        rename = "defaultedMarketRiskMultiplierPct",
        skip_serializing_if = "Option::is_none"
    )]
    pub defaulted_market_risk_multiplier_pct: Option<String>,
}

/// Response wrapper for trading accounts array
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TradingAccountsResponse {
    /// List of trading accounts returned by the API
    pub accounts: Vec<TradingAccount>,
}

impl RestClient {
    /// Get trading accounts
    ///
    /// Gets details for all trading accounts accessible by the API key used in the request.
    ///
    /// [docs]: https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/accounts/trading-accounts
    ///
    /// # Returns
    /// Array of trading account information
    pub async fn get_trading_accounts(&mut self) -> RestResult<TradingAccountsResponse> {
        self.send_get_request(
            TRADING_ACCOUNTS_ENDPOINT,
            (),
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
    fn test_trading_account_deserialization_with_fee_tiers() {
        let account_json = json!({
            "tradingAccountId": "111234567890",
            "tradingAccountName": "algo trading account",
            "tradingAccountDescription": "desc",
            "isPrimaryAccount": true,
            "isBorrowing": true,
            "isLending": false,
            "maxInitialLeverage": "1",
            "rateLimitToken": "abcd1234efgh5678",
            "isDefaulted": false,
            "tradeFeeRate": [
                {"feeGroupId": 1, "makerFee": "1.0", "takerFee": "2.0"}
            ],
            "riskLimitUSD": "100000.00",
            "totalLiabilitiesUSD": "5500.75",
            "totalBorrowedUSD": "5000.25",
            "totalCollateralUSD": "10000.50",
            "initialMarginUSD": "100.00",
            "warningMarginUSD": "200.00",
            "liquidationMarginUSD": "80.00",
            "fullLiquidationMarginUSD": "50.00",
            "defaultedMarginUSD": "10.00",
            "endCustomerId": "end-1",
            "isConcentrationRiskEnabled": true,
            "liquidityAddonUSD": "12.34",
            "marketRiskUSD": "23.45",
            "marginProfile": {
                "initialMarketRiskMultiplierPct": "1.0",
                "warningMarketRiskMultiplierPct": "1.2",
                "liquidationMarketRiskMultiplierPct": "1.3",
                "fullLiquidationMarketRiskMultiplierPct": "1.4",
                "defaultedMarketRiskMultiplierPct": "2.0"
            }
        });

        let account: TradingAccount = serde_json::from_value(account_json).unwrap();
        assert_eq!(account.trading_account_id, "111234567890");
        assert!(account.is_primary_account);
        assert!(account.is_borrowing);
        assert!(!account.is_lending);
        assert_eq!(account.trade_fee_rate.unwrap()[0].fee_group_id, 1);
    }

    #[test]
    fn test_trading_accounts_response_transparent_array() {
        let response_json = json!([
            {"tradingAccountId": "a", "isPrimaryAccount": true, "isBorrowing": false, "isLending": false, "isDefaulted": false, "riskLimitUSD": "0", "totalLiabilitiesUSD": "0", "totalBorrowedUSD": "0", "totalCollateralUSD": "0", "defaultedMarginUSD": "0"},
            {"tradingAccountId": "b", "isPrimaryAccount": false, "isBorrowing": true, "isLending": false, "isDefaulted": false, "riskLimitUSD": "0", "totalLiabilitiesUSD": "0", "totalBorrowedUSD": "0", "totalCollateralUSD": "0", "defaultedMarginUSD": "0"}
        ]);

        let wrapped: TradingAccountsResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(wrapped.accounts.len(), 2);
        assert_eq!(wrapped.accounts[0].trading_account_id, "a");
        assert!(wrapped.accounts[0].is_primary_account);
        assert!(wrapped.accounts[1].is_borrowing);
    }
}
