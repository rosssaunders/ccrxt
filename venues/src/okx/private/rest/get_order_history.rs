use serde::Serialize;

use super::{RestClient, common::OkxApiResponse, get_order::OrderDetails};
use crate::okx::{EndpointType, InstrumentType, RestResult};

/// Request to get order history
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderHistoryRequest {
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

    /// Category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// Pagination of data to return records earlier than the requested ordId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records newer than the requested ordId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Number of results per request. The maximum is 100; the default is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,

    /// Filter with a begin timestamp. Unix timestamp format in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,

    /// Filter with an end timestamp. Unix timestamp format in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

impl RestClient {
    /// Get order history
    ///
    /// # Arguments
    /// * `request` - The get order history request
    ///
    /// # Returns
    /// A result containing the order history or an error
    pub async fn get_order_history(&self, request: &GetOrderHistoryRequest) -> RestResult<OkxApiResponse<OrderDetails>> {
        self.send_request(
            "api/v5/trade/orders-history",
            reqwest::Method::GET,
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
    fn test_get_order_history_request_serialization() {
        let request = GetOrderHistoryRequest {
            inst_type: Some(InstrumentType::Spot),
            uly: None,
            inst_family: None,
            inst_id: Some("BTC-USDT".to_string()),
            ord_type: None,
            state: Some("filled".to_string()),
            category: None,
            after: None,
            before: None,
            limit: Some("20".to_string()),
            begin: Some("1597026383085".to_string()),
            end: Some("1597026483085".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instType=SPOT"));
        assert!(serialized.contains("instId=BTC-USDT"));
        assert!(serialized.contains("state=filled"));
        assert!(serialized.contains("limit=20"));
        assert!(serialized.contains("begin=1597026383085"));
        assert!(serialized.contains("end=1597026483085"));
    }

    #[test]
    fn test_get_order_history_minimal_request() {
        let request = GetOrderHistoryRequest {
            inst_type: None,
            uly: None,
            inst_family: None,
            inst_id: None,
            ord_type: None,
            state: None,
            category: None,
            after: None,
            before: None,
            limit: None,
            begin: None,
            end: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        // Should serialize to empty string or just empty params
        assert_eq!(serialized, "");
    }
}
