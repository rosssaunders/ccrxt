use serde::Serialize;

use crate::bullish::{
    DataOrPaginated, EndpointType, PaginatedResult, PaginationParams,
    PrivateRestClient as RestClient, RestResult,
    enums::{OrderSide, OrderStatus},
};

/// Endpoint URL for historical orders operations
const ORDERS_HISTORY_ENDPOINT: &str = "/v2/history/orders";

/// Parameters for querying historical orders
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetOrdersHistoryParams {
    /// Trading account ID (required)
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,

    /// Market symbol filter (e.g., "BTCUSDC" or "BTC-USDC-PERP"). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Order ID filter. Optional.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Client order ID filter. Optional.
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Order side filter (BUY/SELL). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,

    /// Order status filter. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,

    /// Start ISO8601 datetime (with milliseconds) filter for order creation time. Optional.
    #[serde(
        rename = "createdAtDatetime[gte]",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_datetime_gte: Option<String>,

    /// End ISO8601 datetime (with milliseconds) filter for order creation time. Optional.
    #[serde(
        rename = "createdAtDatetime[lte]",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_datetime_lte: Option<String>,

    /// Start timestamp (ms since epoch) filter for order creation time. Optional.
    #[serde(
        rename = "createdAtTimestamp[gte]",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_timestamp_gte: Option<u64>,

    /// End timestamp (ms since epoch) filter for order creation time. Optional.
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
    /// Get Historical Orders (v2)
    ///
    /// Retrieve a list of historical orders placed by a trading account with specified filters.
    /// Only the last 90 days of data is available for querying. Supports cursor pagination.
    ///
    /// [docs](https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v2/history/orders)
    ///
    /// # Arguments
    /// * `params` - Query parameters for filtering historical orders
    ///
    /// # Returns
    /// List (possibly paginated) of orders matching the filter criteria
    pub async fn get_orders_history(
        &mut self,
        params: GetOrdersHistoryParams,
    ) -> RestResult<PaginatedResult<super::types::Order>> {
        let wire: DataOrPaginated<super::types::Order> = self
            .send_get_authenticated_request(
                ORDERS_HISTORY_ENDPOINT,
                params,
                EndpointType::PrivateOrders,
            )
            .await?;

        Ok(PaginatedResult::from(wire))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_orders_history_params_query_serialization() {
        let params = GetOrdersHistoryParams {
            trading_account_id: "acc-xyz".to_string(),
            symbol: Some("BTCUSDC".to_string()),
            order_id: Some("297735387747975680".to_string()),
            client_order_id: Some("187".to_string()),
            side: Some(OrderSide::Buy),
            status: Some(OrderStatus::Open),
            created_at_datetime_gte: Some("2025-05-20T01:01:01.000Z".to_string()),
            created_at_datetime_lte: Some("2025-05-21T01:01:01.000Z".to_string()),
            created_at_timestamp_gte: Some(1700000000000),
            created_at_timestamp_lte: Some(1700000100000),
            pagination: PaginationParams {
                page_size: Some(10),
                meta_data: Some(true),
                next_page: Some("cursorNext".to_string()),
                previous_page: None,
            },
        };

        let qs = serde_urlencoded::to_string(&params).unwrap();
        assert!(qs.contains("tradingAccountId=acc-xyz"));
        assert!(qs.contains("symbol=BTCUSDC"));
        assert!(qs.contains("orderId=297735387747975680"));
        assert!(qs.contains("clientOrderId=187"));
        assert!(qs.contains("side=BUY"));
        assert!(qs.contains("status="));
        assert!(qs.contains("createdAtDatetime%5Bgte%5D=2025-05-20T01%3A01%3A01.000Z"));
        assert!(qs.contains("createdAtDatetime%5Blte%5D=2025-05-21T01%3A01%3A01.000Z"));
        assert!(qs.contains("createdAtTimestamp%5Bgte%5D=1700000000000"));
        assert!(qs.contains("createdAtTimestamp%5Blte%5D=1700000100000"));
        assert!(qs.contains("_pageSize=10"));
        assert!(qs.contains("_metaData=true"));
        assert!(qs.contains("_nextPage=cursorNext"));
    }
}
