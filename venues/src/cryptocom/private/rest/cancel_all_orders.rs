use serde::{Deserialize, Serialize};

use crate::cryptocom::{ApiResult, PrivateRestClient as RestClient, RestResult};

/// Endpoint path for the cancel-all-orders API
const CANCEL_ALL_ORDERS_ENDPOINT: &str = "exchange/v1/private/cancel-all-orders";

/// Order type filter for cancel all orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CancelOrderType {
    Limit,
    Trigger,
    All,
}

/// Request parameters for canceling all orders
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllOrdersRequest {
    /// Instrument name e.g. BTCUSD-PERP (if not provided, orders of ALL instruments will be canceled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,

    /// Order type filter: LIMIT, TRIGGER, ALL
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<CancelOrderType>,
}

/// Result data for canceling all orders
#[derive(Debug, Clone, Deserialize)]
pub struct CancelAllOrdersResult {
    /// Result code (0 for success)
    pub code: i32,
}

/// Response wrapper for endpoint
pub type CancelAllOrdersResponse = ApiResult<CancelAllOrdersResult>;

impl RestClient {
    /// Cancels all orders for a particular instrument/pair (asynchronous)
    ///
    /// This call is asynchronous, so the response is simply a confirmation of the request.
    /// The user.order subscription can be used to check when the orders are successfully canceled.
    ///
    /// [docs](https://exchange-docs.crypto.com/derivatives/index.html)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The cancel all orders parameters
    ///
    /// # Returns
    /// Success confirmation (code 0)
    pub async fn cancel_all_orders(
        &self,
        request: CancelAllOrdersRequest,
    ) -> RestResult<CancelAllOrdersResponse> {
        self.send_signed_request(CANCEL_ALL_ORDERS_ENDPOINT, request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use secrets::ExposableSecret;

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
    fn test_cancel_all_orders_request_with_instrument() {
        let request = CancelAllOrdersRequest {
            instrument_name: Some("BTCUSD-PERP".to_string()),
            order_type: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instrument_name").unwrap(), "BTCUSD-PERP");
        assert!(!serialized.as_object().unwrap().contains_key("type"));
    }

    #[test]
    fn test_cancel_all_orders_request_with_type_filter() {
        let request = CancelAllOrdersRequest {
            instrument_name: Some("BTCUSD-PERP".to_string()),
            order_type: Some(CancelOrderType::Limit),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instrument_name").unwrap(), "BTCUSD-PERP");
        assert_eq!(serialized.get("type").unwrap(), "LIMIT");
    }

    #[test]
    fn test_cancel_all_orders_request_all_instruments() {
        let request = CancelAllOrdersRequest {
            instrument_name: None,
            order_type: Some(CancelOrderType::All),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("type").unwrap(), "ALL");
        assert!(
            !serialized
                .as_object()
                .unwrap()
                .contains_key("instrument_name")
        );
    }

    #[test]
    fn test_cancel_all_orders_request_minimal() {
        let request = CancelAllOrdersRequest {
            instrument_name: None,
            order_type: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert!(
            !serialized
                .as_object()
                .unwrap()
                .contains_key("instrument_name")
        );
        assert!(!serialized.as_object().unwrap().contains_key("type"));
    }

    #[test]
    fn test_cancel_order_type_serialization() {
        assert_eq!(
            serde_json::to_value(CancelOrderType::Limit).unwrap(),
            "LIMIT"
        );
        assert_eq!(
            serde_json::to_value(CancelOrderType::Trigger).unwrap(),
            "TRIGGER"
        );
        assert_eq!(serde_json::to_value(CancelOrderType::All).unwrap(), "ALL");
    }
}
