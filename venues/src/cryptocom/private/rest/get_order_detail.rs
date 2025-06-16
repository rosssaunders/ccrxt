use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::client::RestClient;
use crate::cryptocom::RestResult;

/// Request parameters for getting order detail
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderDetailRequest {
    /// Order ID (string format is highly recommended)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Client Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
}

/// Order detail information
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct OrderDetail {
    /// Account ID
    pub account_id: String,
    /// Order ID
    pub order_id: String,
    /// Client Order ID
    pub client_oid: String,
    /// Order type: MARKET, LIMIT, STOP_LOSS, STOP_LIMIT, TAKE_PROFIT, TAKE_PROFIT_LIMIT
    pub order_type: String,
    /// Time in force: GOOD_TILL_CANCEL, IMMEDIATE_OR_CANCEL, FILL_OR_KILL
    pub time_in_force: String,
    /// Order side: BUY or SELL
    pub side: String,
    /// Execution instructions: POST_ONLY, LIQUIDATION
    pub exec_inst: Vec<String>,
    /// Quantity specified in the order
    pub quantity: String,
    /// Limit price specified in the order
    pub limit_price: String,
    /// Order value
    pub order_value: String,
    /// User's maker fee rate
    pub maker_fee_rate: String,
    /// User's taker fee rate
    pub taker_fee_rate: String,
    /// Average price
    pub avg_price: String,
    /// Cumulative executed quantity
    pub cumulative_quantity: String,
    /// Cumulative executed value
    pub cumulative_value: String,
    /// Cumulative executed fee
    pub cumulative_fee: String,
    /// Order status: REJECTED, CANCELED, FILLED, EXPIRED
    pub status: String,
    /// Updated user ID
    pub update_user_id: String,
    /// Order creation date
    pub order_date: String,
    /// Instrument name
    pub instrument_name: String,
    /// Currency used for the fees
    pub fee_instrument_name: String,
    /// Reason code (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<u32>,
    /// Order creation timestamp
    pub create_time: u64,
    /// Order creation timestamp (nanosecond)
    pub create_time_ns: String,
    /// Order update timestamp
    pub update_time: u64,
}

impl RestClient {
    /// Gets order detail for a specific order
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html>
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The get order detail parameters
    ///
    /// # Returns
    /// Order detail information
    #[allow(clippy::indexing_slicing)] // Safe: adding optional keys to JSON object
    pub async fn get_order_detail(&self, request: GetOrderDetailRequest) -> RestResult<Value> {
        let params = serde_json::to_value(&request).map_err(|e| crate::cryptocom::Errors::Error(format!("Serialization error: {}", e)))?;

        self.send_signed_request("private/get-order-detail", params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;
    use serde_json::json;

    use super::*;

    /// A plain text implementation of ExposableSecret for testing purposes.
    #[derive(Clone)]
    #[allow(dead_code)]
    struct PlainTextSecret {
        secret: String,
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    impl PlainTextSecret {
        #[allow(dead_code)]
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    #[test]
    fn test_get_order_detail_request_by_order_id() {
        let request = GetOrderDetailRequest {
            order_id: Some("19848525".to_string()),
            client_oid: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("order_id").unwrap(), "19848525");
        assert!(!serialized.as_object().unwrap().contains_key("client_oid"));
    }

    #[test]
    fn test_get_order_detail_request_by_client_oid() {
        let request = GetOrderDetailRequest {
            order_id: None,
            client_oid: Some("1613571154900".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("client_oid").unwrap(), "1613571154900");
        assert!(!serialized.as_object().unwrap().contains_key("order_id"));
    }

    #[test]
    fn test_order_detail_structure_with_reason() {
        let order_json = json!({
            "account_id": "52e7c00f-1324-5a6z-bfgt-de445bde21a5",
            "order_id": "19848525",
            "client_oid": "1613571154900",
            "order_type": "LIMIT",
            "time_in_force": "GOOD_TILL_CANCEL",
            "side": "BUY",
            "exec_inst": [],
            "quantity": "0.0100",
            "limit_price": "50000.0",
            "order_value": "500.000000",
            "maker_fee_rate": "0.000250",
            "taker_fee_rate": "0.000400",
            "avg_price": "0.0",
            "cumulative_quantity": "0.0000",
            "cumulative_value": "0.000000",
            "cumulative_fee": "0.000000",
            "status": "ACTIVE",
            "update_user_id": "fd797356-55db-48c2-a44d-157aabf702e8",
            "order_date": "2021-02-17",
            "instrument_name": "BTCUSD-PERP",
            "fee_instrument_name": "USD",
            "reason": 43012,
            "create_time": 1613575617173_u64,
            "create_time_ns": "1613575617173123456",
            "update_time": 1613575617173_u64
        });

        let order: OrderDetail = serde_json::from_value(order_json).unwrap();
        assert_eq!(order.account_id, "52e7c00f-1324-5a6z-bfgt-de445bde21a5");
        assert_eq!(order.order_id, "19848525");
        assert_eq!(order.client_oid, "1613571154900");
        assert_eq!(order.order_type, "LIMIT");
        assert_eq!(order.side, "BUY");
        assert_eq!(order.status, "ACTIVE");
        assert_eq!(order.instrument_name, "BTCUSD-PERP");
        assert_eq!(order.reason, Some(43012));
    }

    #[test]
    fn test_order_detail_structure_without_reason() {
        let order_json = json!({
            "account_id": "52e7c00f-1324-5a6z-bfgt-de445bde21a5",
            "order_id": "19848525",
            "client_oid": "1613571154900",
            "order_type": "LIMIT",
            "time_in_force": "GOOD_TILL_CANCEL",
            "side": "BUY",
            "exec_inst": [],
            "quantity": "0.0100",
            "limit_price": "50000.0",
            "order_value": "500.000000",
            "maker_fee_rate": "0.000250",
            "taker_fee_rate": "0.000400",
            "avg_price": "0.0",
            "cumulative_quantity": "0.0000",
            "cumulative_value": "0.000000",
            "cumulative_fee": "0.000000",
            "status": "FILLED",
            "update_user_id": "fd797356-55db-48c2-a44d-157aabf702e8",
            "order_date": "2021-02-17",
            "instrument_name": "BTCUSD-PERP",
            "fee_instrument_name": "USD",
            "create_time": 1613575617173_u64,
            "create_time_ns": "1613575617173123456",
            "update_time": 1613575617173_u64
        });

        let order: OrderDetail = serde_json::from_value(order_json).unwrap();
        assert_eq!(order.status, "FILLED");
        assert_eq!(order.reason, None);
    }
}
