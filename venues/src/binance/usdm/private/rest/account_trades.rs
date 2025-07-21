use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::UsdmClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::*;

const ACCOUNT_TRADES_ENDPOINT: &str = "/fapi/v1/userTrades";

/// Request parameters for the account trade list endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountTradesRequest {
    /// Trading symbol (e.g., "BTCUSDT"). Required.
    pub symbol: Cow<'static, str>,

    /// Start time for filtering trades (milliseconds since epoch). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time for filtering trades (milliseconds since epoch). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Trade ID to fetch from. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,

    /// Limit the number of trades returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Represents a single account trade.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountTrade {
    /// Trade ID.
    pub id: u64,

    /// Order ID.
    pub order_id: u64,

    /// Trading symbol.
    pub symbol: Cow<'static, str>,

    /// Trade price.
    pub price: Cow<'static, str>,

    /// Trade quantity.
    pub qty: Cow<'static, str>,

    /// Commission amount.
    pub commission: Cow<'static, str>,

    /// Commission asset.
    pub commission_asset: Cow<'static, str>,

    /// Trade time (milliseconds since epoch).
    pub time: u64,

    /// Trade side (BUY/SELL).
    pub side: OrderSide,

    /// Position side (LONG/SHORT/BOTH).
    pub position_side: PositionSide,

    /// True if buyer.
    pub buyer: bool,

    /// True if maker.
    pub maker: bool,
}

impl UsdmClient {
    /// Account trade list (GET /fapi/v1/userTrades)
    ///
    /// Returns a list of trades for the account.
    /// [docs]: https://binance-docs.github.io/apidocs/futures/en/#account-trade-list-user_data
    ///
    /// Rate limit: 5
    ///
    /// # Arguments
    /// * `params` - The request parameters
    ///
    /// # Returns
    /// Vector of account trades
    pub async fn get_account_trades(
        &self,
        params: GetAccountTradesRequest,
    ) -> RestResult<Vec<AccountTrade>> {
        self.send_signed_request(ACCOUNT_TRADES_ENDPOINT, Method::GET, params, 5, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_account_trades_request_serialization() {
        let request = GetAccountTradesRequest {
            symbol: "BTCUSDT".into(),
            start_time: Some(1625184000000),
            end_time: Some(1625270400000),
            from_id: Some(1000),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("start_time=1625184000000"));
        assert!(serialized.contains("end_time=1625270400000"));
        assert!(serialized.contains("from_id=1000"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_get_account_trades_request_minimal() {
        let request = GetAccountTradesRequest {
            symbol: "ETHUSDT".into(),
            start_time: None,
            end_time: None,
            from_id: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=ETHUSDT");
    }

    #[test]
    fn test_account_trade_deserialization() {
        let json = r#"{
            "id": 1234567,
            "orderId": 9876543210,
            "symbol": "BTCUSDT",
            "price": "45380.10",
            "qty": "0.100",
            "commission": "0.04538010",
            "commissionAsset": "USDT",
            "time": 1625184000000,
            "side": "BUY",
            "positionSide": "LONG",
            "buyer": true,
            "maker": false
        }"#;

        let trade: AccountTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 1234567);
        assert_eq!(trade.order_id, 9876543210);
        assert_eq!(trade.symbol, "BTCUSDT");
        assert_eq!(trade.price, "45380.10");
        assert_eq!(trade.qty, "0.100");
        assert_eq!(trade.commission, "0.04538010");
        assert_eq!(trade.commission_asset, "USDT");
        assert_eq!(trade.time, 1625184000000);
        assert!(matches!(trade.side, OrderSide::Buy));
        assert!(matches!(trade.position_side, PositionSide::Long));
        assert!(trade.buyer);
        assert!(!trade.maker);
    }

    #[test]
    fn test_account_trade_sell_side() {
        let json = r#"{
            "id": 1234568,
            "orderId": 9876543211,
            "symbol": "ETHUSDT",
            "price": "3070.50",
            "qty": "0.500",
            "commission": "0.07676250",
            "commissionAsset": "USDT",
            "time": 1625184000000,
            "side": "SELL",
            "positionSide": "SHORT",
            "buyer": false,
            "maker": true
        }"#;

        let trade: AccountTrade = serde_json::from_str(json).unwrap();
        assert!(matches!(trade.side, OrderSide::Sell));
        assert!(matches!(trade.position_side, PositionSide::Short));
        assert!(!trade.buyer);
        assert!(trade.maker);
    }

    #[test]
    fn test_account_trade_with_btc_commission() {
        let json = r#"{
            "id": 1234569,
            "orderId": 9876543212,
            "symbol": "BTCUSDT",
            "price": "45380.10",
            "qty": "0.100",
            "commission": "0.00000100",
            "commissionAsset": "BTC",
            "time": 1625184000000,
            "side": "BUY",
            "positionSide": "BOTH",
            "buyer": true,
            "maker": false
        }"#;

        let trade: AccountTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.commission, "0.00000100");
        assert_eq!(trade.commission_asset, "BTC");
        assert!(matches!(trade.position_side, PositionSide::Both));
    }

    #[test]
    fn test_account_trades_array_deserialization() {
        let json = r#"[
            {
                "id": 1234567,
                "orderId": 9876543210,
                "symbol": "BTCUSDT",
                "price": "45380.10",
                "qty": "0.100",
                "commission": "0.04538010",
                "commissionAsset": "USDT",
                "time": 1625184000000,
                "side": "BUY",
                "positionSide": "LONG",
                "buyer": true,
                "maker": false
            },
            {
                "id": 1234568,
                "orderId": 9876543211,
                "symbol": "BTCUSDT",
                "price": "45385.20",
                "qty": "0.050",
                "commission": "0.02269260",
                "commissionAsset": "USDT",
                "time": 1625184060000,
                "side": "SELL",
                "positionSide": "LONG",
                "buyer": false,
                "maker": true
            }
        ]"#;

        let trades: Vec<AccountTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].id, 1234567);
        assert_eq!(trades[1].id, 1234568);
        assert!(matches!(trades[0].side, OrderSide::Buy));
        assert!(matches!(trades[1].side, OrderSide::Sell));
    }

    #[test]
    fn test_account_trades_empty_response() {
        let json = r#"[]"#;
        let trades: Vec<AccountTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 0);
    }

    #[test]
    fn test_account_trade_high_precision_values() {
        let json = r#"{
            "id": 1234570,
            "orderId": 9876543213,
            "symbol": "DOGEUSDT",
            "price": "0.12345678",
            "qty": "10000.00000000",
            "commission": "0.00123456",
            "commissionAsset": "DOGE",
            "time": 1625184000000,
            "side": "BUY",
            "positionSide": "BOTH",
            "buyer": true,
            "maker": false
        }"#;

        let trade: AccountTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.price, "0.12345678");
        assert_eq!(trade.qty, "10000.00000000");
        assert_eq!(trade.commission_asset, "DOGE");
    }

    #[test]
    fn test_account_trade_zero_commission() {
        let json = r#"{
            "id": 1234571,
            "orderId": 9876543214,
            "symbol": "BTCUSDT",
            "price": "45380.10",
            "qty": "0.100",
            "commission": "0.00000000",
            "commissionAsset": "BNB",
            "time": 1625184000000,
            "side": "BUY",
            "positionSide": "BOTH",
            "buyer": true,
            "maker": false
        }"#;

        let trade: AccountTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.commission, "0.00000000");
        assert_eq!(trade.commission_asset, "BNB");
    }
}
