use serde::Serialize;

use super::{RestClient, RestResult};

const PLACE_DUAL_ORDER_ENDPOINT: &str = "/earn/dual/orders";

/// Request parameters for placing a Dual Investment order.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PlaceDualOrderRequest {
    /// Product ID
    pub plan_id: String,

    /// Subscription amount, mutually exclusive with copies field
    pub amount: String,

    /// Order custom information. Users can set custom ID with this field.
    /// Must start with `t-`, excluding `t-` length cannot exceed 28 bytes, can only contain numbers, letters, underscore(_), hyphen(-) or dot(.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl RestClient {
    /// Place Dual Investment order endpoint
    ///
    /// Places a dual investment order.
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#place-dual-investment-order)
    ///
    /// Rate limit: per venue config
    ///
    /// # Arguments
    /// * `request` - The order placement request parameters
    ///
    /// # Returns
    /// Empty response - success indicated by HTTP status
    pub async fn place_dual_order(&self, request: PlaceDualOrderRequest) -> RestResult<()> {
        self.send_post_request::<(), _>(PLACE_DUAL_ORDER_ENDPOINT, Some(&request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_dual_order_request_serialization() {
        let req = PlaceDualOrderRequest {
            plan_id: "176".to_string(),
            amount: "1".to_string(),
            text: Some("t-custom-text".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("plan_id"));
        assert!(json.contains("amount"));
        assert!(json.contains("text"));
    }
}
