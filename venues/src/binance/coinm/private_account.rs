use serde::{Deserialize, Serialize};
use super::errors::BinanceCoinMResult;
use super::private_rest::BinanceCoinMPrivateRest;
use super::types::BinanceResponse;
use super::common::request::send_request;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountResponse {
    #[serde(rename = "feeTier")]
    pub fee_tier: i32,
    #[serde(rename = "canTrade")]
    pub can_trade: bool,
    #[serde(rename = "canDeposit")]
    pub can_deposit: bool,
    #[serde(rename = "canWithdraw")]
    pub can_withdraw: bool,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    #[serde(rename = "totalInitialMargin")]
    pub total_initial_margin: String,
    #[serde(rename = "totalMaintMargin")]
    pub total_maint_margin: String,
    #[serde(rename = "totalWalletBalance")]
    pub total_wallet_balance: String,
    #[serde(rename = "totalUnrealizedProfit")]
    pub total_unrealized_profit: String,
    #[serde(rename = "totalMarginBalance")]
    pub total_margin_balance: String,
    #[serde(rename = "totalPositionInitialMargin")]
    pub total_position_initial_margin: String,
    #[serde(rename = "totalOpenOrderInitialMargin")]
    pub total_open_order_initial_margin: String,
    #[serde(rename = "totalCrossWalletBalance")]
    pub total_cross_wallet_balance: String,
    #[serde(rename = "availableBalance")]
    pub available_balance: String,
    #[serde(rename = "maxWithdrawAmount")]
    pub max_withdraw_amount: String,
    pub assets: Vec<Asset>,
    pub positions: Vec<Position>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub asset: String,
    #[serde(rename = "walletBalance")]
    pub wallet_balance: String,
    #[serde(rename = "unrealizedProfit")]
    pub unrealized_profit: String,
    #[serde(rename = "marginBalance")]
    pub margin_balance: String,
    #[serde(rename = "maintMargin")]
    pub maint_margin: String,
    #[serde(rename = "initialMargin")]
    pub initial_margin: String,
    #[serde(rename = "positionInitialMargin")]
    pub position_initial_margin: String,
    #[serde(rename = "openOrderInitialMargin")]
    pub open_order_initial_margin: String,
    #[serde(rename = "crossWalletBalance")]
    pub cross_wallet_balance: String,
    #[serde(rename = "crossUnPnl")]
    pub cross_un_pnl: String,
    #[serde(rename = "availableBalance")]
    pub available_balance: String,
    #[serde(rename = "maxWithdrawAmount")]
    pub max_withdraw_amount: String,
    #[serde(rename = "marginAvailable")]
    pub margin_available: bool,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    #[serde(rename = "positionAmt")]
    pub position_amt: String,
    #[serde(rename = "entryPrice")]
    pub entry_price: String,
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    #[serde(rename = "unRealizedProfit")]
    pub unrealized_profit: String,
    #[serde(rename = "liquidationPrice")]
    pub liquidation_price: String,
    pub leverage: String,
    #[serde(rename = "maxNotionalValue")]
    pub max_notional_value: String,
    #[serde(rename = "marginType")]
    pub margin_type: String,
    #[serde(rename = "isolatedMargin")]
    pub isolated_margin: String,
    #[serde(rename = "isAutoAddMargin")]
    pub is_auto_add_margin: String,
    #[serde(rename = "positionSide")]
    pub position_side: String,
    pub notional: String,
    #[serde(rename = "isolatedWallet")]
    pub isolated_wallet: String,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}

impl BinanceCoinMPrivateRest {
    pub async fn get_account(&self) -> BinanceCoinMResult<BinanceResponse<AccountResponse>> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let query_string = format!("timestamp={}", timestamp);
        let signature = self.sign_request(&query_string);
        
        let endpoint = format!("/dapi/v1/account?{}&signature={}", query_string, signature);
        
        send_request(
            &self.client,
            &self.base_url,
            &endpoint,
            reqwest::Method::GET,
            None,
            Some(&self.api_key),
            || self.rate_limiter.check_weight_limit("account", 1)
        ).await
    }
} 