use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{
    OrderResponseType, OrderSide, OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
};

const TEST_ORDER_ENDPOINT: &str = "/api/v3/order/test";

/// Request parameters for testing a new order
#[derive(Debug, Clone, Serialize)]
pub struct TestNewOrderRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order side (BUY or SELL)
    #[serde(rename = "side")]
    pub side: OrderSide,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Time in force
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Order quantity
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,

    /// Quote order quantity
    #[serde(rename = "quoteOrderQty", skip_serializing_if = "Option::is_none")]
    pub quote_order_qty: Option<Decimal>,

    /// Order price
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,

    /// Client order ID
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    /// Strategy ID
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<u32>,

    /// Strategy type
    #[serde(rename = "strategyType", skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<u32>,

    /// Stop price
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<Decimal>,

    /// Trailing delta
    #[serde(rename = "trailingDelta", skip_serializing_if = "Option::is_none")]
    pub trailing_delta: Option<u32>,

    /// Iceberg quantity
    #[serde(rename = "icebergQty", skip_serializing_if = "Option::is_none")]
    pub iceberg_qty: Option<Decimal>,

    /// Response type
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<OrderResponseType>,

    /// Self-trade prevention mode
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,

    /// Compute commission rates
    #[serde(
        rename = "computeCommissionRates",
        skip_serializing_if = "Option::is_none"
    )]
    pub compute_commission_rates: Option<bool>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Test order response (empty object)
#[derive(Debug, Clone, Deserialize)]
pub struct TestOrderResponse {}

/// Commission rates response (when computeCommissionRates=true)
#[derive(Debug, Clone, Deserialize)]
pub struct CommissionRatesResponse {
    /// Standard commission rates
    #[serde(rename = "standardCommission")]
    pub standard_commission: CommissionRates,

    /// Tax commission rates
    #[serde(rename = "taxCommission")]
    pub tax_commission: CommissionRates,

    /// Discount information
    #[serde(rename = "discount")]
    pub discount: Discount,
}

/// Commission rates structure
#[derive(Debug, Clone, Deserialize)]
pub struct CommissionRates {
    /// Maker commission rate
    #[serde(rename = "maker")]
    pub maker: Decimal,

    /// Taker commission rate
    #[serde(rename = "taker")]
    pub taker: Decimal,

    /// Buyer commission rate
    #[serde(rename = "buyer")]
    pub buyer: Decimal,

    /// Seller commission rate
    #[serde(rename = "seller")]
    pub seller: Decimal,
}

/// Discount information
#[derive(Debug, Clone, Deserialize)]
pub struct Discount {
    /// Enable buy back
    #[serde(rename = "enabledForAccount")]
    pub enabled_for_account: bool,

    /// Enable buy back for symbol
    #[serde(rename = "enabledForSymbol")]
    pub enabled_for_symbol: bool,

    /// Discount asset
    #[serde(rename = "discountAsset")]
    pub discount_asset: String,

    /// Discount rate
    #[serde(rename = "discount")]
    pub discount: Decimal,
}

impl RestClient {
    /// Test new order creation and signature/recvWindow
    ///
    /// Test new order creation and signature/recvWindow long.
    /// Creates and validates a new order but does not send it into the matching engine.
    ///
    /// [docs]: (https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#test-new-order--trade)
    /// Method: POST /api/v3/order/test
    /// Weight: 1 (without computeCommissionRates), 20 (with computeCommissionRates)
    /// Security: TRADE
    pub async fn test_new_order(
        &self,
        params: TestNewOrderRequest,
    ) -> RestResult<serde_json::Value> {
        let weight = if params.compute_commission_rates.unwrap_or(false) {
            20
        } else {
            1
        };

        self.send_post_signed_request(TEST_ORDER_ENDPOINT, params, weight, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_test_new_order_request_minimal_serialization() {
        // Test minimal request with only required fields
        let request = TestNewOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: None,
            quantity: Some(dec!(0.001)),
            quote_order_qty: None,
            price: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            compute_commission_rates: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["side"], "BUY");
        assert_eq!(json["type"], "MARKET");
        assert_eq!(json["quantity"], "0.001");

        // Ensure optional fields are not serialized when None
        assert!(json.get("timeInForce").is_none());
        assert!(json.get("price").is_none());
        assert!(json.get("newClientOrderId").is_none());
        assert!(json.get("strategyId").is_none());
        assert!(json.get("strategyType").is_none());
        assert!(json.get("stopPrice").is_none());
        assert!(json.get("trailingDelta").is_none());
        assert!(json.get("icebergQty").is_none());
        assert!(json.get("newOrderRespType").is_none());
        assert!(json.get("selfTradePreventionMode").is_none());
        assert!(json.get("computeCommissionRates").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_test_new_order_request_limit_order_serialization() {
        // Test LIMIT order with all relevant fields
        let request = TestNewOrderRequest {
            symbol: "ETHUSDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some(dec!(0.5)),
            quote_order_qty: None,
            price: Some(dec!(3000.50)),
            new_client_order_id: Some("test-order-123".to_string()),
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: Some(OrderResponseType::Full),
            self_trade_prevention_mode: None,
            compute_commission_rates: None,
            recv_window: Some(5000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDT");
        assert_eq!(json["side"], "SELL");
        assert_eq!(json["type"], "LIMIT");
        assert_eq!(json["timeInForce"], "GTC");
        assert_eq!(json["quantity"], "0.5");
        assert_eq!(json["price"], "3000.50");
        assert_eq!(json["newClientOrderId"], "test-order-123");
        assert_eq!(json["newOrderRespType"], "FULL");
        assert_eq!(json["recvWindow"], 5000);
    }

    #[test]
    fn test_test_new_order_request_market_order_with_quote_qty() {
        // Test MARKET order using quote order quantity instead of quantity
        let request = TestNewOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: None,
            quantity: None,
            quote_order_qty: Some(dec!(1000)),
            price: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            compute_commission_rates: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["quoteOrderQty"], "1000");
        assert!(json.get("quantity").is_none());
    }

    #[test]
    fn test_test_new_order_request_stop_loss_limit_serialization() {
        // Test STOP_LOSS_LIMIT order with stop price
        let request = TestNewOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::StopLossLimit,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some(dec!(0.1)),
            quote_order_qty: None,
            price: Some(dec!(45000)),
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: Some(dec!(45500)),
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            compute_commission_rates: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["type"], "STOP_LOSS_LIMIT");
        assert_eq!(json["stopPrice"], "45500");
        assert_eq!(json["price"], "45000");
        assert_eq!(json["timeInForce"], "GTC");
    }

    #[test]
    fn test_test_new_order_request_take_profit_serialization() {
        // Test TAKE_PROFIT order
        let request = TestNewOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::TakeProfit,
            time_in_force: None,
            quantity: Some(dec!(0.05)),
            quote_order_qty: None,
            price: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: Some(dec!(55000)),
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            compute_commission_rates: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["type"], "TAKE_PROFIT");
        assert_eq!(json["stopPrice"], "55000");
    }

    #[test]
    fn test_test_new_order_request_iceberg_order_serialization() {
        // Test iceberg order functionality
        let request = TestNewOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some(dec!(10)),
            quote_order_qty: None,
            price: Some(dec!(50000)),
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: Some(dec!(1)),
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            compute_commission_rates: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["icebergQty"], "1");
        assert_eq!(json["quantity"], "10");
    }

    #[test]
    fn test_test_new_order_request_trailing_stop_serialization() {
        // Test trailing stop order
        let request = TestNewOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::StopLoss,
            time_in_force: None,
            quantity: Some(dec!(0.1)),
            quote_order_qty: None,
            price: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: Some(200),
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            compute_commission_rates: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["trailingDelta"], 200);
        assert_eq!(json["type"], "STOP_LOSS");
    }

    #[test]
    fn test_test_new_order_request_with_strategy_serialization() {
        // Test order with strategy fields
        let request = TestNewOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::IOC),
            quantity: Some(dec!(0.5)),
            quote_order_qty: None,
            price: Some(dec!(48000)),
            new_client_order_id: None,
            strategy_id: Some(12345),
            strategy_type: Some(1000000),
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: Some(SelfTradePreventionMode::ExpireTaker),
            compute_commission_rates: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["strategyId"], 12345);
        assert_eq!(json["strategyType"], 1000000);
        assert_eq!(json["selfTradePreventionMode"], "EXPIRE_TAKER");
        assert_eq!(json["timeInForce"], "IOC");
    }

    #[test]
    fn test_test_new_order_request_compute_commission_rates() {
        // Test order with compute commission rates enabled
        let request = TestNewOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: None,
            quantity: Some(dec!(0.1)),
            quote_order_qty: None,
            price: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            compute_commission_rates: Some(true),
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["computeCommissionRates"], true);
    }

    #[test]
    fn test_test_new_order_request_limit_maker_serialization() {
        // Test LIMIT_MAKER order type
        let request = TestNewOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::LimitMaker,
            time_in_force: None,
            quantity: Some(dec!(0.2)),
            quote_order_qty: None,
            price: Some(dec!(49000)),
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            compute_commission_rates: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["type"], "LIMIT_MAKER");
        assert_eq!(json["price"], "49000");
        // LIMIT_MAKER doesn't use timeInForce
        assert!(json.get("timeInForce").is_none());
    }

    #[test]
    fn test_test_new_order_request_all_order_types() {
        // Test serialization of all order types
        let order_types = vec![
            (OrderType::Limit, "LIMIT", true, true),
            (OrderType::Market, "MARKET", false, false),
            (OrderType::StopLoss, "STOP_LOSS", false, false),
            (OrderType::StopLossLimit, "STOP_LOSS_LIMIT", true, true),
            (OrderType::TakeProfit, "TAKE_PROFIT", false, false),
            (OrderType::TakeProfitLimit, "TAKE_PROFIT_LIMIT", true, true),
            (OrderType::LimitMaker, "LIMIT_MAKER", true, false),
        ];

        for (order_type, expected_str, needs_price, needs_tif) in order_types {
            let request = TestNewOrderRequest {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                order_type,
                time_in_force: if needs_tif {
                    Some(TimeInForce::GTC)
                } else {
                    None
                },
                quantity: Some(dec!(1)),
                quote_order_qty: None,
                price: if needs_price { Some(dec!(50000)) } else { None },
                new_client_order_id: None,
                strategy_id: None,
                strategy_type: None,
                stop_price: if order_type.to_string().contains("STOP")
                    || order_type.to_string().contains("TAKE_PROFIT")
                {
                    Some(dec!(49000))
                } else {
                    None
                },
                trailing_delta: None,
                iceberg_qty: None,
                new_order_resp_type: None,
                self_trade_prevention_mode: None,
                compute_commission_rates: None,
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["type"], expected_str);
        }
    }

    #[test]
    fn test_test_new_order_request_all_time_in_force() {
        // Test all TimeInForce values
        let tif_values = vec![
            (TimeInForce::GTC, "GTC"),
            (TimeInForce::IOC, "IOC"),
            (TimeInForce::FOK, "FOK"),
        ];

        for (tif, expected_str) in tif_values {
            let request = TestNewOrderRequest {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                order_type: OrderType::Limit,
                time_in_force: Some(tif),
                quantity: Some(dec!(1)),
                quote_order_qty: None,
                price: Some(dec!(50000)),
                new_client_order_id: None,
                strategy_id: None,
                strategy_type: None,
                stop_price: None,
                trailing_delta: None,
                iceberg_qty: None,
                new_order_resp_type: None,
                self_trade_prevention_mode: None,
                compute_commission_rates: None,
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["timeInForce"], expected_str);
        }
    }

    #[test]
    fn test_test_new_order_request_all_self_trade_prevention_modes() {
        // Test all SelfTradePreventionMode values
        let stp_modes = vec![
            (SelfTradePreventionMode::None, "NONE"),
            (SelfTradePreventionMode::ExpireTaker, "EXPIRE_TAKER"),
            (SelfTradePreventionMode::ExpireMaker, "EXPIRE_MAKER"),
            (SelfTradePreventionMode::ExpireBoth, "EXPIRE_BOTH"),
        ];

        for (stp_mode, expected_str) in stp_modes {
            let request = TestNewOrderRequest {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                order_type: OrderType::Limit,
                time_in_force: Some(TimeInForce::GTC),
                quantity: Some(dec!(1)),
                quote_order_qty: None,
                price: Some(dec!(50000)),
                new_client_order_id: None,
                strategy_id: None,
                strategy_type: None,
                stop_price: None,
                trailing_delta: None,
                iceberg_qty: None,
                new_order_resp_type: None,
                self_trade_prevention_mode: Some(stp_mode),
                compute_commission_rates: None,
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["selfTradePreventionMode"], expected_str);
        }
    }

    #[test]
    fn test_test_new_order_request_all_order_response_types() {
        // Test all OrderResponseType values
        let response_types = vec![
            (OrderResponseType::Ack, "ACK"),
            (OrderResponseType::Result, "RESULT"),
            (OrderResponseType::Full, "FULL"),
        ];

        for (resp_type, expected_str) in response_types {
            let request = TestNewOrderRequest {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                order_type: OrderType::Market,
                time_in_force: None,
                quantity: Some(dec!(1)),
                quote_order_qty: None,
                price: None,
                new_client_order_id: None,
                strategy_id: None,
                strategy_type: None,
                stop_price: None,
                trailing_delta: None,
                iceberg_qty: None,
                new_order_resp_type: Some(resp_type),
                self_trade_prevention_mode: None,
                compute_commission_rates: None,
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["newOrderRespType"], expected_str);
        }
    }

    #[test]
    fn test_test_new_order_request_both_sides() {
        // Test both BUY and SELL sides
        let sides = vec![(OrderSide::Buy, "BUY"), (OrderSide::Sell, "SELL")];

        for (side, expected_str) in sides {
            let request = TestNewOrderRequest {
                symbol: "BTCUSDT".to_string(),
                side,
                order_type: OrderType::Market,
                time_in_force: None,
                quantity: Some(dec!(1)),
                quote_order_qty: None,
                price: None,
                new_client_order_id: None,
                strategy_id: None,
                strategy_type: None,
                stop_price: None,
                trailing_delta: None,
                iceberg_qty: None,
                new_order_resp_type: None,
                self_trade_prevention_mode: None,
                compute_commission_rates: None,
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["side"], expected_str);
        }
    }

    #[test]
    fn test_test_new_order_request_decimal_precision() {
        // Test decimal precision handling
        let request = TestNewOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some(dec!(0.00000001)),
            quote_order_qty: None,
            price: Some(dec!(12345.67890123)),
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: Some(dec!(0.000001)),
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            compute_commission_rates: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["quantity"], "0.00000001");
        assert_eq!(json["price"], "12345.67890123");
        assert_eq!(json["icebergQty"], "0.000001");
    }

    #[test]
    fn test_test_new_order_request_large_values() {
        // Test large numeric values
        let request = TestNewOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some(dec!(999999999.99999999)),
            quote_order_qty: None,
            price: Some(dec!(999999999.99999999)),
            new_client_order_id: None,
            strategy_id: Some(4294967295),   // Max u32
            strategy_type: Some(4294967295), // Max u32
            stop_price: None,
            trailing_delta: Some(999999),
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            compute_commission_rates: None,
            recv_window: Some(60000), // Max recv window
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["quantity"], "999999999.99999999");
        assert_eq!(json["price"], "999999999.99999999");
        assert_eq!(json["strategyId"], 4294967295u32);
        assert_eq!(json["strategyType"], 4294967295u32);
        assert_eq!(json["trailingDelta"], 999999);
        assert_eq!(json["recvWindow"], 60000);
    }

    #[test]
    fn test_test_new_order_request_special_client_order_id() {
        // Test various client order ID formats
        let client_ids = vec![
            "simple-id",
            "id-with-numbers-123",
            "ID_WITH_UNDERSCORES",
            "id.with.dots",
            "very-long-client-order-id-that-tests-the-limits-of-what-binance-accepts",
        ];

        for client_id in client_ids {
            let request = TestNewOrderRequest {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                order_type: OrderType::Market,
                time_in_force: None,
                quantity: Some(dec!(1)),
                quote_order_qty: None,
                price: None,
                new_client_order_id: Some(client_id.to_string()),
                strategy_id: None,
                strategy_type: None,
                stop_price: None,
                trailing_delta: None,
                iceberg_qty: None,
                new_order_resp_type: None,
                self_trade_prevention_mode: None,
                compute_commission_rates: None,
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["newClientOrderId"], client_id);
        }
    }

    #[test]
    fn test_test_order_response_deserialization() {
        // Test empty response deserialization
        let json = "{}";
        let response: TestOrderResponse = serde_json::from_str(json).unwrap();
        // TestOrderResponse is an empty struct, so this just tests it can be deserialized
        let _ = response;
    }

    #[test]
    fn test_commission_rates_response_deserialization() {
        let json = r#"{
            "standardCommission": {
                "maker": "0.00100000",
                "taker": "0.00100000",
                "buyer": "0.00000000",
                "seller": "0.00000000"
            },
            "taxCommission": {
                "maker": "0.00050000",
                "taker": "0.00050000",
                "buyer": "0.00000000",
                "seller": "0.00000000"
            },
            "discount": {
                "enabledForAccount": true,
                "enabledForSymbol": true,
                "discountAsset": "BNB",
                "discount": "0.25000000"
            }
        }"#;

        let response: CommissionRatesResponse = serde_json::from_str(json).unwrap();

        // Test standard commission
        assert_eq!(response.standard_commission.maker.to_string(), "0.00100000");
        assert_eq!(response.standard_commission.taker.to_string(), "0.00100000");
        assert_eq!(response.standard_commission.buyer.to_string(), "0.00000000");
        assert_eq!(
            response.standard_commission.seller.to_string(),
            "0.00000000"
        );

        // Test tax commission
        assert_eq!(response.tax_commission.maker.to_string(), "0.00050000");
        assert_eq!(response.tax_commission.taker.to_string(), "0.00050000");
        assert_eq!(response.tax_commission.buyer.to_string(), "0.00000000");
        assert_eq!(response.tax_commission.seller.to_string(), "0.00000000");

        // Test discount
        assert!(response.discount.enabled_for_account);
        assert!(response.discount.enabled_for_symbol);
        assert_eq!(response.discount.discount_asset, "BNB");
        assert_eq!(response.discount.discount.to_string(), "0.25000000");
    }

    #[test]
    fn test_commission_rates_deserialization() {
        let json = r#"{
            "maker": "0.00100000",
            "taker": "0.00100000",
            "buyer": "0.00000000",
            "seller": "0.00000000"
        }"#;

        let rates: CommissionRates = serde_json::from_str(json).unwrap();
        assert_eq!(rates.maker.to_string(), "0.00100000");
        assert_eq!(rates.taker.to_string(), "0.00100000");
        assert_eq!(rates.buyer.to_string(), "0.00000000");
        assert_eq!(rates.seller.to_string(), "0.00000000");
    }

    #[test]
    fn test_discount_deserialization() {
        let json = r#"{
            "enabledForAccount": false,
            "enabledForSymbol": false,
            "discountAsset": "USDT",
            "discount": "0.00000000"
        }"#;

        let discount: Discount = serde_json::from_str(json).unwrap();
        assert!(!discount.enabled_for_account);
        assert!(!discount.enabled_for_symbol);
        assert_eq!(discount.discount_asset, "USDT");
        assert_eq!(discount.discount.to_string(), "0.00000000");
    }

    #[test]
    fn test_commission_rates_high_precision_deserialization() {
        // Test high precision decimal values
        let json = r#"{
            "maker": "0.00012345",
            "taker": "0.00098765",
            "buyer": "0.00000001",
            "seller": "0.99999999"
        }"#;

        let rates: CommissionRates = serde_json::from_str(json).unwrap();
        assert_eq!(rates.maker.to_string(), "0.00012345");
        assert_eq!(rates.taker.to_string(), "0.00098765");
        assert_eq!(rates.buyer.to_string(), "0.00000001");
        assert_eq!(rates.seller.to_string(), "0.99999999");
    }
}
