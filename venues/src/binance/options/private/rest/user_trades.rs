use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::{OptionsContractType, OptionsOrderSide, RestResult};

const GET_USER_TRADES_ENDPOINT: &str = "/eapi/v1/userTrades";

/// Request parameters for querying user trades
#[derive(Debug, Clone, Serialize)]
pub struct UserTradesRequest {
    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Trade ID to start from (returns trades with ID >= this value)
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,

    /// Start time
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of result sets returned (default: 100, max: 1000)
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// User trade record
#[derive(Debug, Clone, Deserialize)]
pub struct UserTrade {
    /// Trade ID
    #[serde(rename = "id")]
    pub id: u64,

    /// Trade ID (same as id)
    #[serde(rename = "tradeId")]
    pub trade_id: u64,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Trade price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Trade quantity
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    /// Quote quantity
    #[serde(rename = "quoteQty")]
    pub quote_qty: Decimal,

    /// Buy/sell direction
    #[serde(rename = "side")]
    pub side: OptionsOrderSide,

    /// Fee amount
    #[serde(rename = "fee")]
    pub fee: Decimal,

    /// Realized profit/loss
    #[serde(rename = "realizedPnl")]
    pub realized_pnl: Decimal,

    /// Trade time
    #[serde(rename = "time")]
    pub time: u64,

    /// Volatility
    #[serde(rename = "volatility")]
    pub volatility: Decimal,

    /// Volatility for Greeks calculation
    #[serde(rename = "volatilityForGreeks")]
    pub volatility_for_greeks: Decimal,

    /// Underlying price
    #[serde(rename = "underlyingPrice")]
    pub underlying_price: Decimal,

    /// Underlying price for Greeks calculation
    #[serde(rename = "underlyingPriceForGreeks")]
    pub underlying_price_for_greeks: Decimal,

    /// Vega
    #[serde(rename = "vega")]
    pub vega: Decimal,

    /// Delta
    #[serde(rename = "delta")]
    pub delta: Decimal,

    /// Gamma
    #[serde(rename = "gamma")]
    pub gamma: Decimal,

    /// Theta
    #[serde(rename = "theta")]
    pub theta: Decimal,

    /// Price scale
    #[serde(rename = "priceScale")]
    pub price_scale: u32,

    /// Quantity scale
    #[serde(rename = "quantityScale")]
    pub quantity_scale: u32,

    /// Option side (CALL or PUT)
    #[serde(rename = "optionSide")]
    pub option_side: OptionsContractType,

    /// Quote asset
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
}

impl RestClient {
    /// Query user trades
    ///
    /// Returns user's trade history for the specified symbol.
    ///
    /// [docs]: (https://developers.binance.com/docs/derivatives/option/trade/Option-Trade-History)
    /// Method: GET /eapi/v1/userTrades
    /// Weight: 5
    /// Requires: API key and signature
    pub async fn get_user_trades(&self, params: UserTradesRequest) -> RestResult<Vec<UserTrade>> {
        self.send_get_signed_request(
            GET_USER_TRADES_ENDPOINT,
            params,
            5,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_user_trades_request_minimal_serialization() {
        // Test minimal request with only required fields
        let request = UserTradesRequest {
            symbol: "BTC-240329-50000-C".to_string(),
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTC-240329-50000-C");
        assert_eq!(json["timestamp"], 1640995200000u64);
        
        // Ensure optional fields are not serialized when None
        assert!(json.get("fromId").is_none());
        assert!(json.get("startTime").is_none());
        assert!(json.get("endTime").is_none());
        assert!(json.get("limit").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_user_trades_request_with_from_id_serialization() {
        // Test request with from_id parameter
        let request = UserTradesRequest {
            symbol: "ETH-240329-3000-P".to_string(),
            from_id: Some(123456789),
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETH-240329-3000-P");
        assert_eq!(json["fromId"], 123456789);
        assert_eq!(json["timestamp"], 1640995200000u64);
        
        // Ensure other optional fields are not serialized
        assert!(json.get("startTime").is_none());
        assert!(json.get("endTime").is_none());
        assert!(json.get("limit").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_user_trades_request_with_time_range_serialization() {
        // Test request with time range parameters
        let request = UserTradesRequest {
            symbol: "BTC-240329-50000-C".to_string(),
            from_id: None,
            start_time: Some(1640995200000),
            end_time: Some(1640998800000),
            limit: None,
            recv_window: None,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTC-240329-50000-C");
        assert_eq!(json["startTime"], 1640995200000u64);
        assert_eq!(json["endTime"], 1640998800000u64);
        assert_eq!(json["timestamp"], 1640995200000u64);
        
        // Ensure other optional fields are not serialized
        assert!(json.get("fromId").is_none());
        assert!(json.get("limit").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_user_trades_request_with_limit_serialization() {
        // Test request with limit parameter
        let request = UserTradesRequest {
            symbol: "ETH-240329-3000-P".to_string(),
            from_id: None,
            start_time: None,
            end_time: None,
            limit: Some(500),
            recv_window: None,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETH-240329-3000-P");
        assert_eq!(json["limit"], 500);
        assert_eq!(json["timestamp"], 1640995200000u64);
        
        // Ensure other optional fields are not serialized
        assert!(json.get("fromId").is_none());
        assert!(json.get("startTime").is_none());
        assert!(json.get("endTime").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_user_trades_request_with_recv_window_serialization() {
        // Test request with recv_window parameter
        let request = UserTradesRequest {
            symbol: "BTC-240329-50000-C".to_string(),
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: Some(60000),
            timestamp: 1640995200000,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTC-240329-50000-C");
        assert_eq!(json["recvWindow"], 60000);
        assert_eq!(json["timestamp"], 1640995200000u64);
        
        // Ensure other optional fields are not serialized
        assert!(json.get("fromId").is_none());
        assert!(json.get("startTime").is_none());
        assert!(json.get("endTime").is_none());
        assert!(json.get("limit").is_none());
    }

    #[test]
    fn test_user_trades_request_all_fields_serialization() {
        // Test request with all fields populated
        let request = UserTradesRequest {
            symbol: "BTC-240329-50000-C".to_string(),
            from_id: Some(987654321),
            start_time: Some(1640995200000),
            end_time: Some(1640998800000),
            limit: Some(1000),
            recv_window: Some(60000),
            timestamp: 1640995200000,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTC-240329-50000-C");
        assert_eq!(json["fromId"], 987654321);
        assert_eq!(json["startTime"], 1640995200000u64);
        assert_eq!(json["endTime"], 1640998800000u64);
        assert_eq!(json["limit"], 1000);
        assert_eq!(json["recvWindow"], 60000);
        assert_eq!(json["timestamp"], 1640995200000u64);
    }

    #[test]
    fn test_user_trades_request_edge_cases() {
        // Test edge cases for numeric values
        let request = UserTradesRequest {
            symbol: "BTC-240329-50000-C".to_string(),
            from_id: Some(0), // Minimum value
            start_time: Some(0),
            end_time: Some(u64::MAX),
            limit: Some(1), // Minimum limit
            recv_window: Some(1), // Minimum recv_window
            timestamp: u64::MAX,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["fromId"], 0);
        assert_eq!(json["startTime"], 0);
        assert_eq!(json["endTime"], u64::MAX);
        assert_eq!(json["limit"], 1);
        assert_eq!(json["recvWindow"], 1);
        assert_eq!(json["timestamp"], u64::MAX);

        // Test maximum values
        let request = UserTradesRequest {
            symbol: "ETH-240329-3000-P".to_string(),
            from_id: Some(u64::MAX),
            start_time: Some(u64::MAX),
            end_time: Some(u64::MAX),
            limit: Some(1000), // Maximum limit
            recv_window: Some(60000), // Maximum recv_window
            timestamp: u64::MAX,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["fromId"], u64::MAX);
        assert_eq!(json["startTime"], u64::MAX);
        assert_eq!(json["endTime"], u64::MAX);
        assert_eq!(json["limit"], 1000);
        assert_eq!(json["recvWindow"], 60000);
        assert_eq!(json["timestamp"], u64::MAX);
    }

    #[test]
    fn test_user_trades_request_different_symbols() {
        // Test various symbol formats
        let symbols = vec![
            "BTC-240329-50000-C",
            "ETH-240329-3000-P",
            "BTC-240630-60000-C",
            "ETH-240630-2500-P",
            "BTC-241229-45000-C",
            "ETH-241229-3500-P",
        ];

        for symbol in symbols {
            let request = UserTradesRequest {
                symbol: symbol.to_string(),
                from_id: None,
                start_time: None,
                end_time: None,
                limit: None,
                recv_window: None,
                timestamp: 1640995200000,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["symbol"], symbol);
        }
    }

    #[test]
    fn test_user_trade_buy_call_deserialization() {
        let json = r#"{
            "id": 4611686018427387904,
            "tradeId": 4611686018427387904,
            "orderId": 12569099453718797312,
            "symbol": "BTC-240329-50000-C",
            "price": "0.0015",
            "quantity": "5.0",
            "quoteQty": "0.0075",
            "side": "BUY",
            "fee": "0.0000037",
            "realizedPnl": "0.0",
            "time": 1640995200000,
            "volatility": "0.90",
            "volatilityForGreeks": "0.90",
            "underlyingPrice": "50000.0",
            "underlyingPriceForGreeks": "50000.0",
            "vega": "0.12345",
            "delta": "0.54321",
            "gamma": "0.00012",
            "theta": "-0.00500",
            "priceScale": 4,
            "quantityScale": 1,
            "optionSide": "CALL",
            "quoteAsset": "BTC"
        }"#;

        let trade: UserTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 4611686018427387904);
        assert_eq!(trade.trade_id, 4611686018427387904);
        assert_eq!(trade.order_id, 12569099453718797312);
        assert_eq!(trade.symbol, "BTC-240329-50000-C");
        assert_eq!(trade.price, dec!(0.0015));
        assert_eq!(trade.quantity, dec!(5.0));
        assert_eq!(trade.quote_qty, dec!(0.0075));
        assert_eq!(trade.side, OptionsOrderSide::Buy);
        assert_eq!(trade.fee, dec!(0.0000037));
        assert_eq!(trade.realized_pnl, dec!(0.0));
        assert_eq!(trade.time, 1640995200000);
        assert_eq!(trade.volatility, dec!(0.90));
        assert_eq!(trade.volatility_for_greeks, dec!(0.90));
        assert_eq!(trade.underlying_price, dec!(50000.0));
        assert_eq!(trade.underlying_price_for_greeks, dec!(50000.0));
        assert_eq!(trade.vega, dec!(0.12345));
        assert_eq!(trade.delta, dec!(0.54321));
        assert_eq!(trade.gamma, dec!(0.00012));
        assert_eq!(trade.theta, dec!(-0.00500));
        assert_eq!(trade.price_scale, 4);
        assert_eq!(trade.quantity_scale, 1);
        assert_eq!(trade.option_side, OptionsContractType::Call);
        assert_eq!(trade.quote_asset, "BTC");
    }

    #[test]
    fn test_user_trade_sell_put_deserialization() {
        let json = r#"{
            "id": 4611686018427387905,
            "tradeId": 4611686018427387905,
            "orderId": 12569099453718797313,
            "symbol": "ETH-240329-3000-P",
            "price": "0.0025",
            "quantity": "10.0",
            "quoteQty": "0.025",
            "side": "SELL",
            "fee": "0.0000125",
            "realizedPnl": "0.0020",
            "time": 1640995260000,
            "volatility": "0.85",
            "volatilityForGreeks": "0.85",
            "underlyingPrice": "3000.0",
            "underlyingPriceForGreeks": "3000.0",
            "vega": "0.09876",
            "delta": "-0.45678",
            "gamma": "0.00023",
            "theta": "-0.00750",
            "priceScale": 4,
            "quantityScale": 1,
            "optionSide": "PUT",
            "quoteAsset": "ETH"
        }"#;

        let trade: UserTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 4611686018427387905);
        assert_eq!(trade.trade_id, 4611686018427387905);
        assert_eq!(trade.order_id, 12569099453718797313);
        assert_eq!(trade.symbol, "ETH-240329-3000-P");
        assert_eq!(trade.price, dec!(0.0025));
        assert_eq!(trade.quantity, dec!(10.0));
        assert_eq!(trade.quote_qty, dec!(0.025));
        assert_eq!(trade.side, OptionsOrderSide::Sell);
        assert_eq!(trade.fee, dec!(0.0000125));
        assert_eq!(trade.realized_pnl, dec!(0.0020));
        assert_eq!(trade.time, 1640995260000);
        assert_eq!(trade.volatility, dec!(0.85));
        assert_eq!(trade.volatility_for_greeks, dec!(0.85));
        assert_eq!(trade.underlying_price, dec!(3000.0));
        assert_eq!(trade.underlying_price_for_greeks, dec!(3000.0));
        assert_eq!(trade.vega, dec!(0.09876));
        assert_eq!(trade.delta, dec!(-0.45678));
        assert_eq!(trade.gamma, dec!(0.00023));
        assert_eq!(trade.theta, dec!(-0.00750));
        assert_eq!(trade.price_scale, 4);
        assert_eq!(trade.quantity_scale, 1);
        assert_eq!(trade.option_side, OptionsContractType::Put);
        assert_eq!(trade.quote_asset, "ETH");
    }

    #[test]
    fn test_user_trade_negative_pnl_deserialization() {
        // Test trade with negative realized PnL
        let json = r#"{
            "id": 4611686018427387906,
            "tradeId": 4611686018427387906,
            "orderId": 12569099453718797314,
            "symbol": "BTC-240329-50000-C",
            "price": "0.0010",
            "quantity": "2.0",
            "quoteQty": "0.0020",
            "side": "BUY",
            "fee": "0.0000010",
            "realizedPnl": "-0.0015",
            "time": 1640995320000,
            "volatility": "0.95",
            "volatilityForGreeks": "0.95",
            "underlyingPrice": "49000.0",
            "underlyingPriceForGreeks": "49000.0",
            "vega": "0.08765",
            "delta": "0.43210",
            "gamma": "0.00015",
            "theta": "-0.00600",
            "priceScale": 4,
            "quantityScale": 1,
            "optionSide": "CALL",
            "quoteAsset": "BTC"
        }"#;

        let trade: UserTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.realized_pnl, dec!(-0.0015));
        assert_eq!(trade.side, OptionsOrderSide::Buy);
        assert_eq!(trade.option_side, OptionsContractType::Call);
        assert_eq!(trade.underlying_price, dec!(49000.0));
        assert_eq!(trade.delta, dec!(0.43210));
        assert_eq!(trade.theta, dec!(-0.00600));
    }

    #[test]
    fn test_user_trade_zero_values_deserialization() {
        // Test trade with zero values
        let json = r#"{
            "id": 4611686018427387907,
            "tradeId": 4611686018427387907,
            "orderId": 12569099453718797315,
            "symbol": "ETH-240329-3000-P",
            "price": "0.0000",
            "quantity": "0.0",
            "quoteQty": "0.0000",
            "side": "SELL",
            "fee": "0.0000000",
            "realizedPnl": "0.0000",
            "time": 1640995380000,
            "volatility": "0.00",
            "volatilityForGreeks": "0.00",
            "underlyingPrice": "0.0",
            "underlyingPriceForGreeks": "0.0",
            "vega": "0.00000",
            "delta": "0.00000",
            "gamma": "0.00000",
            "theta": "0.00000",
            "priceScale": 0,
            "quantityScale": 0,
            "optionSide": "PUT",
            "quoteAsset": "ETH"
        }"#;

        let trade: UserTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.price, dec!(0.0000));
        assert_eq!(trade.quantity, dec!(0.0));
        assert_eq!(trade.quote_qty, dec!(0.0000));
        assert_eq!(trade.fee, dec!(0.0000000));
        assert_eq!(trade.realized_pnl, dec!(0.0000));
        assert_eq!(trade.volatility, dec!(0.00));
        assert_eq!(trade.volatility_for_greeks, dec!(0.00));
        assert_eq!(trade.underlying_price, dec!(0.0));
        assert_eq!(trade.underlying_price_for_greeks, dec!(0.0));
        assert_eq!(trade.vega, dec!(0.00000));
        assert_eq!(trade.delta, dec!(0.00000));
        assert_eq!(trade.gamma, dec!(0.00000));
        assert_eq!(trade.theta, dec!(0.00000));
        assert_eq!(trade.price_scale, 0);
        assert_eq!(trade.quantity_scale, 0);
    }

    #[test]
    fn test_user_trade_high_precision_deserialization() {
        // Test trade with high precision decimal values
        let json = r#"{
            "id": 4611686018427387908,
            "tradeId": 4611686018427387908,
            "orderId": 12569099453718797316,
            "symbol": "BTC-240329-50000-C",
            "price": "0.123456789",
            "quantity": "9.87654321",
            "quoteQty": "1.219326310",
            "side": "BUY",
            "fee": "0.0006096631",
            "realizedPnl": "0.001234567",
            "time": 1640995440000,
            "volatility": "0.876543210",
            "volatilityForGreeks": "0.876543210",
            "underlyingPrice": "50123.456789",
            "underlyingPriceForGreeks": "50123.456789",
            "vega": "0.123456789",
            "delta": "0.987654321",
            "gamma": "0.000123456",
            "theta": "-0.009876543",
            "priceScale": 9,
            "quantityScale": 8,
            "optionSide": "CALL",
            "quoteAsset": "BTC"
        }"#;

        let trade: UserTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.price, dec!(0.123456789));
        assert_eq!(trade.quantity, dec!(9.87654321));
        assert_eq!(trade.quote_qty, dec!(1.219326310));
        assert_eq!(trade.fee, dec!(0.0006096631));
        assert_eq!(trade.realized_pnl, dec!(0.001234567));
        assert_eq!(trade.volatility, dec!(0.876543210));
        assert_eq!(trade.volatility_for_greeks, dec!(0.876543210));
        assert_eq!(trade.underlying_price, dec!(50123.456789));
        assert_eq!(trade.underlying_price_for_greeks, dec!(50123.456789));
        assert_eq!(trade.vega, dec!(0.123456789));
        assert_eq!(trade.delta, dec!(0.987654321));
        assert_eq!(trade.gamma, dec!(0.000123456));
        assert_eq!(trade.theta, dec!(-0.009876543));
        assert_eq!(trade.price_scale, 9);
        assert_eq!(trade.quantity_scale, 8);
    }

    #[test]
    fn test_user_trade_extreme_values_deserialization() {
        // Test trade with extreme values
        let json = r#"{
            "id": 18446744073709551615,
            "tradeId": 18446744073709551615,
            "orderId": 18446744073709551615,
            "symbol": "BTC-240329-50000-C",
            "price": "999999999.999999999",
            "quantity": "999999999.999999999",
            "quoteQty": "999999999.999999999",
            "side": "SELL",
            "fee": "999999999.999999999",
            "realizedPnl": "-999999999.999999999",
            "time": 18446744073709551615,
            "volatility": "999.999999999",
            "volatilityForGreeks": "999.999999999",
            "underlyingPrice": "999999999.999999999",
            "underlyingPriceForGreeks": "999999999.999999999",
            "vega": "999999999.999999999",
            "delta": "1.000000000",
            "gamma": "999999999.999999999",
            "theta": "-999999999.999999999",
            "priceScale": 4294967295,
            "quantityScale": 4294967295,
            "optionSide": "CALL",
            "quoteAsset": "BTC"
        }"#;

        let trade: UserTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, u64::MAX);
        assert_eq!(trade.trade_id, u64::MAX);
        assert_eq!(trade.order_id, u64::MAX);
        assert_eq!(trade.price, dec!(999999999.999999999));
        assert_eq!(trade.quantity, dec!(999999999.999999999));
        assert_eq!(trade.quote_qty, dec!(999999999.999999999));
        assert_eq!(trade.fee, dec!(999999999.999999999));
        assert_eq!(trade.realized_pnl, dec!(-999999999.999999999));
        assert_eq!(trade.time, u64::MAX);
        assert_eq!(trade.volatility, dec!(999.999999999));
        assert_eq!(trade.volatility_for_greeks, dec!(999.999999999));
        assert_eq!(trade.underlying_price, dec!(999999999.999999999));
        assert_eq!(trade.underlying_price_for_greeks, dec!(999999999.999999999));
        assert_eq!(trade.vega, dec!(999999999.999999999));
        assert_eq!(trade.delta, dec!(1.000000000));
        assert_eq!(trade.gamma, dec!(999999999.999999999));
        assert_eq!(trade.theta, dec!(-999999999.999999999));
        assert_eq!(trade.price_scale, u32::MAX);
        assert_eq!(trade.quantity_scale, u32::MAX);
    }

    #[test]
    fn test_user_trade_different_assets_deserialization() {
        // Test various asset types
        let assets = vec![
            ("BTC-240329-50000-C", "BTC", OptionsContractType::Call),
            ("ETH-240329-3000-P", "ETH", OptionsContractType::Put),
            ("BTC-240630-60000-C", "BTC", OptionsContractType::Call),
            ("ETH-240630-2500-P", "ETH", OptionsContractType::Put),
            ("BTC-241229-45000-C", "BTC", OptionsContractType::Call),
            ("ETH-241229-3500-P", "ETH", OptionsContractType::Put),
        ];

        for (symbol, quote_asset, option_side) in assets {
            let json = format!(r#"{{
                "id": 4611686018427387909,
                "tradeId": 4611686018427387909,
                "orderId": 12569099453718797317,
                "symbol": "{}",
                "price": "0.0015",
                "quantity": "5.0",
                "quoteQty": "0.0075",
                "side": "BUY",
                "fee": "0.0000037",
                "realizedPnl": "0.0",
                "time": 1640995200000,
                "volatility": "0.90",
                "volatilityForGreeks": "0.90",
                "underlyingPrice": "50000.0",
                "underlyingPriceForGreeks": "50000.0",
                "vega": "0.12345",
                "delta": "0.54321",
                "gamma": "0.00012",
                "theta": "-0.00500",
                "priceScale": 4,
                "quantityScale": 1,
                "optionSide": "{}",
                "quoteAsset": "{}"
            }}"#, symbol, option_side, quote_asset);

            let trade: UserTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.symbol, symbol);
            assert_eq!(trade.quote_asset, quote_asset);
            assert_eq!(trade.option_side, option_side);
        }
    }

    #[test]
    fn test_user_trade_buy_and_sell_sides_deserialization() {
        // Test both buy and sell sides
        let sides = vec![
            ("BUY", OptionsOrderSide::Buy),
            ("SELL", OptionsOrderSide::Sell),
        ];

        for (side_str, expected_side) in sides {
            let json = format!(r#"{{
                "id": 4611686018427387910,
                "tradeId": 4611686018427387910,
                "orderId": 12569099453718797318,
                "symbol": "BTC-240329-50000-C",
                "price": "0.0015",
                "quantity": "5.0",
                "quoteQty": "0.0075",
                "side": "{}",
                "fee": "0.0000037",
                "realizedPnl": "0.0",
                "time": 1640995200000,
                "volatility": "0.90",
                "volatilityForGreeks": "0.90",
                "underlyingPrice": "50000.0",
                "underlyingPriceForGreeks": "50000.0",
                "vega": "0.12345",
                "delta": "0.54321",
                "gamma": "0.00012",
                "theta": "-0.00500",
                "priceScale": 4,
                "quantityScale": 1,
                "optionSide": "CALL",
                "quoteAsset": "BTC"
            }}"#, side_str);

            let trade: UserTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.side, expected_side);
        }
    }

    #[test]
    fn test_user_trade_call_and_put_options_deserialization() {
        // Test both call and put options
        let option_sides = vec![
            ("CALL", OptionsContractType::Call),
            ("PUT", OptionsContractType::Put),
        ];

        for (option_side_str, expected_option_side) in option_sides {
            let json = format!(r#"{{
                "id": 4611686018427387911,
                "tradeId": 4611686018427387911,
                "orderId": 12569099453718797319,
                "symbol": "BTC-240329-50000-C",
                "price": "0.0015",
                "quantity": "5.0",
                "quoteQty": "0.0075",
                "side": "BUY",
                "fee": "0.0000037",
                "realizedPnl": "0.0",
                "time": 1640995200000,
                "volatility": "0.90",
                "volatilityForGreeks": "0.90",
                "underlyingPrice": "50000.0",
                "underlyingPriceForGreeks": "50000.0",
                "vega": "0.12345",
                "delta": "0.54321",
                "gamma": "0.00012",
                "theta": "-0.00500",
                "priceScale": 4,
                "quantityScale": 1,
                "optionSide": "{}",
                "quoteAsset": "BTC"
            }}"#, option_side_str);

            let trade: UserTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.option_side, expected_option_side);
        }
    }

    #[test]
    fn test_user_trade_realistic_greeks_deserialization() {
        // Test realistic Greeks values for options
        let json = r#"{
            "id": 4611686018427387912,
            "tradeId": 4611686018427387912,
            "orderId": 12569099453718797320,
            "symbol": "BTC-240329-50000-C",
            "price": "0.0015",
            "quantity": "5.0",
            "quoteQty": "0.0075",
            "side": "BUY",
            "fee": "0.0000037",
            "realizedPnl": "0.0",
            "time": 1640995200000,
            "volatility": "0.90",
            "volatilityForGreeks": "0.90",
            "underlyingPrice": "50000.0",
            "underlyingPriceForGreeks": "50000.0",
            "vega": "0.12345",
            "delta": "0.54321",
            "gamma": "0.00012",
            "theta": "-0.00500",
            "priceScale": 4,
            "quantityScale": 1,
            "optionSide": "CALL",
            "quoteAsset": "BTC"
        }"#;

        let trade: UserTrade = serde_json::from_str(json).unwrap();
        
        // Verify Greeks have realistic values for a call option
        assert!(trade.delta > dec!(0) && trade.delta < dec!(1)); // Delta between 0 and 1 for call
        assert!(trade.gamma > dec!(0)); // Gamma is positive
        assert!(trade.theta < dec!(0)); // Theta is negative (time decay)
        assert!(trade.vega > dec!(0)); // Vega is positive
        assert!(trade.volatility > dec!(0)); // Volatility is positive
        assert!(trade.volatility_for_greeks > dec!(0)); // Volatility for Greeks is positive
        assert!(trade.underlying_price > dec!(0)); // Underlying price is positive
        assert!(trade.underlying_price_for_greeks > dec!(0)); // Underlying price for Greeks is positive
    }

    #[test]
    fn test_user_trade_put_greeks_deserialization() {
        // Test realistic Greeks values for put options
        let json = r#"{
            "id": 4611686018427387913,
            "tradeId": 4611686018427387913,
            "orderId": 12569099453718797321,
            "symbol": "ETH-240329-3000-P",
            "price": "0.0025",
            "quantity": "10.0",
            "quoteQty": "0.025",
            "side": "SELL",
            "fee": "0.0000125",
            "realizedPnl": "0.0020",
            "time": 1640995260000,
            "volatility": "0.85",
            "volatilityForGreeks": "0.85",
            "underlyingPrice": "3000.0",
            "underlyingPriceForGreeks": "3000.0",
            "vega": "0.09876",
            "delta": "-0.45678",
            "gamma": "0.00023",
            "theta": "-0.00750",
            "priceScale": 4,
            "quantityScale": 1,
            "optionSide": "PUT",
            "quoteAsset": "ETH"
        }"#;

        let trade: UserTrade = serde_json::from_str(json).unwrap();
        
        // Verify Greeks have realistic values for a put option
        assert!(trade.delta < dec!(0) && trade.delta > dec!(-1)); // Delta between -1 and 0 for put
        assert!(trade.gamma > dec!(0)); // Gamma is positive
        assert!(trade.theta < dec!(0)); // Theta is negative (time decay)
        assert!(trade.vega > dec!(0)); // Vega is positive
        assert!(trade.volatility > dec!(0)); // Volatility is positive
        assert!(trade.volatility_for_greeks > dec!(0)); // Volatility for Greeks is positive
        assert!(trade.underlying_price > dec!(0)); // Underlying price is positive
        assert!(trade.underlying_price_for_greeks > dec!(0)); // Underlying price for Greeks is positive
    }

    #[test]
    fn test_user_trade_array_deserialization() {
        // Test deserialization of an array of trades
        let json = r#"[
            {
                "id": 4611686018427387914,
                "tradeId": 4611686018427387914,
                "orderId": 12569099453718797322,
                "symbol": "BTC-240329-50000-C",
                "price": "0.0015",
                "quantity": "5.0",
                "quoteQty": "0.0075",
                "side": "BUY",
                "fee": "0.0000037",
                "realizedPnl": "0.0",
                "time": 1640995200000,
                "volatility": "0.90",
                "volatilityForGreeks": "0.90",
                "underlyingPrice": "50000.0",
                "underlyingPriceForGreeks": "50000.0",
                "vega": "0.12345",
                "delta": "0.54321",
                "gamma": "0.00012",
                "theta": "-0.00500",
                "priceScale": 4,
                "quantityScale": 1,
                "optionSide": "CALL",
                "quoteAsset": "BTC"
            },
            {
                "id": 4611686018427387915,
                "tradeId": 4611686018427387915,
                "orderId": 12569099453718797323,
                "symbol": "ETH-240329-3000-P",
                "price": "0.0025",
                "quantity": "10.0",
                "quoteQty": "0.025",
                "side": "SELL",
                "fee": "0.0000125",
                "realizedPnl": "0.0020",
                "time": 1640995260000,
                "volatility": "0.85",
                "volatilityForGreeks": "0.85",
                "underlyingPrice": "3000.0",
                "underlyingPriceForGreeks": "3000.0",
                "vega": "0.09876",
                "delta": "-0.45678",
                "gamma": "0.00023",
                "theta": "-0.00750",
                "priceScale": 4,
                "quantityScale": 1,
                "optionSide": "PUT",
                "quoteAsset": "ETH"
            }
        ]"#;

        let trades: Vec<UserTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 2);

        // Verify first trade (BTC call)
        let btc_trade = &trades[0];
        assert_eq!(btc_trade.id, 4611686018427387914);
        assert_eq!(btc_trade.symbol, "BTC-240329-50000-C");
        assert_eq!(btc_trade.side, OptionsOrderSide::Buy);
        assert_eq!(btc_trade.option_side, OptionsContractType::Call);
        assert_eq!(btc_trade.quote_asset, "BTC");

        // Verify second trade (ETH put)
        let eth_trade = &trades[1];
        assert_eq!(eth_trade.id, 4611686018427387915);
        assert_eq!(eth_trade.symbol, "ETH-240329-3000-P");
        assert_eq!(eth_trade.side, OptionsOrderSide::Sell);
        assert_eq!(eth_trade.option_side, OptionsContractType::Put);
        assert_eq!(eth_trade.quote_asset, "ETH");
    }

    #[test]
    fn test_user_trade_empty_array_deserialization() {
        // Test deserialization of an empty array
        let json = r#"[]"#;

        let trades: Vec<UserTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 0);
    }

    #[test]
    fn test_user_trade_mixed_profitability_scenario() {
        // Test realistic scenario with mixed profitable and loss trades
        let json = r#"[
            {
                "id": 4611686018427387916,
                "tradeId": 4611686018427387916,
                "orderId": 12569099453718797324,
                "symbol": "BTC-240329-50000-C",
                "price": "0.0020",
                "quantity": "3.0",
                "quoteQty": "0.0060",
                "side": "BUY",
                "fee": "0.0000030",
                "realizedPnl": "0.0015",
                "time": 1640995200000,
                "volatility": "0.88",
                "volatilityForGreeks": "0.88",
                "underlyingPrice": "52000.0",
                "underlyingPriceForGreeks": "52000.0",
                "vega": "0.15000",
                "delta": "0.65000",
                "gamma": "0.00018",
                "theta": "-0.00400",
                "priceScale": 4,
                "quantityScale": 1,
                "optionSide": "CALL",
                "quoteAsset": "BTC"
            },
            {
                "id": 4611686018427387917,
                "tradeId": 4611686018427387917,
                "orderId": 12569099453718797325,
                "symbol": "ETH-240329-3000-P",
                "price": "0.0030",
                "quantity": "8.0",
                "quoteQty": "0.0240",
                "side": "SELL",
                "fee": "0.0000120",
                "realizedPnl": "-0.0010",
                "time": 1640995260000,
                "volatility": "0.82",
                "volatilityForGreeks": "0.82",
                "underlyingPrice": "3100.0",
                "underlyingPriceForGreeks": "3100.0",
                "vega": "0.08500",
                "delta": "-0.35000",
                "gamma": "0.00020",
                "theta": "-0.00800",
                "priceScale": 4,
                "quantityScale": 1,
                "optionSide": "PUT",
                "quoteAsset": "ETH"
            }
        ]"#;

        let trades: Vec<UserTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 2);

        // Verify profitable trade
        let profitable_trade = &trades[0];
        assert!(profitable_trade.realized_pnl > dec!(0));
        assert_eq!(profitable_trade.side, OptionsOrderSide::Buy);
        assert_eq!(profitable_trade.option_side, OptionsContractType::Call);

        // Verify loss trade
        let loss_trade = &trades[1];
        assert!(loss_trade.realized_pnl < dec!(0));
        assert_eq!(loss_trade.side, OptionsOrderSide::Sell);
        assert_eq!(loss_trade.option_side, OptionsContractType::Put);
    }

    #[test]
    fn test_user_trade_consistent_id_fields() {
        // Test that id and tradeId are consistent
        let json = r#"{
            "id": 4611686018427387918,
            "tradeId": 4611686018427387918,
            "orderId": 12569099453718797326,
            "symbol": "BTC-240329-50000-C",
            "price": "0.0015",
            "quantity": "5.0",
            "quoteQty": "0.0075",
            "side": "BUY",
            "fee": "0.0000037",
            "realizedPnl": "0.0",
            "time": 1640995200000,
            "volatility": "0.90",
            "volatilityForGreeks": "0.90",
            "underlyingPrice": "50000.0",
            "underlyingPriceForGreeks": "50000.0",
            "vega": "0.12345",
            "delta": "0.54321",
            "gamma": "0.00012",
            "theta": "-0.00500",
            "priceScale": 4,
            "quantityScale": 1,
            "optionSide": "CALL",
            "quoteAsset": "BTC"
        }"#;

        let trade: UserTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, trade.trade_id);
        assert_eq!(trade.id, 4611686018427387918);
        assert_eq!(trade.trade_id, 4611686018427387918);
    }

    #[test]
    fn test_user_trade_scale_consistency() {
        // Test that price and quantity scales are used consistently
        let json = r#"{
            "id": 4611686018427387919,
            "tradeId": 4611686018427387919,
            "orderId": 12569099453718797327,
            "symbol": "BTC-240329-50000-C",
            "price": "0.0015",
            "quantity": "5.0",
            "quoteQty": "0.0075",
            "side": "BUY",
            "fee": "0.0000037",
            "realizedPnl": "0.0",
            "time": 1640995200000,
            "volatility": "0.90",
            "volatilityForGreeks": "0.90",
            "underlyingPrice": "50000.0",
            "underlyingPriceForGreeks": "50000.0",
            "vega": "0.12345",
            "delta": "0.54321",
            "gamma": "0.00012",
            "theta": "-0.00500",
            "priceScale": 4,
            "quantityScale": 1,
            "optionSide": "CALL",
            "quoteAsset": "BTC"
        }"#;

        let trade: UserTrade = serde_json::from_str(json).unwrap();
        
        // Verify scales are reasonable
        assert!(trade.price_scale <= 18); // Reasonable price scale
        assert!(trade.quantity_scale <= 18); // Reasonable quantity scale
        
        // Verify quote quantity matches price * quantity approximately
        let calculated_quote_qty = trade.price * trade.quantity;
        assert!((trade.quote_qty - calculated_quote_qty).abs() < dec!(0.000001));
    }
}
