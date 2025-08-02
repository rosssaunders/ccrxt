use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{RestResult, SelfTradePreventionMode};

const GET_MY_PREVENTED_MATCHES_ENDPOINT: &str = "/api/v3/myPreventedMatches";

/// Request parameters for getting prevented matches
#[derive(Debug, Clone, Serialize)]
pub struct MyPreventedMatchesRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Prevented match ID
    #[serde(rename = "preventedMatchId", skip_serializing_if = "Option::is_none")]
    pub prevented_match_id: Option<u64>,

    /// Order ID
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// From prevented match ID
    #[serde(
        rename = "fromPreventedMatchId",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_prevented_match_id: Option<u64>,

    /// Default 500; max 1000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Prevented match information
#[derive(Debug, Clone, Deserialize)]
pub struct MyPreventedMatch {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Prevented match ID
    #[serde(rename = "preventedMatchId")]
    pub prevented_match_id: u64,

    /// Taker order ID
    #[serde(rename = "takerOrderId")]
    pub taker_order_id: u64,

    /// Maker symbol
    #[serde(rename = "makerSymbol")]
    pub maker_symbol: String,

    /// Maker order ID
    #[serde(rename = "makerOrderId")]
    pub maker_order_id: u64,

    /// Trade group ID
    #[serde(rename = "tradeGroupId")]
    pub trade_group_id: u64,

    /// Self-trade prevention mode
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,

    /// Match price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Maker prevented quantity
    #[serde(rename = "makerPreventedQuantity")]
    pub maker_prevented_quantity: Decimal,

    /// Taker prevented quantity
    #[serde(rename = "takerPreventedQuantity")]
    pub taker_prevented_quantity: Decimal,

    /// Transaction time
    #[serde(rename = "transactTime")]
    pub transact_time: u64,
}

impl RestClient {
    /// Display orders that were expired due to STP
    ///
    /// Display orders that were expired due to STP (Self-Trade Prevention).
    ///
    /// [docs]: (https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#query-prevented-matches--user_data)
    /// Method: GET /api/v3/myPreventedMatches
    /// Weight: 20
    /// Security: USER_DATA
    pub async fn get_my_prevented_matches(
        &self,
        params: MyPreventedMatchesRequest,
    ) -> RestResult<Vec<MyPreventedMatch>> {
        self.send_get_signed_request(
            GET_MY_PREVENTED_MATCHES_ENDPOINT,
            params,
            20,
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
    fn test_my_prevented_matches_request_minimal_serialization() {
        let request = MyPreventedMatchesRequest {
            symbol: "BTCUSDT".to_string(),
            prevented_match_id: None,
            order_id: None,
            from_prevented_match_id: None,
            limit: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert!(json.get("preventedMatchId").is_none());
        assert!(json.get("orderId").is_none());
        assert!(json.get("fromPreventedMatchId").is_none());
        assert!(json.get("limit").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_my_prevented_matches_request_with_prevented_match_id_serialization() {
        let request = MyPreventedMatchesRequest {
            symbol: "ETHUSDT".to_string(),
            prevented_match_id: Some(123456),
            order_id: None,
            from_prevented_match_id: None,
            limit: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDT");
        assert_eq!(json["preventedMatchId"], 123456);
        assert!(json.get("orderId").is_none());
        assert!(json.get("fromPreventedMatchId").is_none());
        assert!(json.get("limit").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_my_prevented_matches_request_with_order_id_serialization() {
        let request = MyPreventedMatchesRequest {
            symbol: "BNBUSDT".to_string(),
            prevented_match_id: None,
            order_id: Some(789012),
            from_prevented_match_id: None,
            limit: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BNBUSDT");
        assert!(json.get("preventedMatchId").is_none());
        assert_eq!(json["orderId"], 789012);
        assert!(json.get("fromPreventedMatchId").is_none());
        assert!(json.get("limit").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_my_prevented_matches_request_with_from_prevented_match_id_serialization() {
        let request = MyPreventedMatchesRequest {
            symbol: "SOLUSDT".to_string(),
            prevented_match_id: None,
            order_id: None,
            from_prevented_match_id: Some(345678),
            limit: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "SOLUSDT");
        assert!(json.get("preventedMatchId").is_none());
        assert!(json.get("orderId").is_none());
        assert_eq!(json["fromPreventedMatchId"], 345678);
        assert!(json.get("limit").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_my_prevented_matches_request_with_limit_serialization() {
        let request = MyPreventedMatchesRequest {
            symbol: "ADAUSDT".to_string(),
            prevented_match_id: None,
            order_id: None,
            from_prevented_match_id: None,
            limit: Some(100),
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ADAUSDT");
        assert!(json.get("preventedMatchId").is_none());
        assert!(json.get("orderId").is_none());
        assert!(json.get("fromPreventedMatchId").is_none());
        assert_eq!(json["limit"], 100);
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_my_prevented_matches_request_with_recv_window_serialization() {
        let request = MyPreventedMatchesRequest {
            symbol: "DOTUSDT".to_string(),
            prevented_match_id: None,
            order_id: None,
            from_prevented_match_id: None,
            limit: None,
            recv_window: Some(5000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "DOTUSDT");
        assert!(json.get("preventedMatchId").is_none());
        assert!(json.get("orderId").is_none());
        assert!(json.get("fromPreventedMatchId").is_none());
        assert!(json.get("limit").is_none());
        assert_eq!(json["recvWindow"], 5000);
    }

    #[test]
    fn test_my_prevented_matches_request_with_all_fields_serialization() {
        let request = MyPreventedMatchesRequest {
            symbol: "BTCUSDT".to_string(),
            prevented_match_id: Some(111111),
            order_id: Some(222222),
            from_prevented_match_id: Some(333333),
            limit: Some(500),
            recv_window: Some(10000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["preventedMatchId"], 111111);
        assert_eq!(json["orderId"], 222222);
        assert_eq!(json["fromPreventedMatchId"], 333333);
        assert_eq!(json["limit"], 500);
        assert_eq!(json["recvWindow"], 10000);
    }

    #[test]
    fn test_my_prevented_matches_request_limit_max_value_serialization() {
        let request = MyPreventedMatchesRequest {
            symbol: "ETHUSDT".to_string(),
            prevented_match_id: None,
            order_id: None,
            from_prevented_match_id: None,
            limit: Some(1000), // Maximum allowed
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDT");
        assert_eq!(json["limit"], 1000);
    }

    #[test]
    fn test_my_prevented_match_basic_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "preventedMatchId": 123456,
            "takerOrderId": 789012,
            "makerSymbol": "BTCUSDT",
            "makerOrderId": 345678,
            "tradeGroupId": 901234,
            "selfTradePreventionMode": "EXPIRE_TAKER",
            "price": "50000.00000000",
            "makerPreventedQuantity": "1.00000000",
            "takerPreventedQuantity": "1.00000000",
            "transactTime": 1684804350000
        }"#;

        let response: MyPreventedMatch = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.prevented_match_id, 123456);
        assert_eq!(response.taker_order_id, 789012);
        assert_eq!(response.maker_symbol, "BTCUSDT");
        assert_eq!(response.maker_order_id, 345678);
        assert_eq!(response.trade_group_id, 901234);
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireTaker
        );
        assert_eq!(response.price, dec!(50000.00000000));
        assert_eq!(response.maker_prevented_quantity, dec!(1.00000000));
        assert_eq!(response.taker_prevented_quantity, dec!(1.00000000));
        assert_eq!(response.transact_time, 1684804350000);
    }

    #[test]
    fn test_my_prevented_match_expire_maker_deserialization() {
        let json = r#"{
            "symbol": "ETHUSDT",
            "preventedMatchId": 555666,
            "takerOrderId": 777888,
            "makerSymbol": "ETHUSDT",
            "makerOrderId": 999000,
            "tradeGroupId": 111222,
            "selfTradePreventionMode": "EXPIRE_MAKER",
            "price": "3000.50000000",
            "makerPreventedQuantity": "2.50000000",
            "takerPreventedQuantity": "2.50000000",
            "transactTime": 1684804360000
        }"#;

        let response: MyPreventedMatch = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "ETHUSDT");
        assert_eq!(response.prevented_match_id, 555666);
        assert_eq!(response.taker_order_id, 777888);
        assert_eq!(response.maker_symbol, "ETHUSDT");
        assert_eq!(response.maker_order_id, 999000);
        assert_eq!(response.trade_group_id, 111222);
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireMaker
        );
        assert_eq!(response.price, dec!(3000.50000000));
        assert_eq!(response.maker_prevented_quantity, dec!(2.50000000));
        assert_eq!(response.taker_prevented_quantity, dec!(2.50000000));
        assert_eq!(response.transact_time, 1684804360000);
    }

    #[test]
    fn test_my_prevented_match_expire_both_deserialization() {
        let json = r#"{
            "symbol": "BNBUSDT",
            "preventedMatchId": 333444,
            "takerOrderId": 555777,
            "makerSymbol": "BNBUSDT",
            "makerOrderId": 888999,
            "tradeGroupId": 666555,
            "selfTradePreventionMode": "EXPIRE_BOTH",
            "price": "300.25000000",
            "makerPreventedQuantity": "10.00000000",
            "takerPreventedQuantity": "10.00000000",
            "transactTime": 1684804370000
        }"#;

        let response: MyPreventedMatch = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BNBUSDT");
        assert_eq!(response.prevented_match_id, 333444);
        assert_eq!(response.taker_order_id, 555777);
        assert_eq!(response.maker_symbol, "BNBUSDT");
        assert_eq!(response.maker_order_id, 888999);
        assert_eq!(response.trade_group_id, 666555);
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireBoth
        );
        assert_eq!(response.price, dec!(300.25000000));
        assert_eq!(response.maker_prevented_quantity, dec!(10.00000000));
        assert_eq!(response.taker_prevented_quantity, dec!(10.00000000));
        assert_eq!(response.transact_time, 1684804370000);
    }

    #[test]
    fn test_my_prevented_match_none_stp_mode_deserialization() {
        let json = r#"{
            "symbol": "SOLUSDT",
            "preventedMatchId": 987654,
            "takerOrderId": 123123,
            "makerSymbol": "SOLUSDT",
            "makerOrderId": 456456,
            "tradeGroupId": 789789,
            "selfTradePreventionMode": "NONE",
            "price": "100.12345678",
            "makerPreventedQuantity": "5.12345678",
            "takerPreventedQuantity": "5.12345678",
            "transactTime": 1684804380000
        }"#;

        let response: MyPreventedMatch = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "SOLUSDT");
        assert_eq!(response.prevented_match_id, 987654);
        assert_eq!(response.taker_order_id, 123123);
        assert_eq!(response.maker_symbol, "SOLUSDT");
        assert_eq!(response.maker_order_id, 456456);
        assert_eq!(response.trade_group_id, 789789);
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::None
        );
        assert_eq!(response.price, dec!(100.12345678));
        assert_eq!(response.maker_prevented_quantity, dec!(5.12345678));
        assert_eq!(response.taker_prevented_quantity, dec!(5.12345678));
        assert_eq!(response.transact_time, 1684804380000);
    }

    #[test]
    fn test_my_prevented_match_different_symbol_pair_deserialization() {
        let json = r#"{
            "symbol": "ADAUSDT",
            "preventedMatchId": 111000,
            "takerOrderId": 222111,
            "makerSymbol": "ADAUSDT",
            "makerOrderId": 333222,
            "tradeGroupId": 444333,
            "selfTradePreventionMode": "EXPIRE_TAKER",
            "price": "0.50000000",
            "makerPreventedQuantity": "1000.00000000",
            "takerPreventedQuantity": "1000.00000000",
            "transactTime": 1684804390000
        }"#;

        let response: MyPreventedMatch = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "ADAUSDT");
        assert_eq!(response.prevented_match_id, 111000);
        assert_eq!(response.taker_order_id, 222111);
        assert_eq!(response.maker_symbol, "ADAUSDT");
        assert_eq!(response.maker_order_id, 333222);
        assert_eq!(response.trade_group_id, 444333);
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireTaker
        );
        assert_eq!(response.price, dec!(0.50000000));
        assert_eq!(response.maker_prevented_quantity, dec!(1000.00000000));
        assert_eq!(response.taker_prevented_quantity, dec!(1000.00000000));
        assert_eq!(response.transact_time, 1684804390000);
    }

    #[test]
    fn test_my_prevented_match_precision_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "preventedMatchId": 999999,
            "takerOrderId": 888888,
            "makerSymbol": "BTCUSDT",
            "makerOrderId": 777777,
            "tradeGroupId": 666666,
            "selfTradePreventionMode": "EXPIRE_BOTH",
            "price": "49999.99999999",
            "makerPreventedQuantity": "0.12345678",
            "takerPreventedQuantity": "0.87654321",
            "transactTime": 1684804400000
        }"#;

        let response: MyPreventedMatch = serde_json::from_str(json).unwrap();
        assert_eq!(response.price.to_string(), "49999.99999999");
        assert_eq!(response.maker_prevented_quantity.to_string(), "0.12345678");
        assert_eq!(response.taker_prevented_quantity.to_string(), "0.87654321");
    }

    #[test]
    fn test_my_prevented_match_all_stp_modes_deserialization() {
        let stp_modes = vec![
            ("NONE", SelfTradePreventionMode::None),
            ("EXPIRE_TAKER", SelfTradePreventionMode::ExpireTaker),
            ("EXPIRE_MAKER", SelfTradePreventionMode::ExpireMaker),
            ("EXPIRE_BOTH", SelfTradePreventionMode::ExpireBoth),
        ];

        for (stp_str, expected_stp) in stp_modes {
            let json = format!(
                r#"{{
                    "symbol": "BTCUSDT",
                    "preventedMatchId": 123456,
                    "takerOrderId": 789012,
                    "makerSymbol": "BTCUSDT",
                    "makerOrderId": 345678,
                    "tradeGroupId": 901234,
                    "selfTradePreventionMode": "{}",
                    "price": "50000.00000000",
                    "makerPreventedQuantity": "1.00000000",
                    "takerPreventedQuantity": "1.00000000",
                    "transactTime": 1684804350000
                }}"#,
                stp_str
            );

            let response: MyPreventedMatch = serde_json::from_str(&json).unwrap();
            assert_eq!(response.self_trade_prevention_mode, expected_stp);
        }
    }

    #[test]
    fn test_my_prevented_match_zero_quantities_deserialization() {
        let json = r#"{
            "symbol": "ETHUSDT",
            "preventedMatchId": 555555,
            "takerOrderId": 444444,
            "makerSymbol": "ETHUSDT",
            "makerOrderId": 333333,
            "tradeGroupId": 222222,
            "selfTradePreventionMode": "EXPIRE_TAKER",
            "price": "3000.00000000",
            "makerPreventedQuantity": "0.00000000",
            "takerPreventedQuantity": "0.00000000",
            "transactTime": 1684804410000
        }"#;

        let response: MyPreventedMatch = serde_json::from_str(json).unwrap();
        assert_eq!(response.maker_prevented_quantity, dec!(0.00000000));
        assert_eq!(response.taker_prevented_quantity, dec!(0.00000000));
        assert_eq!(response.price, dec!(3000.00000000));
    }

    #[test]
    fn test_my_prevented_match_large_numbers_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "preventedMatchId": 18446744073709551615,
            "takerOrderId": 9223372036854775807,
            "makerSymbol": "BTCUSDT",
            "makerOrderId": 9223372036854775806,
            "tradeGroupId": 18446744073709551614,
            "selfTradePreventionMode": "EXPIRE_BOTH",
            "price": "999999.99999999",
            "makerPreventedQuantity": "999999.99999999",
            "takerPreventedQuantity": "999999.99999999",
            "transactTime": 9223372036854775807
        }"#;

        let response: MyPreventedMatch = serde_json::from_str(json).unwrap();
        assert_eq!(response.prevented_match_id, 18446744073709551615);
        assert_eq!(response.taker_order_id, 9223372036854775807);
        assert_eq!(response.maker_order_id, 9223372036854775806);
        assert_eq!(response.trade_group_id, 18446744073709551614);
        assert_eq!(response.transact_time, 9223372036854775807);
        assert_eq!(response.price.to_string(), "999999.99999999");
        assert_eq!(
            response.maker_prevented_quantity.to_string(),
            "999999.99999999"
        );
        assert_eq!(
            response.taker_prevented_quantity.to_string(),
            "999999.99999999"
        );
    }

    #[test]
    fn test_my_prevented_match_cross_symbol_deserialization() {
        // Test when maker and taker have different symbols (edge case)
        let json = r#"{
            "symbol": "BTCUSDT",
            "preventedMatchId": 123456,
            "takerOrderId": 789012,
            "makerSymbol": "ETHUSDT",
            "makerOrderId": 345678,
            "tradeGroupId": 901234,
            "selfTradePreventionMode": "EXPIRE_TAKER",
            "price": "50000.00000000",
            "makerPreventedQuantity": "1.00000000",
            "takerPreventedQuantity": "1.00000000",
            "transactTime": 1684804350000
        }"#;

        let response: MyPreventedMatch = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.maker_symbol, "ETHUSDT");
        assert_ne!(response.symbol, response.maker_symbol);
    }

    #[test]
    fn test_my_prevented_match_different_prevented_quantities_deserialization() {
        // Test when maker and taker have different prevented quantities
        let json = r#"{
            "symbol": "SOLUSDT",
            "preventedMatchId": 654321,
            "takerOrderId": 987654,
            "makerSymbol": "SOLUSDT",
            "makerOrderId": 321987,
            "tradeGroupId": 147852,
            "selfTradePreventionMode": "EXPIRE_BOTH",
            "price": "100.00000000",
            "makerPreventedQuantity": "5.00000000",
            "takerPreventedQuantity": "3.00000000",
            "transactTime": 1684804420000
        }"#;

        let response: MyPreventedMatch = serde_json::from_str(json).unwrap();
        assert_eq!(response.maker_prevented_quantity, dec!(5.00000000));
        assert_eq!(response.taker_prevented_quantity, dec!(3.00000000));
        assert_ne!(
            response.maker_prevented_quantity,
            response.taker_prevented_quantity
        );
    }

    #[test]
    fn test_my_prevented_match_vector_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "preventedMatchId": 123456,
                "takerOrderId": 789012,
                "makerSymbol": "BTCUSDT",
                "makerOrderId": 345678,
                "tradeGroupId": 901234,
                "selfTradePreventionMode": "EXPIRE_TAKER",
                "price": "50000.00000000",
                "makerPreventedQuantity": "1.00000000",
                "takerPreventedQuantity": "1.00000000",
                "transactTime": 1684804350000
            },
            {
                "symbol": "ETHUSDT",
                "preventedMatchId": 555666,
                "takerOrderId": 777888,
                "makerSymbol": "ETHUSDT",
                "makerOrderId": 999000,
                "tradeGroupId": 111222,
                "selfTradePreventionMode": "EXPIRE_MAKER",
                "price": "3000.50000000",
                "makerPreventedQuantity": "2.50000000",
                "takerPreventedQuantity": "2.50000000",
                "transactTime": 1684804360000
            }
        ]"#;

        let response: Vec<MyPreventedMatch> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);

        // First match
        assert_eq!(response[0].symbol, "BTCUSDT");
        assert_eq!(response[0].prevented_match_id, 123456);
        assert_eq!(
            response[0].self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireTaker
        );

        // Second match
        assert_eq!(response[1].symbol, "ETHUSDT");
        assert_eq!(response[1].prevented_match_id, 555666);
        assert_eq!(
            response[1].self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireMaker
        );
    }

    #[test]
    fn test_my_prevented_match_empty_vector_deserialization() {
        let json = r#"[]"#;
        let response: Vec<MyPreventedMatch> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 0);
    }
}
