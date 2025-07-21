use std::borrow::Cow;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const ORDER_AMENDMENT_ENDPOINT: &str = "/fapi/v1/orderAmendment";

/// Request parameters for getting order amendment history.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrderAmendmentRequest {
    /// Trading symbol (e.g., "BTCUSDT"). Required.
    pub symbol: Cow<'static, str>,

    /// Order ID to get amendment history for (either this or origClientOrderId must be provided).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID (either this or orderId must be provided).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<Cow<'static, str>>,

    /// Start time for filtering amendments (milliseconds since epoch).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time for filtering amendments (milliseconds since epoch).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Limit the number of results (default 50, max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// The value cannot be greater than 60000 (milliseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Individual order amendment record.
///
/// Represents a single order modification entry from the API response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderAmendment {
    /// Amendment ID.
    pub amendment_id: u64,

    /// Trading symbol.
    pub symbol: Cow<'static, str>,

    /// Trading pair name.
    pub pair: Cow<'static, str>,

    /// Order ID.
    pub order_id: u64,

    /// Client order ID.
    pub client_order_id: Cow<'static, str>,

    /// Amendment timestamp (milliseconds since epoch).
    pub time: u64,

    /// Amendment details containing before and after values.
    pub amendment: AmendmentDetails,
}

/// Amendment details showing the before and after values for modified fields.
#[derive(Debug, Clone, Deserialize)]
pub struct AmendmentDetails {
    /// Price change information (if price was modified).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<AmendmentField>,

    /// Original quantity change information (if quantity was modified).
    #[serde(rename = "origQty", skip_serializing_if = "Option::is_none")]
    pub orig_qty: Option<AmendmentField>,

    /// Number of times the order has been modified.
    pub count: u32,
}

/// Represents a field change in order modification.
#[derive(Debug, Clone, Deserialize)]
pub struct AmendmentField {
    /// Value before the modification.
    pub before: Cow<'static, str>,

    /// Value after the modification.
    pub after: Cow<'static, str>,
}

/// Response type for order amendment history.
///
/// Based on the API documentation, the response is a direct array of order amendments.
pub type OrderAmendmentResponse = Vec<OrderAmendment>;

impl UsdmClient {
    /// Get Order Modify History (USER_DATA)
    ///
    /// Get order modification history for a specific order or symbol.
    /// Either orderId or origClientOrderId must be sent, and the orderId will prevail if both are sent.
    /// Order modify history longer than 3 months is not available.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Get-Order-Modify-History
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `request` - The order amendment history request parameters
    ///
    /// # Returns
    /// Array of order amendment records
    pub async fn get_order_amendment_history(
        &self,
        request: OrderAmendmentRequest,
    ) -> RestResult<OrderAmendmentResponse> {
        self.send_signed_request(ORDER_AMENDMENT_ENDPOINT, Method::GET, request, 1, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_amendment_response_deserialization() {
        let json = r#"[
            {
                "amendmentId": 5363,
                "symbol": "BTCUSDT",
                "pair": "BTCUSDT",
                "orderId": 20072994037,
                "clientOrderId": "LJ9R4QZDihCaS8UAOOLpgW",
                "time": 1629184560899,
                "amendment": {
                    "price": {
                        "before": "30004",
                        "after": "30003.2"
                    },
                    "origQty": {
                        "before": "1",
                        "after": "1"
                    },
                    "count": 3
                }
            },
            {
                "amendmentId": 5361,
                "symbol": "BTCUSDT",
                "pair": "BTCUSDT",
                "orderId": 20072994037,
                "clientOrderId": "LJ9R4QZDihCaS8UAOOLpgW",
                "time": 1629184533946,
                "amendment": {
                    "price": {
                        "before": "30005",
                        "after": "30004"
                    },
                    "origQty": {
                        "before": "1",
                        "after": "1"
                    },
                    "count": 2
                }
            }
        ]"#;

        let response: OrderAmendmentResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);

        let first_amendment = &response[0];
        assert_eq!(first_amendment.amendment_id, 5363);
        assert_eq!(first_amendment.symbol, "BTCUSDT");
        assert_eq!(first_amendment.pair, "BTCUSDT");
        assert_eq!(first_amendment.order_id, 20072994037);
        assert_eq!(first_amendment.client_order_id, "LJ9R4QZDihCaS8UAOOLpgW");
        assert_eq!(first_amendment.time, 1629184560899);
        assert_eq!(first_amendment.amendment.count, 3);

        let price_change = first_amendment.amendment.price.as_ref().unwrap();
        assert_eq!(price_change.before, "30004");
        assert_eq!(price_change.after, "30003.2");

        let qty_change = first_amendment.amendment.orig_qty.as_ref().unwrap();
        assert_eq!(qty_change.before, "1");
        assert_eq!(qty_change.after, "1");
    }

    #[test]
    fn test_amendment_field_deserialization() {
        let json = r#"{
            "before": "45000.0",
            "after": "45500.0"
        }"#;

        let field: AmendmentField = serde_json::from_str(json).unwrap();
        assert_eq!(field.before, "45000.0");
        assert_eq!(field.after, "45500.0");
    }

    #[test]
    fn test_amendment_details_with_only_price_change() {
        let json = r#"{
            "price": {
                "before": "50000.0",
                "after": "51000.0"
            },
            "count": 1
        }"#;

        let details: AmendmentDetails = serde_json::from_str(json).unwrap();
        assert_eq!(details.count, 1);
        assert!(details.price.is_some());
        assert!(details.orig_qty.is_none());

        let price_change = details.price.unwrap();
        assert_eq!(price_change.before, "50000.0");
        assert_eq!(price_change.after, "51000.0");
    }

    #[test]
    fn test_order_amendment_request_serialization() {
        let request = OrderAmendmentRequest {
            symbol: "BTCUSDT".into(),
            order_id: Some(12345),
            orig_client_order_id: None,
            start_time: Some(1629184560000),
            end_time: Some(1629184570000),
            limit: Some(50),
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("orderId=12345"));
        assert!(serialized.contains("startTime=1629184560000"));
        assert!(serialized.contains("endTime=1629184570000"));
        assert!(serialized.contains("limit=50"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_empty_amendment_response() {
        let json = "[]";
        let response: OrderAmendmentResponse = serde_json::from_str(json).unwrap();
        assert!(response.is_empty());
    }
}
