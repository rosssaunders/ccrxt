use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::options::PrivateRestClient as RestClient;
use crate::binance::options::{OptionsContractType, OptionsPositionSide, RestResult};

const GET_POSITION_ENDPOINT: &str = "/eapi/v1/position";

/// Request parameters for querying position information
#[derive(Debug, Clone, Serialize, Default)]
pub struct PositionRequest {
    /// Option trading pair (if omitted, returns all positions)
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Position information
#[derive(Debug, Clone, Deserialize)]
pub struct Position {
    /// Average entry price
    #[serde(rename = "entryPrice")]
    pub entry_price: Decimal,

    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Position direction (LONG or SHORT)
    #[serde(rename = "side")]
    pub side: OptionsPositionSide,

    /// Number of positions (positive for long, negative for short)
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    /// Number of positions that can be reduced
    #[serde(rename = "reducibleQty")]
    pub reducible_qty: Decimal,

    /// Current market value
    #[serde(rename = "markValue")]
    pub mark_value: Decimal,

    /// Rate of return
    #[serde(rename = "ror")]
    pub ror: Decimal,

    /// Unrealized profit/loss
    #[serde(rename = "unrealizedPNL")]
    pub unrealized_pnl: Decimal,

    /// Mark price
    #[serde(rename = "markPrice")]
    pub mark_price: Decimal,

    /// Strike price
    #[serde(rename = "strikePrice")]
    pub strike_price: Decimal,

    /// Position cost
    #[serde(rename = "positionCost")]
    pub position_cost: Decimal,

    /// Exercise time (expiry date)
    #[serde(rename = "expiryDate")]
    pub expiry_date: u64,

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
    /// Get current position information
    ///
    /// Returns current position information for option contracts.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/option/trade/Option-Position-Information)
    ///
    /// Method: GET /eapi/v1/position
    /// Weight: 5
    /// Requires: API key and signature
    pub async fn get_position(&self, params: PositionRequest) -> RestResult<Vec<Position>> {
        self.send_get_signed_request(GET_POSITION_ENDPOINT, params, 5, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_position_request_minimal_serialization() {
        // Test minimal request with only required timestamp
        let request = PositionRequest {
            symbol: None,
            recv_window: None,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timestamp"], 1640995200000u64);

        // Ensure optional fields are not serialized when None
        assert!(json.get("symbol").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_position_request_with_symbol_serialization() {
        // Test request with symbol
        let request = PositionRequest {
            symbol: Some("BTC-240329-40000-C".to_string()),
            recv_window: None,
            timestamp: 1640995200000,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timestamp"], 1640995200000u64);
        assert_eq!(json["symbol"], "BTC-240329-40000-C");
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_position_request_with_recv_window_serialization() {
        // Test request with recv_window
        let request = PositionRequest {
            symbol: None,
            recv_window: Some(60000),
            timestamp: 1640995200000,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timestamp"], 1640995200000u64);
        assert_eq!(json["recvWindow"], 60000);
        assert!(json.get("symbol").is_none());
    }

    #[test]
    fn test_position_request_all_fields_serialization() {
        // Test request with all fields
        let request = PositionRequest {
            symbol: Some("ETH-240329-3000-P".to_string()),
            recv_window: Some(30000),
            timestamp: 1640995200000,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timestamp"], 1640995200000u64);
        assert_eq!(json["symbol"], "ETH-240329-3000-P");
        assert_eq!(json["recvWindow"], 30000);
    }

    #[test]
    fn test_position_request_default() {
        // Test default implementation creates a valid request
        let request = PositionRequest {
            timestamp: 1640995200000,
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timestamp"], 1640995200000u64);
        assert!(json.get("symbol").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_position_request_edge_cases() {
        // Test edge cases for numeric values
        let request = PositionRequest {
            symbol: Some("BTC-240329-50000-C".to_string()),
            recv_window: Some(1), // Minimum value
            timestamp: 0,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["recvWindow"], 1);
        assert_eq!(json["timestamp"], 0);
        assert_eq!(json["symbol"], "BTC-240329-50000-C");

        // Test maximum recv_window
        let request = PositionRequest {
            symbol: Some("ETH-240329-4000-P".to_string()),
            recv_window: Some(60000), // Maximum value
            timestamp: u64::MAX,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["recvWindow"], 60000);
        assert_eq!(json["timestamp"], u64::MAX);
        assert_eq!(json["symbol"], "ETH-240329-4000-P");
    }

    #[test]
    fn test_position_request_various_symbols() {
        // Test various symbol formats
        let symbols = vec![
            "BTC-240329-40000-C",
            "ETH-240329-3000-P",
            "BTC-240628-50000-C",
            "ETH-240628-2500-P",
            "BTC-241225-60000-C",
            "ETH-241225-4000-P",
        ];

        for symbol in symbols {
            let request = PositionRequest {
                symbol: Some(symbol.to_string()),
                recv_window: Some(5000),
                timestamp: 1640995200000,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["symbol"], symbol);
            assert_eq!(json["recvWindow"], 5000);
            assert_eq!(json["timestamp"], 1640995200000u64);
        }
    }

    #[test]
    fn test_position_long_call_deserialization() {
        let json = r#"{
            "entryPrice": "5000.50000000",
            "symbol": "BTC-240329-40000-C",
            "side": "LONG",
            "quantity": "10.00000000",
            "reducibleQty": "8.50000000",
            "markValue": "52000.00000000",
            "ror": "0.04000000",
            "unrealizedPNL": "2000.00000000",
            "markPrice": "5200.00000000",
            "strikePrice": "40000.00000000",
            "positionCost": "50005.00000000",
            "expiryDate": 1711699200000,
            "priceScale": 8,
            "quantityScale": 4,
            "optionSide": "CALL",
            "quoteAsset": "USDT"
        }"#;

        let position: Position = serde_json::from_str(json).unwrap();
        assert_eq!(position.entry_price, dec!(5000.50000000));
        assert_eq!(position.symbol, "BTC-240329-40000-C");
        assert_eq!(position.side, OptionsPositionSide::Long);
        assert_eq!(position.quantity, dec!(10.00000000));
        assert_eq!(position.reducible_qty, dec!(8.50000000));
        assert_eq!(position.mark_value, dec!(52000.00000000));
        assert_eq!(position.ror, dec!(0.04000000));
        assert_eq!(position.unrealized_pnl, dec!(2000.00000000));
        assert_eq!(position.mark_price, dec!(5200.00000000));
        assert_eq!(position.strike_price, dec!(40000.00000000));
        assert_eq!(position.position_cost, dec!(50005.00000000));
        assert_eq!(position.expiry_date, 1711699200000);
        assert_eq!(position.price_scale, 8);
        assert_eq!(position.quantity_scale, 4);
        assert_eq!(position.option_side, OptionsContractType::Call);
        assert_eq!(position.quote_asset, "USDT");
    }

    #[test]
    fn test_position_short_put_deserialization() {
        let json = r#"{
            "entryPrice": "2500.25000000",
            "symbol": "ETH-240329-3000-P",
            "side": "SHORT",
            "quantity": "-5.00000000",
            "reducibleQty": "4.75000000",
            "markValue": "12400.00000000",
            "ror": "-0.02500000",
            "unrealizedPNL": "-300.00000000",
            "markPrice": "2480.00000000",
            "strikePrice": "3000.00000000",
            "positionCost": "12500.00000000",
            "expiryDate": 1711699200000,
            "priceScale": 8,
            "quantityScale": 4,
            "optionSide": "PUT",
            "quoteAsset": "USDT"
        }"#;

        let position: Position = serde_json::from_str(json).unwrap();
        assert_eq!(position.entry_price, dec!(2500.25000000));
        assert_eq!(position.symbol, "ETH-240329-3000-P");
        assert_eq!(position.side, OptionsPositionSide::Short);
        assert_eq!(position.quantity, dec!(-5.00000000));
        assert_eq!(position.reducible_qty, dec!(4.75000000));
        assert_eq!(position.mark_value, dec!(12400.00000000));
        assert_eq!(position.ror, dec!(-0.02500000));
        assert_eq!(position.unrealized_pnl, dec!(-300.00000000));
        assert_eq!(position.mark_price, dec!(2480.00000000));
        assert_eq!(position.strike_price, dec!(3000.00000000));
        assert_eq!(position.position_cost, dec!(12500.00000000));
        assert_eq!(position.expiry_date, 1711699200000);
        assert_eq!(position.price_scale, 8);
        assert_eq!(position.quantity_scale, 4);
        assert_eq!(position.option_side, OptionsContractType::Put);
        assert_eq!(position.quote_asset, "USDT");
    }

    #[test]
    fn test_position_long_put_deserialization() {
        let json = r#"{
            "entryPrice": "800.75000000",
            "symbol": "BTC-240628-35000-P",
            "side": "LONG",
            "quantity": "2.50000000",
            "reducibleQty": "2.50000000",
            "markValue": "2000.00000000",
            "ror": "0.00000000",
            "unrealizedPNL": "0.00000000",
            "markPrice": "800.00000000",
            "strikePrice": "35000.00000000",
            "positionCost": "2001.875000000",
            "expiryDate": 1719590400000,
            "priceScale": 8,
            "quantityScale": 4,
            "optionSide": "PUT",
            "quoteAsset": "USDT"
        }"#;

        let position: Position = serde_json::from_str(json).unwrap();
        assert_eq!(position.entry_price, dec!(800.75000000));
        assert_eq!(position.symbol, "BTC-240628-35000-P");
        assert_eq!(position.side, OptionsPositionSide::Long);
        assert_eq!(position.quantity, dec!(2.50000000));
        assert_eq!(position.reducible_qty, dec!(2.50000000));
        assert_eq!(position.mark_value, dec!(2000.00000000));
        assert_eq!(position.ror, dec!(0.00000000));
        assert_eq!(position.unrealized_pnl, dec!(0.00000000));
        assert_eq!(position.mark_price, dec!(800.00000000));
        assert_eq!(position.strike_price, dec!(35000.00000000));
        assert_eq!(position.position_cost, dec!(2001.875000000));
        assert_eq!(position.expiry_date, 1719590400000);
        assert_eq!(position.price_scale, 8);
        assert_eq!(position.quantity_scale, 4);
        assert_eq!(position.option_side, OptionsContractType::Put);
        assert_eq!(position.quote_asset, "USDT");
    }

    #[test]
    fn test_position_short_call_deserialization() {
        let json = r#"{
            "entryPrice": "7500.00000000",
            "symbol": "ETH-240628-4000-C",
            "side": "SHORT",
            "quantity": "-1.00000000",
            "reducibleQty": "0.75000000",
            "markValue": "7800.00000000",
            "ror": "-0.04000000",
            "unrealizedPNL": "-300.00000000",
            "markPrice": "7800.00000000",
            "strikePrice": "4000.00000000",
            "positionCost": "7500.00000000",
            "expiryDate": 1719590400000,
            "priceScale": 8,
            "quantityScale": 4,
            "optionSide": "CALL",
            "quoteAsset": "USDT"
        }"#;

        let position: Position = serde_json::from_str(json).unwrap();
        assert_eq!(position.entry_price, dec!(7500.00000000));
        assert_eq!(position.symbol, "ETH-240628-4000-C");
        assert_eq!(position.side, OptionsPositionSide::Short);
        assert_eq!(position.quantity, dec!(-1.00000000));
        assert_eq!(position.reducible_qty, dec!(0.75000000));
        assert_eq!(position.mark_value, dec!(7800.00000000));
        assert_eq!(position.ror, dec!(-0.04000000));
        assert_eq!(position.unrealized_pnl, dec!(-300.00000000));
        assert_eq!(position.mark_price, dec!(7800.00000000));
        assert_eq!(position.strike_price, dec!(4000.00000000));
        assert_eq!(position.position_cost, dec!(7500.00000000));
        assert_eq!(position.expiry_date, 1719590400000);
        assert_eq!(position.price_scale, 8);
        assert_eq!(position.quantity_scale, 4);
        assert_eq!(position.option_side, OptionsContractType::Call);
        assert_eq!(position.quote_asset, "USDT");
    }

    #[test]
    fn test_position_zero_values_deserialization() {
        // Test position with zero values
        let json = r#"{
            "entryPrice": "0.00000000",
            "symbol": "BTC-240329-60000-C",
            "side": "LONG",
            "quantity": "0.00000000",
            "reducibleQty": "0.00000000",
            "markValue": "0.00000000",
            "ror": "0.00000000",
            "unrealizedPNL": "0.00000000",
            "markPrice": "0.00000000",
            "strikePrice": "60000.00000000",
            "positionCost": "0.00000000",
            "expiryDate": 1711699200000,
            "priceScale": 8,
            "quantityScale": 4,
            "optionSide": "CALL",
            "quoteAsset": "USDT"
        }"#;

        let position: Position = serde_json::from_str(json).unwrap();
        assert_eq!(position.entry_price, dec!(0.00000000));
        assert_eq!(position.symbol, "BTC-240329-60000-C");
        assert_eq!(position.side, OptionsPositionSide::Long);
        assert_eq!(position.quantity, dec!(0.00000000));
        assert_eq!(position.reducible_qty, dec!(0.00000000));
        assert_eq!(position.mark_value, dec!(0.00000000));
        assert_eq!(position.ror, dec!(0.00000000));
        assert_eq!(position.unrealized_pnl, dec!(0.00000000));
        assert_eq!(position.mark_price, dec!(0.00000000));
        assert_eq!(position.strike_price, dec!(60000.00000000));
        assert_eq!(position.position_cost, dec!(0.00000000));
        assert_eq!(position.expiry_date, 1711699200000);
        assert_eq!(position.price_scale, 8);
        assert_eq!(position.quantity_scale, 4);
        assert_eq!(position.option_side, OptionsContractType::Call);
        assert_eq!(position.quote_asset, "USDT");
    }

    #[test]
    fn test_position_high_precision_values_deserialization() {
        // Test position with high precision decimal values
        let json = r#"{
            "entryPrice": "12345.67890123",
            "symbol": "BTC-241225-45000-C",
            "side": "LONG",
            "quantity": "0.12345678",
            "reducibleQty": "0.11111111",
            "markValue": "1523.45678901",
            "ror": "0.23456789",
            "unrealizedPNL": "123.45678901",
            "markPrice": "12345.67890123",
            "strikePrice": "45000.00000000",
            "positionCost": "1523.45678901",
            "expiryDate": 1735084800000,
            "priceScale": 8,
            "quantityScale": 8,
            "optionSide": "CALL",
            "quoteAsset": "USDT"
        }"#;

        let position: Position = serde_json::from_str(json).unwrap();
        assert_eq!(position.entry_price, dec!(12345.67890123));
        assert_eq!(position.symbol, "BTC-241225-45000-C");
        assert_eq!(position.side, OptionsPositionSide::Long);
        assert_eq!(position.quantity, dec!(0.12345678));
        assert_eq!(position.reducible_qty, dec!(0.11111111));
        assert_eq!(position.mark_value, dec!(1523.45678901));
        assert_eq!(position.ror, dec!(0.23456789));
        assert_eq!(position.unrealized_pnl, dec!(123.45678901));
        assert_eq!(position.mark_price, dec!(12345.67890123));
        assert_eq!(position.strike_price, dec!(45000.00000000));
        assert_eq!(position.position_cost, dec!(1523.45678901));
        assert_eq!(position.expiry_date, 1735084800000);
        assert_eq!(position.price_scale, 8);
        assert_eq!(position.quantity_scale, 8);
        assert_eq!(position.option_side, OptionsContractType::Call);
        assert_eq!(position.quote_asset, "USDT");
    }

    #[test]
    fn test_position_negative_values_deserialization() {
        // Test position with negative values (loss scenario)
        let json = r#"{
            "entryPrice": "3000.00000000",
            "symbol": "ETH-240329-3500-C",
            "side": "LONG",
            "quantity": "5.00000000",
            "reducibleQty": "4.50000000",
            "markValue": "13500.00000000",
            "ror": "-0.10000000",
            "unrealizedPNL": "-1500.00000000",
            "markPrice": "2700.00000000",
            "strikePrice": "3500.00000000",
            "positionCost": "15000.00000000",
            "expiryDate": 1711699200000,
            "priceScale": 8,
            "quantityScale": 4,
            "optionSide": "CALL",
            "quoteAsset": "USDT"
        }"#;

        let position: Position = serde_json::from_str(json).unwrap();
        assert_eq!(position.entry_price, dec!(3000.00000000));
        assert_eq!(position.symbol, "ETH-240329-3500-C");
        assert_eq!(position.side, OptionsPositionSide::Long);
        assert_eq!(position.quantity, dec!(5.00000000));
        assert_eq!(position.reducible_qty, dec!(4.50000000));
        assert_eq!(position.mark_value, dec!(13500.00000000));
        assert_eq!(position.ror, dec!(-0.10000000));
        assert_eq!(position.unrealized_pnl, dec!(-1500.00000000));
        assert_eq!(position.mark_price, dec!(2700.00000000));
        assert_eq!(position.strike_price, dec!(3500.00000000));
        assert_eq!(position.position_cost, dec!(15000.00000000));
        assert_eq!(position.expiry_date, 1711699200000);
        assert_eq!(position.price_scale, 8);
        assert_eq!(position.quantity_scale, 4);
        assert_eq!(position.option_side, OptionsContractType::Call);
        assert_eq!(position.quote_asset, "USDT");
    }

    #[test]
    fn test_position_extreme_values_deserialization() {
        // Test position with extreme values
        let json = r#"{
            "entryPrice": "999999.99999999",
            "symbol": "BTC-241225-100000-C",
            "side": "LONG",
            "quantity": "0.00000001",
            "reducibleQty": "0.00000001",
            "markValue": "9.99999999",
            "ror": "999.99999999",
            "unrealizedPNL": "9.99999999",
            "markPrice": "999999.99999999",
            "strikePrice": "100000.00000000",
            "positionCost": "9.99999999",
            "expiryDate": 9223372036854775807,
            "priceScale": 8,
            "quantityScale": 8,
            "optionSide": "CALL",
            "quoteAsset": "USDT"
        }"#;

        let position: Position = serde_json::from_str(json).unwrap();
        assert_eq!(position.entry_price, dec!(999999.99999999));
        assert_eq!(position.symbol, "BTC-241225-100000-C");
        assert_eq!(position.side, OptionsPositionSide::Long);
        assert_eq!(position.quantity, dec!(0.00000001));
        assert_eq!(position.reducible_qty, dec!(0.00000001));
        assert_eq!(position.mark_value, dec!(9.99999999));
        assert_eq!(position.ror, dec!(999.99999999));
        assert_eq!(position.unrealized_pnl, dec!(9.99999999));
        assert_eq!(position.mark_price, dec!(999999.99999999));
        assert_eq!(position.strike_price, dec!(100000.00000000));
        assert_eq!(position.position_cost, dec!(9.99999999));
        assert_eq!(position.expiry_date, 9223372036854775807u64);
        assert_eq!(position.price_scale, 8);
        assert_eq!(position.quantity_scale, 8);
        assert_eq!(position.option_side, OptionsContractType::Call);
        assert_eq!(position.quote_asset, "USDT");
    }

    #[test]
    fn test_position_different_scales_deserialization() {
        // Test position with different price and quantity scales
        let json = r#"{
            "entryPrice": "1000.00000000",
            "symbol": "ETH-240329-2000-P",
            "side": "LONG",
            "quantity": "1.00000000",
            "reducibleQty": "1.00000000",
            "markValue": "1000.00000000",
            "ror": "0.00000000",
            "unrealizedPNL": "0.00000000",
            "markPrice": "1000.00000000",
            "strikePrice": "2000.00000000",
            "positionCost": "1000.00000000",
            "expiryDate": 1711699200000,
            "priceScale": 2,
            "quantityScale": 6,
            "optionSide": "PUT",
            "quoteAsset": "USDT"
        }"#;

        let position: Position = serde_json::from_str(json).unwrap();
        assert_eq!(position.entry_price, dec!(1000.00000000));
        assert_eq!(position.symbol, "ETH-240329-2000-P");
        assert_eq!(position.side, OptionsPositionSide::Long);
        assert_eq!(position.quantity, dec!(1.00000000));
        assert_eq!(position.reducible_qty, dec!(1.00000000));
        assert_eq!(position.mark_value, dec!(1000.00000000));
        assert_eq!(position.ror, dec!(0.00000000));
        assert_eq!(position.unrealized_pnl, dec!(0.00000000));
        assert_eq!(position.mark_price, dec!(1000.00000000));
        assert_eq!(position.strike_price, dec!(2000.00000000));
        assert_eq!(position.position_cost, dec!(1000.00000000));
        assert_eq!(position.expiry_date, 1711699200000);
        assert_eq!(position.price_scale, 2);
        assert_eq!(position.quantity_scale, 6);
        assert_eq!(position.option_side, OptionsContractType::Put);
        assert_eq!(position.quote_asset, "USDT");
    }

    #[test]
    fn test_position_different_quote_assets_deserialization() {
        // Test position with different quote assets
        let quote_assets = vec!["USDT", "BUSD", "BNB"];

        for quote_asset in quote_assets {
            let json = format!(
                r#"{{
                "entryPrice": "1000.00000000",
                "symbol": "BTC-240329-40000-C",
                "side": "LONG",
                "quantity": "1.00000000",
                "reducibleQty": "1.00000000",
                "markValue": "1000.00000000",
                "ror": "0.00000000",
                "unrealizedPNL": "0.00000000",
                "markPrice": "1000.00000000",
                "strikePrice": "40000.00000000",
                "positionCost": "1000.00000000",
                "expiryDate": 1711699200000,
                "priceScale": 8,
                "quantityScale": 4,
                "optionSide": "CALL",
                "quoteAsset": "{}"
            }}"#,
                quote_asset
            );

            let position: Position = serde_json::from_str(&json).unwrap();
            assert_eq!(position.quote_asset, quote_asset);
            assert_eq!(position.symbol, "BTC-240329-40000-C");
            assert_eq!(position.side, OptionsPositionSide::Long);
            assert_eq!(position.option_side, OptionsContractType::Call);
        }
    }

    #[test]
    fn test_position_all_option_sides_deserialization() {
        // Test both CALL and PUT option sides
        let option_sides = vec![
            ("CALL", OptionsContractType::Call),
            ("PUT", OptionsContractType::Put),
        ];

        for (side_str, expected_side) in option_sides {
            let json = format!(
                r#"{{
                "entryPrice": "1000.00000000",
                "symbol": "BTC-240329-40000-{}",
                "side": "LONG",
                "quantity": "1.00000000",
                "reducibleQty": "1.00000000",
                "markValue": "1000.00000000",
                "ror": "0.00000000",
                "unrealizedPNL": "0.00000000",
                "markPrice": "1000.00000000",
                "strikePrice": "40000.00000000",
                "positionCost": "1000.00000000",
                "expiryDate": 1711699200000,
                "priceScale": 8,
                "quantityScale": 4,
                "optionSide": "{}",
                "quoteAsset": "USDT"
            }}"#,
                side_str.chars().next().unwrap(),
                side_str
            );

            let position: Position = serde_json::from_str(&json).unwrap();
            assert_eq!(position.option_side, expected_side);
        }
    }

    #[test]
    fn test_position_all_position_sides_deserialization() {
        // Test both LONG and SHORT position sides
        let position_sides = vec![
            ("LONG", OptionsPositionSide::Long),
            ("SHORT", OptionsPositionSide::Short),
        ];

        for (side_str, expected_side) in position_sides {
            let json = format!(
                r#"{{
                "entryPrice": "1000.00000000",
                "symbol": "BTC-240329-40000-C",
                "side": "{}",
                "quantity": "1.00000000",
                "reducibleQty": "1.00000000",
                "markValue": "1000.00000000",
                "ror": "0.00000000",
                "unrealizedPNL": "0.00000000",
                "markPrice": "1000.00000000",
                "strikePrice": "40000.00000000",
                "positionCost": "1000.00000000",
                "expiryDate": 1711699200000,
                "priceScale": 8,
                "quantityScale": 4,
                "optionSide": "CALL",
                "quoteAsset": "USDT"
            }}"#,
                side_str
            );

            let position: Position = serde_json::from_str(&json).unwrap();
            assert_eq!(position.side, expected_side);
        }
    }

    #[test]
    fn test_position_realistic_portfolio_scenario() {
        // Test a realistic portfolio scenario with multiple positions
        let btc_call_long = r#"{
            "entryPrice": "5000.00000000",
            "symbol": "BTC-240329-40000-C",
            "side": "LONG",
            "quantity": "2.00000000",
            "reducibleQty": "2.00000000",
            "markValue": "12000.00000000",
            "ror": "0.20000000",
            "unrealizedPNL": "2000.00000000",
            "markPrice": "6000.00000000",
            "strikePrice": "40000.00000000",
            "positionCost": "10000.00000000",
            "expiryDate": 1711699200000,
            "priceScale": 8,
            "quantityScale": 4,
            "optionSide": "CALL",
            "quoteAsset": "USDT"
        }"#;

        let eth_put_short = r#"{
            "entryPrice": "800.00000000",
            "symbol": "ETH-240329-3000-P",
            "side": "SHORT",
            "quantity": "-3.00000000",
            "reducibleQty": "2.50000000",
            "markValue": "2100.00000000",
            "ror": "0.12500000",
            "unrealizedPNL": "300.00000000",
            "markPrice": "700.00000000",
            "strikePrice": "3000.00000000",
            "positionCost": "2400.00000000",
            "expiryDate": 1711699200000,
            "priceScale": 8,
            "quantityScale": 4,
            "optionSide": "PUT",
            "quoteAsset": "USDT"
        }"#;

        let btc_call_long_pos: Position = serde_json::from_str(btc_call_long).unwrap();
        let eth_put_short_pos: Position = serde_json::from_str(eth_put_short).unwrap();

        // Verify BTC long call position
        assert_eq!(btc_call_long_pos.symbol, "BTC-240329-40000-C");
        assert_eq!(btc_call_long_pos.side, OptionsPositionSide::Long);
        assert_eq!(btc_call_long_pos.option_side, OptionsContractType::Call);
        assert_eq!(btc_call_long_pos.quantity, dec!(2.00000000));
        assert!(btc_call_long_pos.quantity > dec!(0)); // Long position has positive quantity
        assert!(btc_call_long_pos.unrealized_pnl > dec!(0)); // Profitable position
        assert!(btc_call_long_pos.ror > dec!(0)); // Positive return

        // Verify ETH short put position
        assert_eq!(eth_put_short_pos.symbol, "ETH-240329-3000-P");
        assert_eq!(eth_put_short_pos.side, OptionsPositionSide::Short);
        assert_eq!(eth_put_short_pos.option_side, OptionsContractType::Put);
        assert_eq!(eth_put_short_pos.quantity, dec!(-3.00000000));
        assert!(eth_put_short_pos.quantity < dec!(0)); // Short position has negative quantity
        assert!(eth_put_short_pos.unrealized_pnl > dec!(0)); // Profitable position
        assert!(eth_put_short_pos.ror > dec!(0)); // Positive return

        // Verify both positions have same expiry
        assert_eq!(btc_call_long_pos.expiry_date, eth_put_short_pos.expiry_date);
    }

    #[test]
    fn test_position_various_expiry_dates() {
        // Test positions with various expiry dates
        let expiry_dates = vec![
            1711699200000u64, // March 29, 2024
            1719590400000u64, // June 28, 2024
            1735084800000u64, // December 25, 2024
            1640995200000u64, // January 1, 2022
            0u64,             // Epoch time
        ];

        for expiry_date in expiry_dates {
            let json = format!(
                r#"{{
                "entryPrice": "1000.00000000",
                "symbol": "BTC-240329-40000-C",
                "side": "LONG",
                "quantity": "1.00000000",
                "reducibleQty": "1.00000000",
                "markValue": "1000.00000000",
                "ror": "0.00000000",
                "unrealizedPNL": "0.00000000",
                "markPrice": "1000.00000000",
                "strikePrice": "40000.00000000",
                "positionCost": "1000.00000000",
                "expiryDate": {},
                "priceScale": 8,
                "quantityScale": 4,
                "optionSide": "CALL",
                "quoteAsset": "USDT"
            }}"#,
                expiry_date
            );

            let position: Position = serde_json::from_str(&json).unwrap();
            assert_eq!(position.expiry_date, expiry_date);
        }
    }

    #[test]
    fn test_position_real_world_trading_scenarios() {
        // Test real-world trading scenarios with realistic values

        // Scenario 1: Deep ITM Call (In-The-Money)
        let deep_itm_call = r#"{
            "entryPrice": "10000.00000000",
            "symbol": "BTC-240329-30000-C",
            "side": "LONG",
            "quantity": "0.50000000",
            "reducibleQty": "0.50000000",
            "markValue": "7500.00000000",
            "ror": "0.50000000",
            "unrealizedPNL": "2500.00000000",
            "markPrice": "15000.00000000",
            "strikePrice": "30000.00000000",
            "positionCost": "5000.00000000",
            "expiryDate": 1711699200000,
            "priceScale": 8,
            "quantityScale": 4,
            "optionSide": "CALL",
            "quoteAsset": "USDT"
        }"#;

        // Scenario 2: OTM Put (Out-of-The-Money) nearing expiry
        let otm_put = r#"{
            "entryPrice": "50.00000000",
            "symbol": "ETH-240329-2000-P",
            "side": "LONG",
            "quantity": "10.00000000",
            "reducibleQty": "10.00000000",
            "markValue": "100.00000000",
            "ror": "-0.80000000",
            "unrealizedPNL": "-400.00000000",
            "markPrice": "10.00000000",
            "strikePrice": "2000.00000000",
            "positionCost": "500.00000000",
            "expiryDate": 1711699200000,
            "priceScale": 8,
            "quantityScale": 4,
            "optionSide": "PUT",
            "quoteAsset": "USDT"
        }"#;

        let deep_itm_position: Position = serde_json::from_str(deep_itm_call).unwrap();
        let otm_put_position: Position = serde_json::from_str(otm_put).unwrap();

        // Verify deep ITM call characteristics
        assert_eq!(deep_itm_position.option_side, OptionsContractType::Call);
        assert!(deep_itm_position.mark_price > deep_itm_position.entry_price); // Appreciated
        assert!(deep_itm_position.unrealized_pnl > dec!(0)); // Profitable
        assert!(deep_itm_position.ror > dec!(0)); // Positive return

        // Verify OTM put characteristics
        assert_eq!(otm_put_position.option_side, OptionsContractType::Put);
        assert!(otm_put_position.mark_price < otm_put_position.entry_price); // Depreciated
        assert!(otm_put_position.unrealized_pnl < dec!(0)); // Losing money
        assert!(otm_put_position.ror < dec!(0)); // Negative return
    }

    #[test]
    fn test_position_edge_case_reducible_quantity() {
        // Test scenarios where reducible quantity differs from total quantity
        let json = r#"{
            "entryPrice": "1000.00000000",
            "symbol": "BTC-240329-40000-C",
            "side": "LONG",
            "quantity": "5.00000000",
            "reducibleQty": "3.50000000",
            "markValue": "5000.00000000",
            "ror": "0.00000000",
            "unrealizedPNL": "0.00000000",
            "markPrice": "1000.00000000",
            "strikePrice": "40000.00000000",
            "positionCost": "5000.00000000",
            "expiryDate": 1711699200000,
            "priceScale": 8,
            "quantityScale": 4,
            "optionSide": "CALL",
            "quoteAsset": "USDT"
        }"#;

        let position: Position = serde_json::from_str(json).unwrap();
        assert_eq!(position.quantity, dec!(5.00000000));
        assert_eq!(position.reducible_qty, dec!(3.50000000));
        assert!(position.reducible_qty < position.quantity); // Reducible is less than total
    }

    #[test]
    fn test_position_consistent_calculations() {
        // Test that position calculations are consistent with the data
        let json = r#"{
            "entryPrice": "1000.00000000",
            "symbol": "BTC-240329-40000-C",
            "side": "LONG",
            "quantity": "2.00000000",
            "reducibleQty": "2.00000000",
            "markValue": "2400.00000000",
            "ror": "0.20000000",
            "unrealizedPNL": "400.00000000",
            "markPrice": "1200.00000000",
            "strikePrice": "40000.00000000",
            "positionCost": "2000.00000000",
            "expiryDate": 1711699200000,
            "priceScale": 8,
            "quantityScale": 4,
            "optionSide": "CALL",
            "quoteAsset": "USDT"
        }"#;

        let position: Position = serde_json::from_str(json).unwrap();

        // Verify consistency: mark_value should equal mark_price * quantity
        let expected_mark_value = position.mark_price * position.quantity;
        assert_eq!(position.mark_value, expected_mark_value);

        // Verify consistency: unrealized_pnl should equal mark_value - position_cost
        let expected_unrealized_pnl = position.mark_value - position.position_cost;
        assert_eq!(position.unrealized_pnl, expected_unrealized_pnl);

        // Verify consistency: ror should equal unrealized_pnl / position_cost
        let expected_ror = position.unrealized_pnl / position.position_cost;
        assert_eq!(position.ror, expected_ror);
    }
}
