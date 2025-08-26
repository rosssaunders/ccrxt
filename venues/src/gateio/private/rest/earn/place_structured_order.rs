use super::{RestClient, RestResult};
use serde::Serialize;

const PLACE_STRUCTURED_ORDER_ENDPOINT: &str = "/earn/structured/orders";

/// Request parameters for placing a Structured Product order.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaceStructuredOrderRequest {
    /// Product ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,

    /// Buy Quantity. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

impl RestClient {
    /// Place Structured Product Order endpoint
    ///
    /// Places a structured product order.
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#place-structured-product-order)
    ///
    /// Rate limit: per venue config
    ///
    /// # Arguments
    /// * `request` - The order placement request parameters
    ///
    /// # Returns
    /// Empty response - success indicated by HTTP status
    pub async fn place_structured_order(
        &self,
        request: PlaceStructuredOrderRequest,
    ) -> RestResult<()> {
        self.send_post_request::<(), _>(PLACE_STRUCTURED_ORDER_ENDPOINT, Some(&request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_structured_order_request_serialization() {
        let req = PlaceStructuredOrderRequest {
            pid: Some("1".to_string()),
            amount: Some("0.5".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("pid"));
        assert!(json.contains("amount"));
    }
}
