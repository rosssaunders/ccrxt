use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::RestResult;

/// Enum representing the status of an order
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    Pending,
    Active,
}

/// Request parameters for getting open orders
#[derive(Debug, Clone, Serialize)]
pub struct GetOpenOrdersRequest {
    /// Instrument name e.g. BTCUSD-PERP (omit for 'all')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
}

/// Open order information
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct OpenOrder {
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
    /// Order status: NEW, PENDING, ACTIVE
    pub status: String,
    /// Updated user ID
    pub update_user_id: String,
    /// Order creation date
    pub order_date: String,
    /// Order creation timestamp
    pub create_time: u64,
    /// Order creation timestamp (nanosecond)
    pub create_time_ns: String,
    /// Order update timestamp
    pub update_time: u64,
    /// Instrument name
    pub instrument_name: String,
    /// Currency used for the fees
    pub fee_instrument_name: String,
}

/// Response for getting open orders
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct GetOpenOrdersResponse {
    /// Array of open orders
    pub data: Vec<OpenOrder>,
}

impl RestClient {
    /// Gets all open orders for a particular instrument
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html>
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The get open orders parameters
    ///
    /// # Returns
    /// Array of open orders
    pub async fn get_open_orders(&self, request: GetOpenOrdersRequest) -> RestResult<GetOpenOrdersResponse> {
        let params = serde_json::to_value(&request).map_err(|e| crate::cryptocom::Errors::Error(format!("Serialization error: {}", e)))?;

        self.send_signed_request("private/get-open-orders", params)
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
    fn test_get_open_orders_request_with_instrument() {
        let request = GetOpenOrdersRequest {
            instrument_name: Some("BTCUSD-PERP".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instrument_name").unwrap(), "BTCUSD-PERP");
    }

    #[test]
    fn test_get_open_orders_request_all_instruments() {
        let request = GetOpenOrdersRequest {
            instrument_name: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert!(
            !serialized
                .as_object()
                .unwrap()
                .contains_key("instrument_name")
        );
    }

    #[test]
    fn test_open_order_structure() {
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
            "create_time": 1613575617173_u64,
            "create_time_ns": "1613575617173123456",
            "update_time": 1613575617173_u64
        });

        let order: OpenOrder = serde_json::from_value(order_json).unwrap();
        assert_eq!(order.account_id, "52e7c00f-1324-5a6z-bfgt-de445bde21a5");
        assert_eq!(order.order_id, "19848525");
        assert_eq!(order.client_oid, "1613571154900");
        assert_eq!(order.order_type, "LIMIT");
        assert_eq!(order.side, "BUY");
        assert_eq!(order.status, "ACTIVE");
        assert_eq!(order.instrument_name, "BTCUSD-PERP");
        assert_eq!(order.exec_inst.len(), 0);
    }

    #[test]
    fn test_get_open_orders_response_structure() {
        let response_json = json!({
            "data": [{
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
                "create_time": 1613575617173_u64,
                "create_time_ns": "1613575617173123456",
                "update_time": 1613575617173_u64
            }]
        });

        let response: GetOpenOrdersResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().order_id, "19848525");
        assert_eq!(response.data.first().unwrap().status, "ACTIVE");
    }
}
