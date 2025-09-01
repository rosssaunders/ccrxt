use serde::Serialize;

use crate::bullish::{EndpointType, PrivateRestClient as RestClient, RestResult};

/// Endpoint URL for orders operations
const ORDERS_ENDPOINT: &str = "/v2/orders";

/// Request parameters for retrieving a specific order.
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderRequest {
    /// The order ID to retrieve (path parameter)
    #[serde(skip_serializing)]
    pub order_id: String,

    /// Trading account ID used to scope the order lookup (query parameter)
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,
}

impl RestClient {
    /// Get specific order by ID
    ///
    /// Retrieve details for a specific order by its order ID.
    ///
    /// [docs](https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v2/orders/-orderId-)
    ///
    /// Rate limit: per private trading endpoints
    ///
    /// # Arguments
    /// * `request` - Contains order ID and trading account ID
    ///
    /// # Returns
    /// Order details
    pub async fn get_order(&mut self, request: GetOrderRequest) -> RestResult<super::types::Order> {
        let endpoint = format!("{}/{}", ORDERS_ENDPOINT, request.order_id);

        // Only serialize query parameters (trading_account_id)
        #[derive(Serialize)]
        struct QueryParams<'a> {
            #[serde(rename = "tradingAccountId")]
            trading_account_id: &'a str,
        }
        let params = QueryParams {
            trading_account_id: &request.trading_account_id,
        };

        self.send_get_request(&endpoint, params, EndpointType::PrivateOrders)
            .await
    }
}
