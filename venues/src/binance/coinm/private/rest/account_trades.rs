use serde::{Deserialize, Serialize};

use crate::binance::coinm::{super::RestClient, OrderSide, PositionSide, RestResult};

const ACCOUNT_TRADES_ENDPOINT: &str = "/dapi/v1/userTrades";

/// Request parameters for the Account Trade List endpoint (GET /dapi/v1/userTrades).
#[derive(Debug, Clone, Serialize, Default)]
pub struct AccountTradeListRequest {
    /// The trading symbol (e.g., "BTCUSD_PERP").
    /// Either `symbol` or `pair` must be sent, but not both.
    /// Format: "{ASSET}USD_PERP" for perpetual contracts, "{ASSET}USD_{EXPIRY}" for delivery contracts.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// The trading pair (e.g., "BTCUSD").
    /// Either `symbol` or `pair` must be sent, but not both.
    /// Format: "{ASSET}USD" where ASSET is the base asset (e.g., BTC, ETH).
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,

    /// The order ID to filter trades by. Can only be sent with `symbol`.
    /// Must be a valid order ID from a previous order.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Start time in milliseconds since epoch.
    /// Must be less than endTime if both are provided.
    /// Range: 0 to current timestamp.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds since epoch.
    /// Must be greater than startTime if both are provided.
    /// Range: 0 to current timestamp.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Trade ID to fetch from. Default gets most recent trades. Cannot be sent with `pair`.
    /// Must be a valid trade ID from a previous trade.
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,

    /// Number of results to return. Default 50; max 1000.
    /// Range: 1 to 1000.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// The value cannot be greater than 60000.
    /// Range: 0 to 60000 milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds since epoch. Mandatory.
    /// Must be within 1000ms of server time.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Represents a single trade returned by the Account Trade List endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct AccountTrade {
    /// The trading symbol (e.g., "BTCUSD_200626").
    /// Format: "{ASSET}USD_{EXPIRY}" for delivery contracts, "{ASSET}USD_PERP" for perpetual contracts.
    pub symbol: String,

    /// Trade ID.
    /// Unique identifier for the trade.
    pub id: u64,

    /// Order ID associated with this trade.
    /// Links to the original order that generated this trade.
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// The trading pair (e.g., "BTCUSD").
    /// Format: "{ASSET}USD" where ASSET is the base asset.
    pub pair: String,

    /// The side of the trade (BUY or SELL).
    /// Indicates whether the trade was a buy or sell order.
    pub side: OrderSide,

    /// The price at which the trade was executed.
    /// Format: Decimal string with precision up to 8 decimal places.
    pub price: String,

    /// The quantity of the trade.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "qty")]
    pub quantity: String,

    /// Realized PnL for this trade.
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Positive for profit, negative for loss.
    #[serde(rename = "realizedPnl")]
    pub realized_pnl: String,

    /// The margin asset used for this trade (e.g., "BTC").
    /// The asset used for margin calculations and fee payments.
    #[serde(rename = "marginAsset")]
    pub margin_asset: String,

    /// The base quantity for this trade.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "baseQty")]
    pub base_qty: String,

    /// The commission paid for this trade.
    /// Format: Decimal string with precision up to 8 decimal places.
    pub commission: String,

    /// The asset in which commission was paid.
    /// The asset used to pay the trading fee.
    #[serde(rename = "commissionAsset")]
    pub commission_asset: String,

    /// The timestamp of the trade in milliseconds since epoch.
    /// When the trade was executed on the exchange.
    pub time: u64,

    /// The position side (BOTH, LONG, SHORT).
    /// Indicates the position direction for this trade.
    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,

    /// Whether the buyer is the taker.
    /// True if the buyer was the taker, false if the seller was the taker.
    pub buyer: bool,

    /// Whether the trade was a maker trade.
    /// True if the trade was a maker trade, false if it was a taker trade.
    pub maker: bool,
}

impl RestClient {
    /// Fetches trades for a specific account and symbol or pair.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Account-Trade-List
    ///
    /// GET /dapi/v1/userTrades
    /// Weight: 20 with symbol, 40 with pair
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`AccountTradeListRequest`])
    ///
    /// # Returns
    /// A vector of [`AccountTrade`] objects.
    pub async fn get_account_trades(
        &self,
        params: AccountTradeListRequest,
    ) -> RestResult<Vec<AccountTrade>> {
        let weight = if params.pair.is_some() { 40 } else { 20 };
        self.send_signed_request(
            ACCOUNT_TRADES_ENDPOINT,
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_trade_list_request_serialization_with_symbol() {
        let request = AccountTradeListRequest {
            symbol: Some("BTCUSD_PERP".to_string()),
            pair: None,
            order_id: Some("123456".to_string()),
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
            from_id: None,
            limit: Some(100),
            recv_window: Some(5000),
            timestamp: 1625184000000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("orderId=123456"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625184000000"));
        assert!(!serialized.contains("pair="));
        assert!(!serialized.contains("fromId="));
    }

    #[test]
    fn test_account_trade_list_request_serialization_with_pair() {
        let request = AccountTradeListRequest {
            symbol: None,
            pair: Some("BTCUSD".to_string()),
            order_id: None,
            start_time: None,
            end_time: None,
            from_id: Some(789012),
            limit: Some(50),
            recv_window: None,
            timestamp: 1625184000000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("pair=BTCUSD"));
        assert!(serialized.contains("fromId=789012"));
        assert!(serialized.contains("limit=50"));
        assert!(serialized.contains("timestamp=1625184000000"));
        assert!(!serialized.contains("symbol="));
        assert!(!serialized.contains("orderId="));
        assert!(!serialized.contains("startTime="));
        assert!(!serialized.contains("endTime="));
        assert!(!serialized.contains("recvWindow="));
    }

    #[test]
    fn test_account_trade_list_request_minimal_serialization() {
        let request = AccountTradeListRequest {
            symbol: Some("ETHUSD_PERP".to_string()),
            pair: None,
            order_id: None,
            start_time: None,
            end_time: None,
            from_id: None,
            limit: None,
            recv_window: None,
            timestamp: 1625184000000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=ETHUSD_PERP&timestamp=1625184000000");
    }

    #[test]
    fn test_account_trade_deserialization() {
        let json = r#"{
            "symbol": "BTCUSD_200626",
            "id": 215970406,
            "orderId": 469935690,
            "pair": "BTCUSD",
            "side": "SELL",
            "price": "9638.0",
            "qty": "1",
            "realizedPnl": "-0.00058794",
            "marginAsset": "BTC",
            "baseQty": "0.01037883",
            "commission": "0.00000454",
            "commissionAsset": "BTC",
            "time": 1591155762721,
            "positionSide": "BOTH",
            "buyer": false,
            "maker": true
        }"#;

        let trade: AccountTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.symbol, "BTCUSD_200626");
        assert_eq!(trade.id, 215970406);
        assert_eq!(trade.order_id, 469935690);
        assert_eq!(trade.pair, "BTCUSD");
        assert_eq!(trade.side, OrderSide::Sell);
        assert_eq!(trade.price, "9638.0");
        assert_eq!(trade.quantity, "1");
        assert_eq!(trade.realized_pnl, "-0.00058794");
        assert_eq!(trade.margin_asset, "BTC");
        assert_eq!(trade.base_qty, "0.01037883");
        assert_eq!(trade.commission, "0.00000454");
        assert_eq!(trade.commission_asset, "BTC");
        assert_eq!(trade.time, 1591155762721);
        assert_eq!(trade.position_side, PositionSide::Both);
        assert!(!trade.buyer);
        assert!(trade.maker);
    }

    #[test]
    fn test_account_trade_deserialization_buy_taker() {
        let json = r#"{
            "symbol": "ETHUSD_PERP",
            "id": 315970407,
            "orderId": 569935691,
            "pair": "ETHUSD",
            "side": "BUY",
            "price": "3200.50",
            "qty": "10",
            "realizedPnl": "0.00123456",
            "marginAsset": "ETH",
            "baseQty": "0.00312109",
            "commission": "0.00000125",
            "commissionAsset": "ETH",
            "time": 1625184000000,
            "positionSide": "LONG",
            "buyer": true,
            "maker": false
        }"#;

        let trade: AccountTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.symbol, "ETHUSD_PERP");
        assert_eq!(trade.id, 315970407);
        assert_eq!(trade.order_id, 569935691);
        assert_eq!(trade.pair, "ETHUSD");
        assert_eq!(trade.side, OrderSide::Buy);
        assert_eq!(trade.price, "3200.50");
        assert_eq!(trade.quantity, "10");
        assert_eq!(trade.realized_pnl, "0.00123456");
        assert_eq!(trade.margin_asset, "ETH");
        assert_eq!(trade.base_qty, "0.00312109");
        assert_eq!(trade.commission, "0.00000125");
        assert_eq!(trade.commission_asset, "ETH");
        assert_eq!(trade.time, 1625184000000);
        assert_eq!(trade.position_side, PositionSide::Long);
        assert!(trade.buyer);
        assert!(!trade.maker);
    }

    #[test]
    fn test_account_trades_list_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSD_PERP",
                "id": 123456789,
                "orderId": 987654321,
                "pair": "BTCUSD",
                "side": "BUY",
                "price": "50000.0",
                "qty": "5",
                "realizedPnl": "0.00010000",
                "marginAsset": "BTC",
                "baseQty": "0.00010000",
                "commission": "0.00000010",
                "commissionAsset": "BTC",
                "time": 1625097600000,
                "positionSide": "LONG",
                "buyer": true,
                "maker": true
            },
            {
                "symbol": "BTCUSD_PERP",
                "id": 123456790,
                "orderId": 987654322,
                "pair": "BTCUSD",
                "side": "SELL",
                "price": "51000.0",
                "qty": "5",
                "realizedPnl": "0.00019608",
                "marginAsset": "BTC",
                "baseQty": "0.00009804",
                "commission": "0.00000010",
                "commissionAsset": "BTC",
                "time": 1625097700000,
                "positionSide": "SHORT",
                "buyer": false,
                "maker": false
            }
        ]"#;

        let trades: Vec<AccountTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 2);

        assert_eq!(trades[0].id, 123456789);
        assert_eq!(trades[0].side, OrderSide::Buy);
        assert_eq!(trades[0].price, "50000.0");
        assert!(trades[0].maker);

        assert_eq!(trades[1].id, 123456790);
        assert_eq!(trades[1].side, OrderSide::Sell);
        assert_eq!(trades[1].price, "51000.0");
        assert!(!trades[1].maker);
    }
}
