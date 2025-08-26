use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::spot::PrivateRestClient as RestClient;
use crate::binance::spot::RestResult;

const GET_ORDER_AMENDMENTS_ENDPOINT: &str = "/api/v3/order/amendments";

/// Request parameters for querying order amendments
#[derive(Debug, Clone, Serialize)]
pub struct OrderAmendmentsRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// From execution ID
    #[serde(rename = "fromExecutionId", skip_serializing_if = "Option::is_none")]
    pub from_execution_id: Option<u64>,

    /// Default 500; max 1000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Order amendment information
#[derive(Debug, Clone, Deserialize)]
pub struct OrderAmendment {
    /// Amendment ID
    #[serde(rename = "amendmentId")]
    pub amendment_id: u64,

    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Amendment time
    #[serde(rename = "time")]
    pub time: u64,

    /// Original quantity
    #[serde(rename = "origQty")]
    pub orig_qty: Decimal,

    /// Current quantity
    #[serde(rename = "qty")]
    pub qty: Decimal,

    /// Original price
    #[serde(rename = "origPrice")]
    pub orig_price: Decimal,

    /// Current price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Amendment status
    #[serde(rename = "status")]
    pub status: String,

    /// Execution ID
    #[serde(rename = "executionId")]
    pub execution_id: u64,
}

impl RestClient {
    /// Query all amendments of a single order
    ///
    /// Query all amendments of a single order.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#query-order-amendments--user_data)
    ///
    /// Method: GET /api/v3/order/amendments
    /// Weight: 4
    /// Security: USER_DATA
    pub async fn get_order_amendments(
        &self,
        params: OrderAmendmentsRequest,
    ) -> RestResult<Vec<OrderAmendment>> {
        self.send_get_signed_request(GET_ORDER_AMENDMENTS_ENDPOINT, params, 4, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_order_amendments_request_minimal_serialization() {
        let request = OrderAmendmentsRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: 12345,
            from_execution_id: None,
            limit: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["orderId"], 12345);
        assert!(json.get("fromExecutionId").is_none());
        assert!(json.get("limit").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_order_amendments_request_with_all_fields_serialization() {
        let request = OrderAmendmentsRequest {
            symbol: "ETHUSDT".to_string(),
            order_id: 67890,
            from_execution_id: Some(111111),
            limit: Some(100),
            recv_window: Some(5000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDT");
        assert_eq!(json["orderId"], 67890);
        assert_eq!(json["fromExecutionId"], 111111);
        assert_eq!(json["limit"], 100);
        assert_eq!(json["recvWindow"], 5000);
    }

    #[test]
    fn test_order_amendments_request_with_execution_id_serialization() {
        let request = OrderAmendmentsRequest {
            symbol: "BNBUSDT".to_string(),
            order_id: 98765,
            from_execution_id: Some(555555),
            limit: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BNBUSDT");
        assert_eq!(json["orderId"], 98765);
        assert_eq!(json["fromExecutionId"], 555555);
        assert!(json.get("limit").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_order_amendments_request_with_limit_serialization() {
        let request = OrderAmendmentsRequest {
            symbol: "ADAUSDT".to_string(),
            order_id: 24680,
            from_execution_id: None,
            limit: Some(500),
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ADAUSDT");
        assert_eq!(json["orderId"], 24680);
        assert!(json.get("fromExecutionId").is_none());
        assert_eq!(json["limit"], 500);
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_order_amendments_request_with_recv_window_serialization() {
        let request = OrderAmendmentsRequest {
            symbol: "SOLUSDT".to_string(),
            order_id: 13579,
            from_execution_id: None,
            limit: None,
            recv_window: Some(10000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "SOLUSDT");
        assert_eq!(json["orderId"], 13579);
        assert!(json.get("fromExecutionId").is_none());
        assert!(json.get("limit").is_none());
        assert_eq!(json["recvWindow"], 10000);
    }

    #[test]
    fn test_order_amendments_request_with_max_limit_serialization() {
        let request = OrderAmendmentsRequest {
            symbol: "DOTUSDT".to_string(),
            order_id: 77777,
            from_execution_id: Some(999999),
            limit: Some(1000),
            recv_window: Some(60000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "DOTUSDT");
        assert_eq!(json["orderId"], 77777);
        assert_eq!(json["fromExecutionId"], 999999);
        assert_eq!(json["limit"], 1000);
        assert_eq!(json["recvWindow"], 60000);
    }

    #[test]
    fn test_order_amendment_basic_deserialization() {
        let json = r#"{
            "amendmentId": 111111,
            "symbol": "BTCUSDT",
            "orderId": 12345,
            "clientOrderId": "myOrder123",
            "time": 1684804350000,
            "origQty": "1.00000000",
            "qty": "0.50000000",
            "origPrice": "50000.00000000",
            "price": "48000.00000000",
            "status": "SUCCESS",
            "executionId": 999999
        }"#;

        let amendment: OrderAmendment = serde_json::from_str(json).unwrap();
        assert_eq!(amendment.amendment_id, 111111);
        assert_eq!(amendment.symbol, "BTCUSDT");
        assert_eq!(amendment.order_id, 12345);
        assert_eq!(amendment.client_order_id, "myOrder123");
        assert_eq!(amendment.time, 1684804350000);
        assert_eq!(amendment.orig_qty, dec!(1.00000000));
        assert_eq!(amendment.qty, dec!(0.50000000));
        assert_eq!(amendment.orig_price, dec!(50000.00000000));
        assert_eq!(amendment.price, dec!(48000.00000000));
        assert_eq!(amendment.status, "SUCCESS");
        assert_eq!(amendment.execution_id, 999999);
    }

    #[test]
    fn test_order_amendment_with_price_only_change_deserialization() {
        let json = r#"{
            "amendmentId": 222222,
            "symbol": "ETHUSDT",
            "orderId": 67890,
            "clientOrderId": "ethOrder456",
            "time": 1684804360000,
            "origQty": "5.00000000",
            "qty": "5.00000000",
            "origPrice": "3000.00000000",
            "price": "2950.00000000",
            "status": "SUCCESS",
            "executionId": 888888
        }"#;

        let amendment: OrderAmendment = serde_json::from_str(json).unwrap();
        assert_eq!(amendment.amendment_id, 222222);
        assert_eq!(amendment.symbol, "ETHUSDT");
        assert_eq!(amendment.order_id, 67890);
        assert_eq!(amendment.client_order_id, "ethOrder456");
        assert_eq!(amendment.time, 1684804360000);
        assert_eq!(amendment.orig_qty, amendment.qty); // Quantity unchanged
        assert_eq!(amendment.orig_price, dec!(3000.00000000));
        assert_eq!(amendment.price, dec!(2950.00000000));
        assert_eq!(amendment.status, "SUCCESS");
        assert_eq!(amendment.execution_id, 888888);
    }

    #[test]
    fn test_order_amendment_with_quantity_only_change_deserialization() {
        let json = r#"{
            "amendmentId": 333333,
            "symbol": "BNBUSDT",
            "orderId": 98765,
            "clientOrderId": "bnbOrder789",
            "time": 1684804370000,
            "origQty": "10.00000000",
            "qty": "7.50000000",
            "origPrice": "400.00000000",
            "price": "400.00000000",
            "status": "SUCCESS",
            "executionId": 777777
        }"#;

        let amendment: OrderAmendment = serde_json::from_str(json).unwrap();
        assert_eq!(amendment.amendment_id, 333333);
        assert_eq!(amendment.symbol, "BNBUSDT");
        assert_eq!(amendment.order_id, 98765);
        assert_eq!(amendment.client_order_id, "bnbOrder789");
        assert_eq!(amendment.time, 1684804370000);
        assert_eq!(amendment.orig_qty, dec!(10.00000000));
        assert_eq!(amendment.qty, dec!(7.50000000));
        assert_eq!(amendment.orig_price, amendment.price); // Price unchanged
        assert_eq!(amendment.status, "SUCCESS");
        assert_eq!(amendment.execution_id, 777777);
    }

    #[test]
    fn test_order_amendment_with_both_price_and_quantity_change_deserialization() {
        let json = r#"{
            "amendmentId": 444444,
            "symbol": "ADAUSDT",
            "orderId": 24680,
            "clientOrderId": "adaOrder111",
            "time": 1684804380000,
            "origQty": "100.00000000",
            "qty": "150.00000000",
            "origPrice": "1.25000000",
            "price": "1.20000000",
            "status": "SUCCESS",
            "executionId": 666666
        }"#;

        let amendment: OrderAmendment = serde_json::from_str(json).unwrap();
        assert_eq!(amendment.amendment_id, 444444);
        assert_eq!(amendment.symbol, "ADAUSDT");
        assert_eq!(amendment.order_id, 24680);
        assert_eq!(amendment.client_order_id, "adaOrder111");
        assert_eq!(amendment.time, 1684804380000);
        assert_eq!(amendment.orig_qty, dec!(100.00000000));
        assert_eq!(amendment.qty, dec!(150.00000000));
        assert_eq!(amendment.orig_price, dec!(1.25000000));
        assert_eq!(amendment.price, dec!(1.20000000));
        assert_eq!(amendment.status, "SUCCESS");
        assert_eq!(amendment.execution_id, 666666);
    }

    #[test]
    fn test_order_amendment_with_failed_status_deserialization() {
        let json = r#"{
            "amendmentId": 555555,
            "symbol": "SOLUSDT",
            "orderId": 13579,
            "clientOrderId": "solOrder222",
            "time": 1684804390000,
            "origQty": "5.00000000",
            "qty": "5.00000000",
            "origPrice": "150.00000000",
            "price": "150.00000000",
            "status": "FAILED",
            "executionId": 555555
        }"#;

        let amendment: OrderAmendment = serde_json::from_str(json).unwrap();
        assert_eq!(amendment.amendment_id, 555555);
        assert_eq!(amendment.symbol, "SOLUSDT");
        assert_eq!(amendment.order_id, 13579);
        assert_eq!(amendment.client_order_id, "solOrder222");
        assert_eq!(amendment.time, 1684804390000);
        assert_eq!(amendment.orig_qty, amendment.qty); // No changes due to failure
        assert_eq!(amendment.orig_price, amendment.price); // No changes due to failure
        assert_eq!(amendment.status, "FAILED");
        assert_eq!(amendment.execution_id, 555555);
    }

    #[test]
    fn test_order_amendment_with_high_precision_decimals_deserialization() {
        let json = r#"{
            "amendmentId": 666666,
            "symbol": "DOTUSDT",
            "orderId": 77777,
            "clientOrderId": "dotOrder333",
            "time": 1684804400000,
            "origQty": "123.45678901",
            "qty": "98.76543210",
            "origPrice": "25.12345678",
            "price": "24.98765432",
            "status": "SUCCESS",
            "executionId": 444444
        }"#;

        let amendment: OrderAmendment = serde_json::from_str(json).unwrap();
        assert_eq!(amendment.amendment_id, 666666);
        assert_eq!(amendment.symbol, "DOTUSDT");
        assert_eq!(amendment.order_id, 77777);
        assert_eq!(amendment.client_order_id, "dotOrder333");
        assert_eq!(amendment.time, 1684804400000);
        assert_eq!(amendment.orig_qty.to_string(), "123.45678901");
        assert_eq!(amendment.qty.to_string(), "98.76543210");
        assert_eq!(amendment.orig_price.to_string(), "25.12345678");
        assert_eq!(amendment.price.to_string(), "24.98765432");
        assert_eq!(amendment.status, "SUCCESS");
        assert_eq!(amendment.execution_id, 444444);
    }

    #[test]
    fn test_order_amendment_with_zero_values_deserialization() {
        let json = r#"{
            "amendmentId": 777777,
            "symbol": "MATICUSDT",
            "orderId": 88888,
            "clientOrderId": "maticOrder444",
            "time": 1684804410000,
            "origQty": "0.00000000",
            "qty": "0.00000000",
            "origPrice": "0.00000000",
            "price": "0.00000000",
            "status": "FAILED",
            "executionId": 333333
        }"#;

        let amendment: OrderAmendment = serde_json::from_str(json).unwrap();
        assert_eq!(amendment.amendment_id, 777777);
        assert_eq!(amendment.symbol, "MATICUSDT");
        assert_eq!(amendment.order_id, 88888);
        assert_eq!(amendment.client_order_id, "maticOrder444");
        assert_eq!(amendment.time, 1684804410000);
        assert_eq!(amendment.orig_qty, dec!(0));
        assert_eq!(amendment.qty, dec!(0));
        assert_eq!(amendment.orig_price, dec!(0));
        assert_eq!(amendment.price, dec!(0));
        assert_eq!(amendment.status, "FAILED");
        assert_eq!(amendment.execution_id, 333333);
    }

    #[test]
    fn test_order_amendment_with_large_values_deserialization() {
        let json = r#"{
            "amendmentId": 999999999,
            "symbol": "SHIBUSDT",
            "orderId": 999999999999,
            "clientOrderId": "shibOrder999",
            "time": 9999999999999,
            "origQty": "1000000000.00000000",
            "qty": "999999999.00000000",
            "origPrice": "0.00001234",
            "price": "0.00001230",
            "status": "SUCCESS",
            "executionId": 888888888
        }"#;

        let amendment: OrderAmendment = serde_json::from_str(json).unwrap();
        assert_eq!(amendment.amendment_id, 999999999);
        assert_eq!(amendment.symbol, "SHIBUSDT");
        assert_eq!(amendment.order_id, 999999999999);
        assert_eq!(amendment.client_order_id, "shibOrder999");
        assert_eq!(amendment.time, 9999999999999);
        assert_eq!(amendment.orig_qty.to_string(), "1000000000.00000000");
        assert_eq!(amendment.qty.to_string(), "999999999.00000000");
        assert_eq!(amendment.orig_price.to_string(), "0.00001234");
        assert_eq!(amendment.price.to_string(), "0.00001230");
        assert_eq!(amendment.status, "SUCCESS");
        assert_eq!(amendment.execution_id, 888888888);
    }

    #[test]
    fn test_order_amendments_array_deserialization_empty() {
        let json = "[]";
        let amendments: Vec<OrderAmendment> = serde_json::from_str(json).unwrap();
        assert!(amendments.is_empty());
    }

    #[test]
    fn test_order_amendments_array_deserialization_single() {
        let json = r#"[{
            "amendmentId": 111111,
            "symbol": "BTCUSDT",
            "orderId": 12345,
            "clientOrderId": "myOrder123",
            "time": 1684804350000,
            "origQty": "1.00000000",
            "qty": "0.50000000",
            "origPrice": "50000.00000000",
            "price": "48000.00000000",
            "status": "SUCCESS",
            "executionId": 999999
        }]"#;

        let amendments: Vec<OrderAmendment> = serde_json::from_str(json).unwrap();
        assert_eq!(amendments.len(), 1);

        let amendment = &amendments[0];
        assert_eq!(amendment.amendment_id, 111111);
        assert_eq!(amendment.symbol, "BTCUSDT");
        assert_eq!(amendment.order_id, 12345);
        assert_eq!(amendment.client_order_id, "myOrder123");
        assert_eq!(amendment.status, "SUCCESS");
    }

    #[test]
    fn test_order_amendments_array_deserialization_multiple() {
        let json = r#"[
            {
                "amendmentId": 111111,
                "symbol": "BTCUSDT",
                "orderId": 12345,
                "clientOrderId": "myOrder123",
                "time": 1684804350000,
                "origQty": "1.00000000",
                "qty": "0.50000000",
                "origPrice": "50000.00000000",
                "price": "48000.00000000",
                "status": "SUCCESS",
                "executionId": 999999
            },
            {
                "amendmentId": 222222,
                "symbol": "BTCUSDT",
                "orderId": 12345,
                "clientOrderId": "myOrder123",
                "time": 1684804360000,
                "origQty": "0.50000000",
                "qty": "0.25000000",
                "origPrice": "48000.00000000",
                "price": "47000.00000000",
                "status": "SUCCESS",
                "executionId": 888888
            },
            {
                "amendmentId": 333333,
                "symbol": "BTCUSDT",
                "orderId": 12345,
                "clientOrderId": "myOrder123",
                "time": 1684804370000,
                "origQty": "0.25000000",
                "qty": "0.25000000",
                "origPrice": "47000.00000000",
                "price": "46000.00000000",
                "status": "FAILED",
                "executionId": 777777
            }
        ]"#;

        let amendments: Vec<OrderAmendment> = serde_json::from_str(json).unwrap();
        assert_eq!(amendments.len(), 3);

        // Verify first amendment
        assert_eq!(amendments[0].amendment_id, 111111);
        assert_eq!(amendments[0].orig_qty, dec!(1.00000000));
        assert_eq!(amendments[0].qty, dec!(0.50000000));
        assert_eq!(amendments[0].orig_price, dec!(50000.00000000));
        assert_eq!(amendments[0].price, dec!(48000.00000000));
        assert_eq!(amendments[0].status, "SUCCESS");
        assert_eq!(amendments[0].execution_id, 999999);

        // Verify second amendment
        assert_eq!(amendments[1].amendment_id, 222222);
        assert_eq!(amendments[1].orig_qty, dec!(0.50000000));
        assert_eq!(amendments[1].qty, dec!(0.25000000));
        assert_eq!(amendments[1].orig_price, dec!(48000.00000000));
        assert_eq!(amendments[1].price, dec!(47000.00000000));
        assert_eq!(amendments[1].status, "SUCCESS");
        assert_eq!(amendments[1].execution_id, 888888);

        // Verify third amendment (failed)
        assert_eq!(amendments[2].amendment_id, 333333);
        assert_eq!(amendments[2].orig_qty, dec!(0.25000000));
        assert_eq!(amendments[2].qty, dec!(0.25000000));
        assert_eq!(amendments[2].orig_price, dec!(47000.00000000));
        assert_eq!(amendments[2].price, dec!(46000.00000000));
        assert_eq!(amendments[2].status, "FAILED");
        assert_eq!(amendments[2].execution_id, 777777);

        // Verify all amendments belong to the same order
        assert!(amendments.iter().all(|a| a.order_id == 12345));
        assert!(amendments.iter().all(|a| a.symbol == "BTCUSDT"));
        assert!(amendments.iter().all(|a| a.client_order_id == "myOrder123"));
    }

    #[test]
    fn test_order_amendment_different_status_types() {
        let status_types = vec!["SUCCESS", "FAILED", "PENDING", "REJECTED", "CANCELED"];

        for status in status_types {
            let json = format!(
                r#"{{
                "amendmentId": 111111,
                "symbol": "BTCUSDT",
                "orderId": 12345,
                "clientOrderId": "myOrder123",
                "time": 1684804350000,
                "origQty": "1.00000000",
                "qty": "0.50000000",
                "origPrice": "50000.00000000",
                "price": "48000.00000000",
                "status": "{}",
                "executionId": 999999
            }}"#,
                status
            );

            let amendment: OrderAmendment = serde_json::from_str(&json).unwrap();
            assert_eq!(amendment.status, status);
        }
    }

    #[test]
    fn test_order_amendment_with_minimal_required_fields() {
        // Test with smallest possible valid values
        let json = r#"{
            "amendmentId": 1,
            "symbol": "BTC",
            "orderId": 1,
            "clientOrderId": "1",
            "time": 0,
            "origQty": "0.00000001",
            "qty": "0.00000001",
            "origPrice": "0.00000001",
            "price": "0.00000001",
            "status": "SUCCESS",
            "executionId": 1
        }"#;

        let amendment: OrderAmendment = serde_json::from_str(json).unwrap();
        assert_eq!(amendment.amendment_id, 1);
        assert_eq!(amendment.symbol, "BTC");
        assert_eq!(amendment.order_id, 1);
        assert_eq!(amendment.client_order_id, "1");
        assert_eq!(amendment.time, 0);
        assert_eq!(amendment.orig_qty.to_string(), "0.00000001");
        assert_eq!(amendment.qty.to_string(), "0.00000001");
        assert_eq!(amendment.orig_price.to_string(), "0.00000001");
        assert_eq!(amendment.price.to_string(), "0.00000001");
        assert_eq!(amendment.status, "SUCCESS");
        assert_eq!(amendment.execution_id, 1);
    }
}
