use super::client::RestClient;
use crate::cryptocom::{enums::*, RestResult};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Individual order in an order list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderListItem {
    /// Instrument name (e.g., ETH_CRO, BTC_USDT)
    pub instrument_name: String,
    /// Order side (BUY or SELL)
    pub side: OrderSide,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Order price (required for LIMIT and STOP_LIMIT orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Order quantity (required for most order types)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    /// Notional amount (for MARKET BUY orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notional: Option<String>,
    /// Client order ID (maximum 36 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
    /// Time in force (LIMIT orders only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    /// Execution instructions (LIMIT orders only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_inst: Option<Vec<ExecInst>>,
    /// Trigger price (for stop orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    /// STP scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_scope: Option<StpScope>,
    /// STP instruction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_inst: Option<StpInst>,
    /// STP ID (0 to 32767)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_id: Option<u16>,
    /// Preferred fee token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_instrument_name: Option<String>,
    /// Reference price for OCO orders (STOP_LOSS orders in OCO)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_price: Option<String>,
}

/// Request for creating an order list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderListRequest {
    /// Contingency type (LIST or OCO)
    pub contingency_type: ContingencyType,
    /// List of orders (1-10 for LIST, exactly 2 for OCO)
    pub order_list: Vec<OrderListItem>,
}

/// Result for individual order creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreationResult {
    /// Index of the order in the request (starts from 0)
    pub index: u32,
    /// Status code (0 if successful)
    pub code: i32,
    /// Error message (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Order ID (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Client order ID (if provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
}

/// Response for creating a list of orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderListResponse {
    /// List of order creation results
    pub result_list: Vec<OrderCreationResult>,
}

/// Response for creating OCO orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOcoOrderResponse {
    /// List ID for the OCO order
    pub list_id: String,
}

impl RestClient {
    /// Create a list of orders
    ///
    /// Creates multiple orders in a single request. Supports both LIST (1-10 orders)
    /// and OCO (exactly 2 orders) contingency types.
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html>
    ///
    /// Rate limit: 10 requests per second per user (if more than 1 order)
    ///
    /// # Arguments
    /// * `request` - The order list creation request
    ///
    /// # Returns
    /// The response depends on contingency type:
    /// - LIST: CreateOrderListResponse with individual order results
    /// - OCO: CreateOcoOrderResponse with list_id
    pub async fn create_order_list(&self, request: CreateOrderListRequest) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;
        let params = serde_json::to_value(&request)?;

        let signature = self.sign_request("private/create-order-list", id, &params, nonce)?;

        let request_body = json!({
            "id": id,
            "method": "private/create-order-list",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self
            .client
            .post(format!("{}/v1/private/create-order-list", self.base_url))
            .json(&request_body)
            .send()
            .await?;

        let result: Value = response.json().await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rest::secrets::ExposableSecret;
    use serde_json::json;

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
    fn test_order_list_item_structure() {
        let order_json = json!({
            "instrument_name": "ETH_CRO",
            "side": "BUY",
            "type": "LIMIT",
            "price": "5799",
            "quantity": "1",
            "client_oid": "my_order_0001",
            "time_in_force": "GOOD_TILL_CANCEL",
            "exec_inst": ["POST_ONLY"]
        });

        let order: OrderListItem = serde_json::from_value(order_json).unwrap();
        assert_eq!(order.instrument_name, "ETH_CRO");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.price, Some("5799".to_string()));
        assert_eq!(order.quantity, Some("1".to_string()));
        assert_eq!(order.client_oid, Some("my_order_0001".to_string()));
        assert_eq!(order.time_in_force, Some(TimeInForce::GoodTillCancel));
        assert!(order.exec_inst.is_some());
        assert_eq!(order.exec_inst.unwrap(), vec![ExecInst::PostOnly]);
    }

    #[test]
    fn test_create_order_list_request_structure() {
        let request_json = json!({
            "contingency_type": "LIST",
            "order_list": [
                {
                    "instrument_name": "ETH_CRO",
                    "side": "BUY",
                    "type": "LIMIT",
                    "price": "5799",
                    "quantity": "1",
                    "client_oid": "my_order_0001",
                    "time_in_force": "GOOD_TILL_CANCEL",
                    "exec_inst": ["POST_ONLY"]
                },
                {
                    "instrument_name": "ETH_CRO",
                    "side": "BUY",
                    "type": "LIMIT",
                    "price": "5780",
                    "quantity": "1",
                    "client_oid": "my_order_0002",
                    "time_in_force": "GOOD_TILL_CANCEL",
                    "exec_inst": ["POST_ONLY"]
                }
            ]
        });

        let request: CreateOrderListRequest = serde_json::from_value(request_json).unwrap();
        assert_eq!(request.contingency_type, ContingencyType::List);
        assert_eq!(request.order_list.len(), 2);
        assert_eq!(request.order_list.first().unwrap().instrument_name, "ETH_CRO");
        assert_eq!(
            request.order_list.get(1).unwrap().client_oid,
            Some("my_order_0002".to_string())
        );
    }

    #[test]
    fn test_create_order_list_response_structure() {
        let response_json = json!({
            "result_list": [
                {
                    "index": 0,
                    "code": 0,
                    "order_id": "2015106383706015873",
                    "client_oid": "my_order_0001"
                },
                {
                    "index": 1,
                    "code": 0,
                    "order_id": "2015119459882149857",
                    "client_oid": "my_order_0002"
                }
            ]
        });

        let response: CreateOrderListResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result_list.len(), 2);
        assert_eq!(response.result_list.first().unwrap().index, 0);
        assert_eq!(response.result_list.first().unwrap().code, 0);
        assert_eq!(
            response.result_list.first().unwrap().order_id,
            Some("2015106383706015873".to_string())
        );
        assert_eq!(
            response.result_list.get(1).unwrap().client_oid,
            Some("my_order_0002".to_string())
        );
    }

    #[test]
    fn test_create_order_list_response_with_errors() {
        let response_json = json!({
            "result_list": [
                {
                    "index": 0,
                    "code": 0,
                    "order_id": "2015106383706015873",
                    "client_oid": "my_order_0001"
                },
                {
                    "index": 1,
                    "code": 20007,
                    "message": "INVALID_REQUEST",
                    "client_oid": "my_order_0002"
                }
            ]
        });

        let response: CreateOrderListResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result_list.len(), 2);
        assert_eq!(response.result_list.first().unwrap().code, 0);
        assert_eq!(response.result_list.get(1).unwrap().code, 20007);
        assert_eq!(
            response.result_list.get(1).unwrap().message,
            Some("INVALID_REQUEST".to_string())
        );
        assert!(response.result_list.get(1).unwrap().order_id.is_none());
    }

    #[test]
    fn test_oco_order_request_structure() {
        let request_json = json!({
            "contingency_type": "OCO",
            "order_list": [
                {
                    "instrument_name": "BTCUSD-PERP",
                    "quantity": "0.1",
                    "type": "LIMIT",
                    "price": "23000",
                    "side": "SELL"
                },
                {
                    "instrument_name": "BTCUSD-PERP",
                    "quantity": "0.1",
                    "type": "STOP_LOSS",
                    "ref_price": "19000",
                    "side": "SELL"
                }
            ]
        });

        let request: CreateOrderListRequest = serde_json::from_value(request_json).unwrap();
        assert_eq!(request.contingency_type, ContingencyType::Oco);
        assert_eq!(request.order_list.len(), 2);
        assert_eq!(request.order_list.first().unwrap().order_type, OrderType::Limit);
        assert_eq!(request.order_list.get(1).unwrap().order_type, OrderType::StopLoss);
        assert_eq!(request.order_list.get(1).unwrap().ref_price, Some("19000".to_string()));
    }

    #[test]
    fn test_create_oco_order_response_structure() {
        let response_json = json!({
            "list_id": "6498090546073120100"
        });

        let response: CreateOcoOrderResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.list_id, "6498090546073120100");
    }

    #[test]
    fn test_order_with_stp_fields() {
        let order_json = json!({
            "instrument_name": "BTC_USDT",
            "side": "BUY",
            "type": "LIMIT",
            "price": "50000",
            "quantity": "0.1",
            "stp_scope": "M",
            "stp_inst": "B",
            "stp_id": 100
        });

        let order: OrderListItem = serde_json::from_value(order_json).unwrap();
        assert_eq!(order.stp_scope, Some(StpScope::MasterOrSubAccount));
        assert_eq!(order.stp_inst, Some(StpInst::CancelBoth));
        assert_eq!(order.stp_id, Some(100));
    }

    #[test]
    fn test_order_serialization() {
        let order = OrderListItem {
            instrument_name: "BTC_USDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            price: Some("50000".to_string()),
            quantity: Some("0.1".to_string()),
            notional: None,
            client_oid: Some("test_order_123".to_string()),
            time_in_force: Some(TimeInForce::GoodTillCancel),
            exec_inst: Some(vec![ExecInst::PostOnly]),
            trigger_price: None,
            stp_scope: None,
            stp_inst: None,
            stp_id: None,
            fee_instrument_name: None,
            ref_price: None,
        };

        let serialized = serde_json::to_value(&order).unwrap();
        assert_eq!(serialized.get("instrument_name").unwrap(), "BTC_USDT");
        assert_eq!(serialized.get("side").unwrap(), "BUY");
        assert_eq!(serialized.get("type").unwrap(), "LIMIT");
        assert!(!serialized.as_object().unwrap().contains_key("notional"));
        assert!(!serialized
            .as_object()
            .unwrap()
            .contains_key("trigger_price"));
    }
}
