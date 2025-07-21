use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const CONVERT_GET_QUOTE_ENDPOINT: &str = "/fapi/v1/convert/getQuote";

/// Request parameters for getting convert quote.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetConvertQuoteRequest {
    /// From asset symbol.
    pub from_asset: String,

    /// To asset symbol.
    pub to_asset: String,

    /// From amount (required if toAmount is not provided).
    /// When specified, it is the amount you will be debited after the conversion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_amount: Option<String>,

    /// To amount (required if fromAmount is not provided).
    /// When specified, it is the amount you will be credited after the conversion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_amount: Option<String>,

    /// Valid period in seconds for this quote (10s, default 10s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_time: Option<String>,

    /// Request timestamp in milliseconds since epoch.
    pub timestamp: u64,

    /// Optional receive window (milliseconds). If not set, default is used by API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response from convert quote endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertQuoteResponse {
    /// Quote ID for the conversion.
    pub quote_id: String,

    /// Quote ratio for conversion.
    pub ratio: String,

    /// Inverse ratio for conversion.
    pub inverse_ratio: String,

    /// Valid timestamp in milliseconds when quote expires.
    pub valid_timestamp: u64,

    /// To amount for the conversion.
    pub to_amount: String,

    /// From amount for the conversion.
    pub from_amount: String,
}

impl UsdmClient {
    /// Send Quote Request
    ///
    /// Request a quote for the requested token pairs.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/convert/Send-quote-request
    ///
    /// Rate limit: 50
    ///
    /// # Arguments
    /// * `params` - The convert quote request parameters
    ///
    /// # Returns
    /// ConvertQuoteResponse - Quote information for the conversion
    pub async fn get_convert_quote(
        &self,
        params: GetConvertQuoteRequest,
    ) -> RestResult<ConvertQuoteResponse> {
        self.send_signed_request(CONVERT_GET_QUOTE_ENDPOINT, Method::POST, params, 50, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_quote_response_deserialization() {
        let json = r#"
        {
            "quoteId": "12415572564",
            "ratio": "95.65",
            "inverseRatio": "0.01045",
            "validTimestamp": 1625097600000,
            "toAmount": "95.65",
            "fromAmount": "1"
        }
        "#;

        let response: ConvertQuoteResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.quote_id, "12415572564");
        assert_eq!(response.ratio, "95.65");
        assert_eq!(response.valid_timestamp, 1625097600000);
    }

    #[test]
    fn test_get_convert_quote_request_serialization() {
        let request = GetConvertQuoteRequest {
            from_asset: "BTC".to_string(),
            to_asset: "USDT".to_string(),
            from_amount: Some("0.1".to_string()),
            to_amount: None,
            valid_time: Some("10s".to_string()),
            timestamp: 1625097600000,
            recv_window: Some(5000),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("fromAsset=BTC"));
        assert!(serialized.contains("toAsset=USDT"));
        assert!(serialized.contains("fromAmount=0.1"));
        assert!(serialized.contains("validTime=10s"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("recvWindow=5000"));
    }
}
