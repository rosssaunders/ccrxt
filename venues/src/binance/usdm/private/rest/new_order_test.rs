use super::UsdmClient;
use super::new_order::NewOrderRequest;
use crate::binance::usdm::RestResult;
use serde::Deserialize;

const TEST_ORDER_ENDPOINT: &str = "/fapi/v1/order/test";

/// Response for a test order (usually empty JSON object {}).
#[derive(Debug, Clone, Deserialize)]
pub struct TestOrderResponse {
    // Binance returns an empty object for successful test orders
}

impl UsdmClient {
    /// New Order Test
    ///
    /// Test endpoint for placing a new order (does not create an order).
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/New-Order-Test
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `request` - The new order request parameters
    ///
    /// # Returns
    /// Empty response if successful
    pub async fn new_order_test(&self, request: NewOrderRequest) -> RestResult<TestOrderResponse> {
        self.send_signed_request(
            TEST_ORDER_ENDPOINT,
            reqwest::Method::POST,
            request,
            1,
            false,
        )
        .await
    }
}
