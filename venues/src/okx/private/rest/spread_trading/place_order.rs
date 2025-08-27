use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const PLACE_SPREAD_ORDER_ENDPOINT: &str = "/api/v5/sprd/order";

/// Request parameters for placing a spread order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaceSpreadOrderRequest {
    /// Spread ID
    #[serde(rename = "sprdId")]
    pub sprd_id: String,

    /// Client-supplied order ID
    #[serde(rename = "clOrdId", skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,

    /// Order side: buy, sell
    #[serde(rename = "side")]
    pub side: String,

    /// Order size
    #[serde(rename = "sz")]
    pub sz: String,

    /// Order price
    #[serde(rename = "px")]
    pub px: String,
}

/// Response data for placing a spread order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaceSpreadOrderResponse {
    /// Order ID
    #[serde(rename = "ordId")]
    pub ord_id: String,

    /// Client-supplied order ID
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: String,

    /// Spread ID
    #[serde(rename = "sprdId")]
    pub sprd_id: String,

    /// Error code
    #[serde(rename = "sCode")]
    pub s_code: String,

    /// Error message
    #[serde(rename = "sMsg")]
    pub s_msg: String,
}

impl RestClient {
    /// Place spread order
    ///
    /// Submit an order to a spread's order book
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#spread-trading-rest-api-place-order)
    pub async fn place_spread_order(
        &self,
        request: PlaceSpreadOrderRequest,
    ) -> RestResult<PlaceSpreadOrderResponse> {
        self.send_post_request(
            PLACE_SPREAD_ORDER_ENDPOINT,
            Some(&request),
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
    fn test_place_spread_order_request_serialization() {
        let request = PlaceSpreadOrderRequest {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            cl_ord_id: Some("client123".to_string()),
            side: "buy".to_string(),
            sz: "1".to_string(),
            px: "50000".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: PlaceSpreadOrderRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_place_spread_order_request_minimal() {
        let request = PlaceSpreadOrderRequest {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            cl_ord_id: None,
            side: "sell".to_string(),
            sz: "0.5".to_string(),
            px: "49000".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("sprdId"));
        assert!(serialized.contains("side"));
        assert!(serialized.contains("sz"));
        assert!(serialized.contains("px"));
        assert!(!serialized.contains("clOrdId"));
    }

    #[test]
    fn test_place_spread_order_response_deserialization() {
        let json_response = r#"{
            "ordId": "312269865356374016",
            "clOrdId": "client123",
            "sprdId": "BTC-USDT_BTC-USDT-SWAP",
            "sCode": "0",
            "sMsg": ""
        }"#;

        let response: PlaceSpreadOrderResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.ord_id, "312269865356374016");
        assert_eq!(response.cl_ord_id, "client123");
        assert_eq!(response.sprd_id, "BTC-USDT_BTC-USDT-SWAP");
        assert_eq!(response.s_code, "0");
        assert_eq!(response.s_msg, "");
    }

    #[test]
    fn test_place_spread_order_response_error() {
        let json_response = r#"{
            "ordId": "",
            "clOrdId": "client456",
            "sprdId": "BTC-USDT_BTC-USDT-SWAP",
            "sCode": "51000",
            "sMsg": "Invalid order size"
        }"#;

        let response: PlaceSpreadOrderResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.ord_id, "");
        assert_eq!(response.s_code, "51000");
        assert_eq!(response.s_msg, "Invalid order size");
    }
}
