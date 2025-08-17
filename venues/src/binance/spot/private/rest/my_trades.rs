use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

const GET_MY_TRADES_ENDPOINT: &str = "/api/v3/myTrades";

/// Request parameters for getting account trades
#[derive(Debug, Clone, Serialize)]
pub struct MyTradesRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID - if set, it will get trades for this order
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Start time timestamp in ms
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time timestamp in ms
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Trade ID to fetch from
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,

    /// Default 500; max 1000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Account trade information
#[derive(Debug, Clone, Deserialize)]
pub struct MyTrade {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Trade ID
    #[serde(rename = "id")]
    pub id: u64,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Order list ID
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,

    /// Trade price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Trade quantity
    #[serde(rename = "qty")]
    pub qty: Decimal,

    /// Quote quantity
    #[serde(rename = "quoteQty")]
    pub quote_qty: Decimal,

    /// Commission amount
    #[serde(rename = "commission")]
    pub commission: Decimal,

    /// Commission asset
    #[serde(rename = "commissionAsset")]
    pub commission_asset: String,

    /// Trade execution time
    #[serde(rename = "time")]
    pub time: u64,

    /// Is buyer
    #[serde(rename = "isBuyer")]
    pub is_buyer: bool,

    /// Is maker
    #[serde(rename = "isMaker")]
    pub is_maker: bool,

    /// Is best match
    #[serde(rename = "isBestMatch")]
    pub is_best_match: bool,
}

impl RestClient {
    /// Get trades for a specific account and symbol
    ///
    /// Get trades for a specific account and symbol.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#account-trade-list--user_data)
    ///
    /// Method: GET /api/v3/myTrades
    /// Weight: 20 (without orderId), 5 (with orderId)
    /// Security: USER_DATA
    pub async fn get_my_trades(&self, params: MyTradesRequest) -> RestResult<Vec<MyTrade>> {
        let weight = if params.order_id.is_some() { 5 } else { 20 };

        self.send_get_signed_request(GET_MY_TRADES_ENDPOINT, params, weight, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_trades_request_serialization_minimal() {
        let request = MyTradesRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: None,
            start_time: None,
            end_time: None,
            from_id: None,
            limit: None,
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_my_trades_request_serialization_with_order_id() {
        let request = MyTradesRequest {
            symbol: "ETHUSDT".to_string(),
            order_id: Some(123456789),
            start_time: None,
            end_time: None,
            from_id: None,
            limit: None,
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSDT"));
        assert!(serialized.contains("orderId=123456789"));
    }

    #[test]
    fn test_my_trades_request_serialization_with_time_range() {
        let request = MyTradesRequest {
            symbol: "BNBUSDT".to_string(),
            order_id: None,
            start_time: Some(1625184000000),
            end_time: Some(1625270400000),
            from_id: None,
            limit: None,
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BNBUSDT"));
        assert!(serialized.contains("startTime=1625184000000"));
        assert!(serialized.contains("endTime=1625270400000"));
    }

    #[test]
    fn test_my_trades_request_serialization_full() {
        let request = MyTradesRequest {
            symbol: "ADAUSDT".to_string(),
            order_id: Some(987654321),
            start_time: Some(1625184000000),
            end_time: Some(1625270400000),
            from_id: Some(1000000),
            limit: Some(500),
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ADAUSDT"));
        assert!(serialized.contains("orderId=987654321"));
        assert!(serialized.contains("startTime=1625184000000"));
        assert!(serialized.contains("endTime=1625270400000"));
        assert!(serialized.contains("fromId=1000000"));
        assert!(serialized.contains("limit=500"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_my_trades_request_serialization_with_limit_and_from_id() {
        let request = MyTradesRequest {
            symbol: "DOTUSDT".to_string(),
            order_id: None,
            start_time: None,
            end_time: None,
            from_id: Some(2000000),
            limit: Some(1000),
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=DOTUSDT"));
        assert!(serialized.contains("fromId=2000000"));
        assert!(serialized.contains("limit=1000"));
        assert!(!serialized.contains("orderId"));
        assert!(!serialized.contains("startTime"));
        assert!(!serialized.contains("endTime"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_my_trade_deserialization_basic() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "id": 1234567,
            "orderId": 987654321,
            "orderListId": -1,
            "price": "50000.00000000",
            "qty": "0.10000000",
            "quoteQty": "5000.00000000",
            "commission": "0.00010000",
            "commissionAsset": "BTC",
            "time": 1625184000000,
            "isBuyer": true,
            "isMaker": false,
            "isBestMatch": true
        }"#;

        let trade: MyTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.symbol, "BTCUSDT");
        assert_eq!(trade.id, 1234567);
        assert_eq!(trade.order_id, 987654321);
        assert_eq!(trade.order_list_id, -1);
        assert_eq!(trade.price.to_string(), "50000.00000000");
        assert_eq!(trade.qty.to_string(), "0.10000000");
        assert_eq!(trade.quote_qty.to_string(), "5000.00000000");
        assert_eq!(trade.commission.to_string(), "0.00010000");
        assert_eq!(trade.commission_asset, "BTC");
        assert_eq!(trade.time, 1625184000000);
        assert!(trade.is_buyer);
        assert!(!trade.is_maker);
        assert!(trade.is_best_match);
    }

    #[test]
    fn test_my_trade_deserialization_seller_taker() {
        let json = r#"{
            "symbol": "ETHUSDT",
            "id": 7654321,
            "orderId": 123456789,
            "orderListId": 0,
            "price": "3000.50000000",
            "qty": "1.50000000",
            "quoteQty": "4500.75000000",
            "commission": "4.50075000",
            "commissionAsset": "USDT",
            "time": 1625270400000,
            "isBuyer": false,
            "isMaker": true,
            "isBestMatch": false
        }"#;

        let trade: MyTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.symbol, "ETHUSDT");
        assert_eq!(trade.id, 7654321);
        assert_eq!(trade.order_id, 123456789);
        assert_eq!(trade.order_list_id, 0);
        assert_eq!(trade.price.to_string(), "3000.50000000");
        assert_eq!(trade.qty.to_string(), "1.50000000");
        assert_eq!(trade.quote_qty.to_string(), "4500.75000000");
        assert_eq!(trade.commission.to_string(), "4.50075000");
        assert_eq!(trade.commission_asset, "USDT");
        assert_eq!(trade.time, 1625270400000);
        assert!(!trade.is_buyer);
        assert!(trade.is_maker);
        assert!(!trade.is_best_match);
    }

    #[test]
    fn test_my_trade_deserialization_with_order_list() {
        let json = r#"{
            "symbol": "BNBUSDT",
            "id": 9999999,
            "orderId": 111111111,
            "orderListId": 555555,
            "price": "400.25000000",
            "qty": "2.00000000",
            "quoteQty": "800.50000000",
            "commission": "0.00200000",
            "commissionAsset": "BNB",
            "time": 1625356800000,
            "isBuyer": true,
            "isMaker": true,
            "isBestMatch": true
        }"#;

        let trade: MyTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.symbol, "BNBUSDT");
        assert_eq!(trade.id, 9999999);
        assert_eq!(trade.order_id, 111111111);
        assert_eq!(trade.order_list_id, 555555);
        assert_eq!(trade.price.to_string(), "400.25000000");
        assert_eq!(trade.qty.to_string(), "2.00000000");
        assert_eq!(trade.quote_qty.to_string(), "800.50000000");
        assert_eq!(trade.commission.to_string(), "0.00200000");
        assert_eq!(trade.commission_asset, "BNB");
        assert_eq!(trade.time, 1625356800000);
        assert!(trade.is_buyer);
        assert!(trade.is_maker);
        assert!(trade.is_best_match);
    }

    #[test]
    fn test_my_trades_array_deserialization_empty() {
        let json = "[]";
        let trades: Vec<MyTrade> = serde_json::from_str(json).unwrap();
        assert!(trades.is_empty());
    }

    #[test]
    fn test_my_trades_array_deserialization_multiple() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "id": 1000001,
                "orderId": 2000001,
                "orderListId": -1,
                "price": "45000.00000000",
                "qty": "0.05000000",
                "quoteQty": "2250.00000000",
                "commission": "0.00005000",
                "commissionAsset": "BTC",
                "time": 1625184000000,
                "isBuyer": true,
                "isMaker": false,
                "isBestMatch": true
            },
            {
                "symbol": "BTCUSDT",
                "id": 1000002,
                "orderId": 2000001,
                "orderListId": -1,
                "price": "45100.00000000",
                "qty": "0.05000000",
                "quoteQty": "2255.00000000",
                "commission": "0.00005000",
                "commissionAsset": "BTC",
                "time": 1625184060000,
                "isBuyer": true,
                "isMaker": false,
                "isBestMatch": true
            },
            {
                "symbol": "BTCUSDT",
                "id": 1000003,
                "orderId": 2000002,
                "orderListId": -1,
                "price": "44900.00000000",
                "qty": "0.10000000",
                "quoteQty": "4490.00000000",
                "commission": "4.49000000",
                "commissionAsset": "USDT",
                "time": 1625184120000,
                "isBuyer": false,
                "isMaker": true,
                "isBestMatch": false
            }
        ]"#;

        let trades: Vec<MyTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 3);

        // Verify first trade
        assert_eq!(trades[0].symbol, "BTCUSDT");
        assert_eq!(trades[0].id, 1000001);
        assert_eq!(trades[0].order_id, 2000001);
        assert_eq!(trades[0].price.to_string(), "45000.00000000");
        assert!(trades[0].is_buyer);
        assert!(!trades[0].is_maker);

        // Verify second trade
        assert_eq!(trades[1].id, 1000002);
        assert_eq!(trades[1].order_id, 2000001); // Same order ID as first trade
        assert_eq!(trades[1].price.to_string(), "45100.00000000");

        // Verify third trade
        assert_eq!(trades[2].id, 1000003);
        assert_eq!(trades[2].order_id, 2000002); // Different order ID
        assert!(!trades[2].is_buyer);
        assert!(trades[2].is_maker);
        assert_eq!(trades[2].commission_asset, "USDT");
    }

    #[test]
    fn test_my_trade_with_extreme_values() {
        let json = r#"{
            "symbol": "SHIBUSDT",
            "id": 999999999999,
            "orderId": 888888888888,
            "orderListId": 777777777,
            "price": "0.00001234",
            "qty": "1000000000.00000000",
            "quoteQty": "12340.00000000",
            "commission": "0.00000001",
            "commissionAsset": "SHIB",
            "time": 9999999999999,
            "isBuyer": false,
            "isMaker": false,
            "isBestMatch": false
        }"#;

        let trade: MyTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.symbol, "SHIBUSDT");
        assert_eq!(trade.id, 999999999999);
        assert_eq!(trade.order_id, 888888888888);
        assert_eq!(trade.order_list_id, 777777777);
        assert_eq!(trade.price.to_string(), "0.00001234");
        assert_eq!(trade.qty.to_string(), "1000000000.00000000");
        assert_eq!(trade.quote_qty.to_string(), "12340.00000000");
        assert_eq!(trade.commission.to_string(), "0.00000001");
        assert_eq!(trade.commission_asset, "SHIB");
        assert_eq!(trade.time, 9999999999999);
        assert!(!trade.is_buyer);
        assert!(!trade.is_maker);
        assert!(!trade.is_best_match);
    }

    #[test]
    fn test_my_trade_with_zero_commission() {
        let json = r#"{
            "symbol": "ADAUSDT",
            "id": 5555555,
            "orderId": 6666666,
            "orderListId": -1,
            "price": "1.25000000",
            "qty": "100.00000000",
            "quoteQty": "125.00000000",
            "commission": "0.00000000",
            "commissionAsset": "ADA",
            "time": 1625443200000,
            "isBuyer": true,
            "isMaker": true,
            "isBestMatch": true
        }"#;

        let trade: MyTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.symbol, "ADAUSDT");
        assert_eq!(trade.commission.to_string(), "0.00000000");
        assert_eq!(trade.commission_asset, "ADA");
    }
}
