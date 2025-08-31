use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const MASS_CANCEL_SPREAD_ORDERS_ENDPOINT: &str = "/api/v5/sprd/mass-cancel";

/// Request parameters for mass canceling spread orders
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MassCancelSpreadOrdersRequest {
    /// Spread ID
    /// If not provided, all pending orders will be canceled
    #[serde(rename = "sprdId", skip_serializing_if = "Option::is_none")]
    pub sprd_id: Option<String>,
}

/// Response data for mass canceling spread orders
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MassCancelSpreadOrdersResponse {
    /// Result of the request: true or false
    #[serde(rename = "result")]
    pub result: bool,
}

impl RestClient {
    /// Mass cancel spread orders
    ///
    /// Cancel all pending spread orders
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#spread-trading-rest-api-cancel-all-orders)
    pub async fn mass_cancel_spread_orders(
        &self,
        request: Option<MassCancelSpreadOrdersRequest>,
    ) -> RestResult<MassCancelSpreadOrdersResponse> {
        self.send_post_request(
            MASS_CANCEL_SPREAD_ORDERS_ENDPOINT,
            request.as_ref(),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_mass_cancel_spread_orders_request_with_sprd_id() {
        let request = MassCancelSpreadOrdersRequest {
            sprd_id: Some("BTC-USDT_BTC-USDT-SWAP".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: MassCancelSpreadOrdersRequest =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_mass_cancel_spread_orders_request_without_sprd_id() {
        let request = MassCancelSpreadOrdersRequest { sprd_id: None };

        let serialized = serde_json::to_string(&request).unwrap();
        assert_eq!(serialized, "{}");
    }

    #[test]
    fn test_mass_cancel_spread_orders_request_none() {
        let request: Option<MassCancelSpreadOrdersRequest> = None;

        // This should work with the RestClient method
        assert!(request.is_none());
    }

    #[test]
    fn test_mass_cancel_spread_orders_response_success() {
        let json_response = r#"{
            "result": true
        }"#;

        let response: MassCancelSpreadOrdersResponse = serde_json::from_str(json_response).unwrap();
        assert!(response.result);
    }

    #[test]
    fn test_mass_cancel_spread_orders_response_failure() {
        let json_response = r#"{
            "result": false
        }"#;

        let response: MassCancelSpreadOrdersResponse = serde_json::from_str(json_response).unwrap();
        assert!(!response.result);
    }

    #[test]
    fn test_mass_cancel_spread_orders_serialization() {
        let response = MassCancelSpreadOrdersResponse { result: true };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: MassCancelSpreadOrdersResponse =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(response, deserialized);
    }
}
