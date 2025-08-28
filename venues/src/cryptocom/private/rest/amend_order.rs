use serde::{Deserialize, Serialize};

use crate::cryptocom::{PrivateRestClient as RestClient, RestResult};

/// Endpoint path for the amend-order API
const AMEND_ORDER_ENDPOINT: &str = "exchange/v1/private/amend-order";

/// Request parameters for amending an existing order
#[derive(Debug, Clone, Serialize)]
pub struct AmendOrderRequest {
    /// Order ID (string format is highly recommended)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Original Client Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_client_oid: Option<String>,

    /// The new amended price (if no change required, input original value)
    pub new_price: String,

    /// The new amended quantity (if no change required, input original value)  
    pub new_quantity: String,
}

/// Response for amending an order
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct AmendOrderResponse {
    /// Client order ID
    pub client_oid: String,

    /// Order ID  
    pub order_id: String,
}

impl RestClient {
    /// Amend an existing order on the Exchange
    ///
    /// This call is asynchronous, so the response is simply a confirmation of the request.
    /// The user.order subscription can be used to check when the order is successfully amended.
    ///
    /// Please note that amend order is designed as a convenience function such that it performs
    /// cancel and then create behind the scene. The new order will lose queue priority, except
    /// if the amend is only to amend down order quantity.
    ///
    /// [docs](https://exchange-docs.crypto.com/derivatives/index.html)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The amend order parameters
    ///
    /// # Returns
    /// Client order ID and order ID
    pub async fn amend_order(&self, request: AmendOrderRequest) -> RestResult<AmendOrderResponse> {
        self.send_signed_request(AMEND_ORDER_ENDPOINT, request)
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
    fn test_amend_order_request_by_order_id() {
        let request = AmendOrderRequest {
            order_id: Some("6530219466236720401".to_string()),
            orig_client_oid: None,
            new_price: "82000".to_string(),
            new_quantity: "0.0002".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("order_id").unwrap(), "6530219466236720401");
        assert_eq!(serialized.get("new_price").unwrap(), "82000");
        assert_eq!(serialized.get("new_quantity").unwrap(), "0.0002");
        assert!(
            !serialized
                .as_object()
                .unwrap()
                .contains_key("orig_client_oid")
        );
    }

    #[test]
    fn test_amend_order_request_by_client_oid() {
        let request = AmendOrderRequest {
            order_id: None,
            orig_client_oid: Some("53".to_string()),
            new_price: "83000".to_string(),
            new_quantity: "0.0001".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("orig_client_oid").unwrap(), "53");
        assert_eq!(serialized.get("new_price").unwrap(), "83000");
        assert_eq!(serialized.get("new_quantity").unwrap(), "0.0001");
        assert!(!serialized.as_object().unwrap().contains_key("order_id"));
    }

    #[test]
    fn test_amend_order_request_both_identifiers() {
        let request = AmendOrderRequest {
            order_id: Some("6530219466236720401".to_string()),
            orig_client_oid: Some("53".to_string()),
            new_price: "84000".to_string(),
            new_quantity: "0.0003".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("order_id").unwrap(), "6530219466236720401");
        assert_eq!(serialized.get("orig_client_oid").unwrap(), "53");
        assert_eq!(serialized.get("new_price").unwrap(), "84000");
        assert_eq!(serialized.get("new_quantity").unwrap(), "0.0003");
    }

    #[test]
    fn test_amend_order_response_structure() {
        let response_json = json!({
            "client_oid": "55",
            "order_id": "6530219466236720401"
        });

        let response: AmendOrderResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.client_oid, "55");
        assert_eq!(response.order_id, "6530219466236720401");
    }
}
