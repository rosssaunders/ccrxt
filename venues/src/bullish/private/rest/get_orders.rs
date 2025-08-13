use serde::{Deserialize, Serialize};

use crate::bullish::{
    DataOrPaginated, EndpointType, PaginatedResult, PaginationParams, RestResult, enums::OrderSide,
    private::rest::client::RestClient,
};

/// Endpoint URL for orders operations
const ORDERS_ENDPOINT: &str = "/v2/orders";

/// Status filter for Get Orders endpoint
///
/// Endpoint-specific enumerated values supported by Bullish filtering:
/// - OPEN | CLOSED | CANCELLED | REJECTED
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderStatusFilter {
    Open,

    Closed,

    Cancelled,

    Rejected,
}

/// Parameters for querying orders
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetOrdersParams {
    /// Trading account ID (required)
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,

    /// Market symbol filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Client order ID filter
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Order side filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,

    /// Order status filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatusFilter>,

    /// Start ISO8601 datetime (with milliseconds) filter for order creation time.
    #[serde(
        rename = "createdAtDatetime[gte]",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_datetime_gte: Option<String>,

    /// End ISO8601 datetime (with milliseconds) filter for order creation time.
    #[serde(
        rename = "createdAtDatetime[lte]",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_datetime_lte: Option<String>,

    /// Start timestamp (ms since epoch) filter for order creation time.
    #[serde(
        rename = "createdAtTimestamp[gte]",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_timestamp_gte: Option<u64>,

    /// End timestamp (ms since epoch) filter for order creation time.
    #[serde(
        rename = "createdAtTimestamp[lte]",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_timestamp_lte: Option<u64>,

    /// Pagination parameters (flattened into top-level query)
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

impl RestClient {
    /// Get orders with optional filters
    ///
    /// Retrieve a list of orders placed by a trading account with specified filters.
    /// Only the last 24 hours of data is available for querying.
    ///
    /// [docs](https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v2/orders)
    ///
    /// # Arguments
    /// * `params` - Query parameters for filtering orders
    ///
    /// # Returns
    /// List of orders matching the filter criteria
    pub async fn get_orders(
        &mut self,
        params: GetOrdersParams,
    ) -> RestResult<PaginatedResult<super::types::Order>> {
        let wire: DataOrPaginated<super::types::Order> = self
            .send_get_request(ORDERS_ENDPOINT, params, EndpointType::PrivateOrders)
            .await?;

        Ok(PaginatedResult::from(wire))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_orders_params_query_serialization() {
        let params = GetOrdersParams {
            trading_account_id: "acc-123".to_string(),
            symbol: Some("BTCUSD".to_string()),
            client_order_id: Some("cli-1".to_string()),
            side: Some(OrderSide::Buy),
            status: Some(OrderStatusFilter::Closed),
            created_at_datetime_gte: Some("2025-05-20T00:00:00.000Z".to_string()),
            created_at_datetime_lte: Some("2025-05-21T00:00:00.000Z".to_string()),
            created_at_timestamp_gte: Some(1700000000000),
            created_at_timestamp_lte: Some(1700000100000),
            pagination: PaginationParams {
                page_size: Some(50),
                meta_data: Some(true),
                next_page: Some("cursorNext".to_string()),
                previous_page: None,
            },
        };

        let qs = serde_urlencoded::to_string(&params).unwrap();
        assert!(qs.contains("tradingAccountId=acc-123"));
        assert!(qs.contains("symbol=BTCUSD"));
        assert!(qs.contains("clientOrderId=cli-1"));
        assert!(qs.contains("side=BUY"));
        assert!(qs.contains("status=CLOSED"));
        assert!(qs.contains("createdAtDatetime%5Bgte%5D=2025-05-20T00%3A00%3A00.000Z"));
        assert!(qs.contains("createdAtDatetime%5Blte%5D=2025-05-21T00%3A00%3A00.000Z"));
        assert!(qs.contains("createdAtTimestamp%5Bgte%5D=1700000000000"));
        assert!(qs.contains("createdAtTimestamp%5Blte%5D=1700000100000"));
        assert!(qs.contains("_pageSize=50"));
        assert!(qs.contains("_metaData=true"));
        assert!(qs.contains("_nextPage=cursorNext"));
    }
}
