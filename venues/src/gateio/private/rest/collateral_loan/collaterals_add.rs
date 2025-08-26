use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const COLLATERALS_ENDPOINT: &str = "/loan/collateral/collaterals";

/// Request to increase or redeem collateral for a loan order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CollateralOperationRequest {
    /// Order ID (required)
    pub order_id: i64,

    /// Collateral currency (required)
    pub collateral_currency: String,

    /// Collateral amount (required)
    pub collateral_amount: String,

    /// Operation type: "append" to add collateral, "redeem" to withdraw (required)
    pub r#type: String,
}

/// No content response for successful collateral operation.
#[derive(Debug, Clone, Deserialize)]
pub struct CollateralOperationResponse;

impl RestClient {
    /// Increase or redeem collateral for a loan order
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#increase-or-redeem-collateral)
    ///
    /// # Arguments
    /// * `request` - Parameters for collateral operation
    ///
    /// # Returns
    /// No content response on success
    pub async fn operate_collateral(
        &self,
        request: CollateralOperationRequest,
    ) -> RestResult<CollateralOperationResponse> {
        self.send_post_request(COLLATERALS_ENDPOINT, Some(&request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_serialize_append_request() {
        let req = CollateralOperationRequest {
            order_id: 123456,
            collateral_currency: "BTC".to_string(),
            collateral_amount: "1.0".to_string(),
            r#type: "append".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("append"));
        assert!(json.contains("collateral_currency"));
    }

    #[test]
    fn test_serialize_redeem_request() {
        let req = CollateralOperationRequest {
            order_id: 123456,
            collateral_currency: "BTC".to_string(),
            collateral_amount: "0.5".to_string(),
            r#type: "redeem".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("redeem"));
    }
}
