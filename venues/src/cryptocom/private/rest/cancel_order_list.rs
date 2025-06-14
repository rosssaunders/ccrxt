use super::client::RestClient;
use crate::cryptocom::{enums::ContingencyType, RestResult};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Individual order to cancel in a list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderListItem {
    /// Instrument name (e.g., ETH_CRO, BTC_USDT)
    pub instrument_name: String,
    /// Order ID to cancel
    pub order_id: String,
}

/// Request for canceling an order list (LIST type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderListRequest {
    /// Contingency type (must be LIST)
    pub contingency_type: ContingencyType,
    /// List of orders to cancel
    pub order_list: Vec<CancelOrderListItem>,
}

/// Request for canceling OCO orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOcoOrderRequest {
    /// Contingency type (must be OCO)
    pub contingency_type: ContingencyType,
    /// List ID of the OCO order to cancel
    pub list_id: String,
    /// Instrument name
    pub instrument_name: String,
}

/// Result for individual order cancellation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCancellationResult {
    /// Index of the order in the request (starts from 0)
    pub index: u32,
    /// Status code (0 if successful)
    pub code: i32,
    /// Error message (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Response for canceling a list of orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderListResponse {
    /// List of order cancellation results
    pub result_list: Vec<OrderCancellationResult>,
}

impl RestClient {
    /// Cancel a list of orders
    ///
    /// Cancels multiple orders in a single request using the LIST contingency type.
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html#private-cancel-order-list>
    ///
    /// Rate limit: 10 requests per second per user
    ///
    /// # Arguments
    /// * `request` - The order list cancellation request
    ///
    /// # Returns
    /// Response with cancellation results for each order
    pub async fn cancel_order_list(&self, request: CancelOrderListRequest) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;
        let params = serde_json::to_value(&request)?;

        let signature = self.sign_request("private/cancel-order-list", id, &params, nonce)?;

        let request_body = json!({
            "id": id,
            "method": "private/cancel-order-list",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self
            .client
            .post(&format!("{}/v1/private/cancel-order-list", self.base_url))
            .json(&request_body)
            .send()
            .await?;

        let result: Value = response.json().await?;
        Ok(result)
    }

    /// Cancel OCO orders
    ///
    /// Cancels a contingency order (OCO) using the list ID.
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html#private-cancel-order-list>
    ///
    /// Rate limit: 10 requests per second per user
    ///
    /// # Arguments
    /// * `request` - The OCO order cancellation request
    ///
    /// # Returns
    /// Response confirming the cancellation request
    pub async fn cancel_oco_order(&self, request: CancelOcoOrderRequest) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;
        let params = serde_json::to_value(&request)?;

        let signature = self.sign_request("private/cancel-order-list", id, &params, nonce)?;

        let request_body = json!({
            "id": id,
            "method": "private/cancel-order-list",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self
            .client
            .post(&format!("{}/v1/private/cancel-order-list", self.base_url))
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
    struct PlainTextSecret {
        secret: String,
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    impl PlainTextSecret {
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    #[test]
    fn test_cancel_order_list_item_structure() {
        let item_json = json!({
            "instrument_name": "ETH_CRO",
            "order_id": "2015106383706015873"
        });

        let item: CancelOrderListItem = serde_json::from_value(item_json).unwrap();
        assert_eq!(item.instrument_name, "ETH_CRO");
        assert_eq!(item.order_id, "2015106383706015873");
    }

    #[test]
    fn test_cancel_order_list_request_structure() {
        let request_json = json!({
            "contingency_type": "LIST",
            "order_list": [
                {
                    "instrument_name": "ETH_CRO",
                    "order_id": "2015106383706015873"
                },
                {
                    "instrument_name": "ETH_CRO",
                    "order_id": "2015119459882149857"
                }
            ]
        });

        let request: CancelOrderListRequest = serde_json::from_value(request_json).unwrap();
        assert_eq!(request.contingency_type, ContingencyType::List);
        assert_eq!(request.order_list.len(), 2);
        assert_eq!(request.order_list[0].instrument_name, "ETH_CRO");
        assert_eq!(request.order_list[1].order_id, "2015119459882149857");
    }

    #[test]
    fn test_cancel_oco_order_request_structure() {
        let request_json = json!({
            "contingency_type": "OCO",
            "list_id": "4421958062479290999",
            "instrument_name": "BTCUSD-PERP"
        });

        let request: CancelOcoOrderRequest = serde_json::from_value(request_json).unwrap();
        assert_eq!(request.contingency_type, ContingencyType::Oco);
        assert_eq!(request.list_id, "4421958062479290999");
        assert_eq!(request.instrument_name, "BTCUSD-PERP");
    }

    #[test]
    fn test_cancel_order_list_response_structure() {
        let response_json = json!({
            "result_list": [
                {
                    "index": 0,
                    "code": 0
                },
                {
                    "index": 1,
                    "code": 0
                }
            ]
        });

        let response: CancelOrderListResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result_list.len(), 2);
        assert_eq!(response.result_list[0].index, 0);
        assert_eq!(response.result_list[0].code, 0);
        assert_eq!(response.result_list[1].index, 1);
        assert_eq!(response.result_list[1].code, 0);
        assert!(response.result_list[0].message.is_none());
    }

    #[test]
    fn test_cancel_order_list_response_with_errors() {
        let response_json = json!({
            "result_list": [
                {
                    "index": 0,
                    "code": 0
                },
                {
                    "index": 1,
                    "code": 20007,
                    "message": "INVALID_REQUEST"
                }
            ]
        });

        let response: CancelOrderListResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result_list.len(), 2);
        assert_eq!(response.result_list[0].code, 0);
        assert_eq!(response.result_list[1].code, 20007);
        assert_eq!(
            response.result_list[1].message,
            Some("INVALID_REQUEST".to_string())
        );
    }

    #[test]
    fn test_order_cancellation_result_serialization() {
        let result = OrderCancellationResult {
            index: 0,
            code: 0,
            message: None,
        };

        let serialized = serde_json::to_value(&result).unwrap();
        assert_eq!(serialized.get("index").unwrap(), 0);
        assert_eq!(serialized.get("code").unwrap(), 0);
        assert!(!serialized.as_object().unwrap().contains_key("message"));
    }

    #[test]
    fn test_cancel_order_list_request_serialization() {
        let request = CancelOrderListRequest {
            contingency_type: ContingencyType::List,
            order_list: vec![CancelOrderListItem {
                instrument_name: "BTC_USDT".to_string(),
                order_id: "123456789".to_string(),
            }],
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("contingency_type").unwrap(), "LIST");
        assert_eq!(serialized.get("order_list").unwrap()[0].get("instrument_name").unwrap(), "BTC_USDT");
        assert_eq!(serialized.get("order_list").unwrap()[0].get("order_id").unwrap(), "123456789");
    }

    #[test]
    fn test_cancel_oco_order_request_serialization() {
        let request = CancelOcoOrderRequest {
            contingency_type: ContingencyType::Oco,
            list_id: "6498090546073120100".to_string(),
            instrument_name: "BTCUSD-PERP".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("contingency_type").unwrap(), "OCO");
        assert_eq!(serialized.get("list_id").unwrap(), "6498090546073120100");
        assert_eq!(serialized.get("instrument_name").unwrap(), "BTCUSD-PERP");
    }
}
