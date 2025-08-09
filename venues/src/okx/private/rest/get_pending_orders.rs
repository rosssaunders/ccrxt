use serde::Serialize;

use super::{RestClient, get_order::OrderDetails};
use crate::okx::{EndpointType, InstrumentType, RestResult};

const TRADE_ORDERS_PENDING_ENDPOINT: &str = "api/v5/trade/orders-pending";

/// Request to get pending orders
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPendingOrdersRequest {
    /// Instrument type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,

    /// Underlying
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,

    /// Instrument family
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,

    /// Instrument ID, e.g. "BTC-USDT"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,

    /// Order type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_type: Option<String>,

    /// Order state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Pagination of data to return records earlier than the requested ordId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records newer than the requested ordId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Number of results per request. The maximum is 100; the default is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl RestClient {
    /// Get pending orders
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#order-book-trading-trade-get-order-list
    ///
    /// # Arguments
    /// * `request` - The get pending orders request
    ///
    /// # Returns
    /// A result containing the pending orders or an error
    pub async fn get_pending_orders(
        &self,
        request: &GetPendingOrdersRequest,
    ) -> RestResult<OrderDetails> {
        self.send_get_request(
            TRADE_ORDERS_PENDING_ENDPOINT,
            Some(request),
            EndpointType::PrivateTrading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pending_orders_request_serialization() {
        let request = GetPendingOrdersRequest {
            inst_type: Some(InstrumentType::Spot),
            uly: None,
            inst_family: None,
            inst_id: Some("BTC-USDT".to_string()),
            ord_type: None,
            state: None,
            after: None,
            before: None,
            limit: Some("50".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instType=SPOT"));
        assert!(serialized.contains("instId=BTC-USDT"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_get_pending_orders_minimal_request() {
        let request = GetPendingOrdersRequest {
            inst_type: None,
            uly: None,
            inst_family: None,
            inst_id: None,
            ord_type: None,
            state: None,
            after: None,
            before: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        // Should serialize to empty string or just empty params
        assert_eq!(serialized, "");
    }
}
