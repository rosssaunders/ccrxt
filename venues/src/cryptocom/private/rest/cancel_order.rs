use super::client::RestClient;
use crate::cryptocom::RestResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Request parameters for canceling an order
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderRequest {
    /// Order ID (string format is highly recommended)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Client Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
}

/// Response for canceling an order
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct CancelOrderResponse {
    /// Order ID
    pub order_id: String,
    /// Client Order ID
    pub client_oid: String,
}

impl RestClient {
    /// Cancels an existing order on the Exchange (asynchronous)
    ///
    /// This call is asynchronous, so the response is simply a confirmation of the request.
    /// The user.order subscription can be used to check when the order is successfully canceled.
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The cancel order parameters
    ///
    /// # Returns
    /// Order ID and client order ID
    pub async fn cancel_order(&self, request: CancelOrderRequest) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;
        let params = serde_json::to_value(&request)
            .map_err(|e| crate::cryptocom::Errors::Error(format!("Serialization error: {}", e)))?;

        self.send_signed_request("private/cancel-order", params).await
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
    fn test_cancel_order_request_by_order_id() {
        let request = CancelOrderRequest {
            order_id: Some("18342311".to_string()),
            client_oid: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("order_id").unwrap(), "18342311");
        assert!(!serialized.as_object().unwrap().contains_key("client_oid"));
    }

    #[test]
    fn test_cancel_order_request_by_client_oid() {
        let request = CancelOrderRequest {
            order_id: None,
            client_oid: Some("c5f682ed-7108-4f1c-b755-972fcdca0f02".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("client_oid").unwrap(),
            "c5f682ed-7108-4f1c-b755-972fcdca0f02"
        );
        assert!(!serialized.as_object().unwrap().contains_key("order_id"));
    }

    #[test]
    fn test_cancel_order_request_both_identifiers() {
        let request = CancelOrderRequest {
            order_id: Some("18342311".to_string()),
            client_oid: Some("c5f682ed-7108-4f1c-b755-972fcdca0f02".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("order_id").unwrap(), "18342311");
        assert_eq!(
            serialized.get("client_oid").unwrap(),
            "c5f682ed-7108-4f1c-b755-972fcdca0f02"
        );
    }

    #[test]
    fn test_cancel_order_response_structure() {
        let response_json = json!({
            "order_id": "18342311",
            "client_oid": "c5f682ed-7108-4f1c-b755-972fcdca0f02"
        });

        let response: CancelOrderResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.order_id, "18342311");
        assert_eq!(response.client_oid, "c5f682ed-7108-4f1c-b755-972fcdca0f02");
    }
}
