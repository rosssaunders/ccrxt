use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::client::RestClient;
use crate::cryptocom::{RestResult, enums::*};

/// Request for getting OCO order details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderListRequest {
    /// Contingency type (must be OCO)
    pub contingency_type: ContingencyType,
    /// List ID of the OCO order
    pub list_id: String,
    /// Instrument name
    pub instrument_name: String,
}

/// Individual order details in OCO order list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderDetails {
    /// Account ID
    pub account_id: String,
    /// Order ID
    pub order_id: String,
    /// Client order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Order side
    pub side: OrderSide,
    /// Execution instructions
    pub exec_inst: Vec<ExecInst>,
    /// Order quantity
    pub quantity: String,
    /// Order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Order value
    pub order_value: String,
    /// Average price
    pub avg_price: String,
    /// Trigger price
    pub trigger_price: String,
    /// Cumulative quantity
    pub cumulative_quantity: String,
    /// Cumulative value
    pub cumulative_value: String,
    /// Cumulative fee
    pub cumulative_fee: String,
    /// Order status
    pub status: String,
    /// Update user ID
    pub update_user_id: String,
    /// Order date
    pub order_date: String,
    /// Instrument name
    pub instrument_name: String,
    /// Fee instrument name
    pub fee_instrument_name: String,
    /// List ID
    pub list_id: String,
    /// Contingency type
    pub contingency_type: ContingencyType,
    /// Trigger price type
    pub trigger_price_type: String,
    /// Create time (milliseconds)
    pub create_time: u64,
    /// Create time (nanoseconds)
    pub create_time_ns: String,
    /// Update time (milliseconds)
    pub update_time: u64,
}

/// Response for getting OCO order details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderListResponse {
    /// Array of order details
    pub data: Vec<OrderDetails>,
}

impl RestClient {
    /// Get OCO order details
    ///
    /// Gets the details of an outstanding (not executed) contingency order on Exchange.
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html>
    ///
    /// Rate limit: 10 requests per second per user
    ///
    /// # Arguments
    /// * `request` - The OCO order details request
    ///
    /// # Returns
    /// Response containing the order details for the OCO order
    #[allow(clippy::indexing_slicing)] // Safe: adding optional keys to JSON object
    pub async fn get_order_list(&self, request: GetOrderListRequest) -> RestResult<Value> {
        
        let params = serde_json::to_value(&request)?;

        self.send_signed_request("private/get-order-list", params)
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
    fn test_get_order_list_request_structure() {
        let request_json = json!({
            "contingency_type": "OCO",
            "list_id": "6498090546073120100",
            "instrument_name": "BTCUSD-PERP"
        });

        let request: GetOrderListRequest = serde_json::from_value(request_json).unwrap();
        assert_eq!(request.contingency_type, ContingencyType::Oco);
        assert_eq!(request.list_id, "6498090546073120100");
        assert_eq!(request.instrument_name, "BTCUSD-PERP");
    }

    #[test]
    fn test_order_details_structure() {
        let order_json = json!({
            "account_id": "88888888-8888-8888-8888-000000000001",
            "order_id": "4611686018427387905",
            "client_oid": "1661331443",
            "type": "LIMIT",
            "time_in_force": "GOOD_TILL_CANCEL",
            "side": "SELL",
            "exec_inst": [],
            "quantity": "0.1000",
            "price": "23000.0",
            "order_value": "2300.00000000",
            "avg_price": "0",
            "trigger_price": "0",
            "cumulative_quantity": "0",
            "cumulative_value": "0",
            "cumulative_fee": "0",
            "status": "ACTIVE",
            "update_user_id": "11111111-1111-1111-1111-000000000001",
            "order_date": "2022-08-24",
            "instrument_name": "BTCUSD-PERP",
            "fee_instrument_name": "USD",
            "list_id": "6498090546073120100",
            "contingency_type": "OCO",
            "trigger_price_type": "NULL_VAL",
            "create_time": 1661331445398_u64,
            "create_time_ns": "1661331445398773329",
            "update_time": 1661331445482_u64
        });

        let order: OrderDetails = serde_json::from_value(order_json).unwrap();
        assert_eq!(order.account_id, "88888888-8888-8888-8888-000000000001");
        assert_eq!(order.order_id, "4611686018427387905");
        assert_eq!(order.client_oid, Some("1661331443".to_string()));
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.time_in_force, TimeInForce::GoodTillCancel);
        assert_eq!(order.side, OrderSide::Sell);
        assert_eq!(order.quantity, "0.1000");
        assert_eq!(order.price, Some("23000.0".to_string()));
        assert_eq!(order.status, "ACTIVE");
        assert_eq!(order.instrument_name, "BTCUSD-PERP");
        assert_eq!(order.list_id, "6498090546073120100");
        assert_eq!(order.contingency_type, ContingencyType::Oco);
        assert_eq!(order.create_time, 1661331445398);
        assert_eq!(order.update_time, 1661331445482);
    }

    #[test]
    fn test_order_details_stop_loss_structure() {
        let order_json = json!({
            "account_id": "88888888-8888-8888-8888-000000000001",
            "order_id": "4611686018427387906",
            "client_oid": "1661331443-2",
            "type": "STOP_LOSS",
            "time_in_force": "GOOD_TILL_CANCEL",
            "side": "SELL",
            "exec_inst": [],
            "quantity": "0.1000",
            "order_value": "1900.00000000",
            "avg_price": "0",
            "trigger_price": "0",
            "cumulative_quantity": "0",
            "cumulative_value": "0",
            "cumulative_fee": "0",
            "status": "PENDING",
            "update_user_id": "11111111-1111-1111-1111-000000000001",
            "order_date": "2022-08-24",
            "instrument_name": "BTCUSD-PERP",
            "fee_instrument_name": "USD",
            "list_id": "6498090546073120100",
            "contingency_type": "OCO",
            "trigger_price_type": "NULL_VAL",
            "create_time": 1661331445040_u64,
            "create_time_ns": "1661331445040100934",
            "update_time": 0
        });

        let order: OrderDetails = serde_json::from_value(order_json).unwrap();
        assert_eq!(order.order_type, OrderType::StopLoss);
        assert_eq!(order.status, "PENDING");
        assert_eq!(order.update_time, 0);
        assert!(order.price.is_none()); // STOP_LOSS orders don't have price field
    }

    #[test]
    fn test_get_order_list_response_structure() {
        let response_json = json!({
            "data": [
                {
                    "account_id": "88888888-8888-8888-8888-000000000001",
                    "order_id": "4611686018427387905",
                    "client_oid": "1661331443",
                    "type": "LIMIT",
                    "time_in_force": "GOOD_TILL_CANCEL",
                    "side": "SELL",
                    "exec_inst": [],
                    "quantity": "0.1000",
                    "price": "23000.0",
                    "order_value": "2300.00000000",
                    "avg_price": "0",
                    "trigger_price": "0",
                    "cumulative_quantity": "0",
                    "cumulative_value": "0",
                    "cumulative_fee": "0",
                    "status": "ACTIVE",
                    "update_user_id": "11111111-1111-1111-1111-000000000001",
                    "order_date": "2022-08-24",
                    "instrument_name": "BTCUSD-PERP",
                    "fee_instrument_name": "USD",
                    "list_id": "6498090546073120100",
                    "contingency_type": "OCO",
                    "trigger_price_type": "NULL_VAL",
                    "create_time": 1661331445398_u64,
                    "create_time_ns": "1661331445398773329",
                    "update_time": 1661331445482_u64
                }
            ]
        });

        let response: GetOrderListResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(
            response.data.first().unwrap().order_id,
            "4611686018427387905"
        );
        assert_eq!(
            response.data.first().unwrap().contingency_type,
            ContingencyType::Oco
        );
    }

    #[test]
    fn test_get_order_list_request_serialization() {
        let request = GetOrderListRequest {
            contingency_type: ContingencyType::Oco,
            list_id: "6498090546073120100".to_string(),
            instrument_name: "BTCUSD-PERP".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("contingency_type").unwrap(), "OCO");
        assert_eq!(serialized.get("list_id").unwrap(), "6498090546073120100");
        assert_eq!(serialized.get("instrument_name").unwrap(), "BTCUSD-PERP");
    }

    #[test]
    fn test_order_details_with_empty_exec_inst() {
        let order_json = json!({
            "account_id": "test-account",
            "order_id": "test-order",
            "type": "LIMIT",
            "time_in_force": "GOOD_TILL_CANCEL",
            "side": "BUY",
            "exec_inst": [],
            "quantity": "1.0",
            "price": "100.0",
            "order_value": "100.0",
            "avg_price": "0",
            "trigger_price": "0",
            "cumulative_quantity": "0",
            "cumulative_value": "0",
            "cumulative_fee": "0",
            "status": "NEW",
            "update_user_id": "test-user",
            "order_date": "2023-01-01",
            "instrument_name": "BTC_USDT",
            "fee_instrument_name": "USDT",
            "list_id": "test-list",
            "contingency_type": "OCO",
            "trigger_price_type": "NULL_VAL",
            "create_time": 1672531200000_u64,
            "create_time_ns": "1672531200000000000",
            "update_time": 1672531200000_u64
        });

        let order: OrderDetails = serde_json::from_value(order_json).unwrap();
        assert_eq!(order.exec_inst.len(), 0);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.side, OrderSide::Buy);
    }
}
