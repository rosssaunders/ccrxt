use serde::{Deserialize, Serialize};

use crate::binance::usdm::PrivateRestClient as RestClient;
use super::new_order::NewOrderRequest;
use crate::binance::usdm::RestResult;

/// Endpoint path for Binance USDM Test Order
const TEST_ORDER_ENDPOINT: &str = "/fapi/v1/order/test";

/// Response for a test order (usually empty JSON object {}).
///
/// This struct is returned by the Binance USDM Test Order endpoint.
/// The response is always an empty object `{}` if successful.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TestOrderResponse {}

impl RestClient {
    /// New Order Test
    ///
    /// Test endpoint for placing a new order (does not create an order).
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/New-Order-Test)
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `request` - The new order request parameters
    ///
    /// # Returns
    /// Empty response if successful
    pub async fn new_order_test(&self, request: NewOrderRequest) -> RestResult<TestOrderResponse> {
        self.send_post_signed_request(TEST_ORDER_ENDPOINT, request, 1, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_empty_object() {
        let data = "{}";
        let resp: TestOrderResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp, TestOrderResponse::default());
    }

    #[test]
    fn test_serialize_empty_object() {
        let resp = TestOrderResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        assert_eq!(json, "{}");
    }
}
