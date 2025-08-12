use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

/// Endpoint path for the convert order status API.
const CONVERT_ORDER_STATUS_ENDPOINT: &str = "/fapi/v1/convert/orderStatus";

/// Request parameters for the convert order status endpoint.
///
/// Query order status by order ID or quote ID. Either `order_id` or `quote_id` is required.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetConvertOrderStatusRequest {
    /// Order ID for the conversion. Either `order_id` or `quote_id` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Quote ID for the conversion. Either `order_id` or `quote_id` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,

    /// Request timestamp in milliseconds since epoch.
    pub timestamp: u64,

    /// Optional receive window (milliseconds). If not set, the API default is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Status of a convert order.
///
/// See [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/convert/Order-Status)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConvertOrderStatus {
    /// Order is being processed.
    #[serde(rename = "PROCESS")]
    Process,

    /// Order quote has been accepted successfully.
    #[serde(rename = "ACCEPT_SUCCESS")]
    AcceptSuccess,

    /// Order has been completed successfully.
    #[serde(rename = "SUCCESS")]
    Success,

    /// Order has failed.
    #[serde(rename = "FAIL")]
    Fail,
}

/// Response for the convert order status endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertOrderStatusResponse {
    /// Order ID for the conversion.
    pub order_id: String,

    /// Status of the convert order.
    pub order_status: ConvertOrderStatus,

    /// Symbol of the asset being converted from.
    pub from_asset: String,

    /// Amount of the asset being converted from.
    pub from_amount: String,

    /// Symbol of the asset being converted to.
    pub to_asset: String,

    /// Amount of the asset being converted to.
    pub to_amount: String,

    /// Conversion ratio.
    pub ratio: String,

    /// Inverse conversion ratio.
    pub inverse_ratio: String,

    /// Creation time in milliseconds since epoch.
    pub create_time: u64,
}

impl UsdmClient {
    /// Order status
    ///
    /// Query order status by order ID or quote ID.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/convert/Order-Status)
    ///
    /// Rate limit: 50 (IP)
    ///
    /// # Arguments
    /// * `params` - The convert order status request parameters
    ///
    /// # Returns
    /// `ConvertOrderStatusResponse` - Order status and details
    pub async fn get_convert_order_status(
        &self,
        params: GetConvertOrderStatusRequest,
    ) -> RestResult<ConvertOrderStatusResponse> {
        self.send_get_signed_request(CONVERT_ORDER_STATUS_ENDPOINT, params, 50, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_order_status_response_deserialization() {
        let json = r#"{
            "orderId": "933256278426274426",
            "orderStatus": "SUCCESS",
            "fromAsset": "USDT",
            "fromAmount": "20",
            "toAsset": "BNB",
            "toAmount": "0.06154036",
            "ratio": "0.00307702",
            "inverseRatio": "324.99",
            "createTime": 1624248872184
        }"#;

        let response: ConvertOrderStatusResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, "933256278426274426");
        assert_eq!(response.from_asset, "USDT");
        assert_eq!(response.to_asset, "BNB");
        assert_eq!(response.order_status, ConvertOrderStatus::Success);
        assert_eq!(response.create_time, 1624248872184);
    }

    #[test]
    fn test_get_convert_order_status_request_serialization() {
        let request = GetConvertOrderStatusRequest {
            order_id: Some("933256278426274426".to_string()),
            quote_id: None,
            timestamp: 1625097600000,
            recv_window: Some(5000),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("orderId=933256278426274426"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("recvWindow=5000"));
    }
}
