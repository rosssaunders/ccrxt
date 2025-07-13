use serde::Serialize;

use super::RestClient;

const DELIVERY_MY_TRADES_ENDPOINT: &str = "/delivery/{}/my_trades";

/// Request parameters for delivery my trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryMyTradesRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Order ID filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,

    /// Maximum number of records (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// List offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Specify starting point
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,

    /// Count only (returns count instead of trades)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_total: Option<i32>,
}

/// Delivery trade information (reusing the public DeliveryTrade from public module)
pub type DeliveryTrade = crate::gateio::delivery::public::rest::trades::DeliveryTrade;

impl RestClient {
    /// List personal delivery trading history
    ///
    /// Retrieves the user's trading history for delivery contracts.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The delivery trades request parameters
    ///
    /// # Returns
    /// List of delivery trades
    pub async fn get_delivery_my_trades(
        &self,
        params: DeliveryMyTradesRequest,
    ) -> crate::gateio::delivery::Result<Vec<DeliveryTrade>> {
        let endpoint = DELIVERY_MY_TRADES_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, &params).await
    }
}
