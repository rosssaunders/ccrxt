use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::ConvertOrderStatus;
use crate::binance::usdm::RestResult;

const ACCEPT_CONVERT_QUOTE_ENDPOINT: &str = "/fapi/v1/convert/acceptQuote";

/// Request parameters for accepting convert quote.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AcceptConvertQuoteRequest {
    /// Quote ID from quote request.
    pub quote_id: String,

    /// Request timestamp in milliseconds since epoch.
    pub timestamp: u64,

    /// Optional receive window (milliseconds). If not set, default is used by API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response from accept convert quote endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptConvertQuoteResponse {
    /// Order ID for the conversion.
    pub order_id: String,

    /// Create time in milliseconds since epoch.
    pub create_time: u64,

    /// Order status for the conversion.
    pub order_status: ConvertOrderStatus,
}

impl UsdmClient {
    /// Accept the offered quote
    ///
    /// Accept the offered quote by quote ID.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/convert/Accept-Quote
    ///
    /// Rate limit: 200
    ///
    /// # Arguments
    /// * `params` - The accept convert quote request parameters
    ///
    /// # Returns
    /// AcceptConvertQuoteResponse - Order information for the accepted quote
    pub async fn accept_convert_quote(
        &self,
        params: AcceptConvertQuoteRequest,
    ) -> RestResult<AcceptConvertQuoteResponse> {
        self.send_signed_request(
            ACCEPT_CONVERT_QUOTE_ENDPOINT,
            Method::POST,
            params,
            200,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accept_convert_quote_request_serialization() {
        let request = AcceptConvertQuoteRequest {
            quote_id: "12415572564".to_string(),
            timestamp: 1625097600000,
            recv_window: Some(5000),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("quoteId=12415572564"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("recvWindow=5000"));
    }
}
