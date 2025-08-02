use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::UsdmClient;
use crate::binance::usdm::RestResult;

/// Endpoint path for the convert get quote API.
const CONVERT_GET_QUOTE_ENDPOINT: &str = "/fapi/v1/convert/getQuote";

/// Request parameters for the Send Quote Request
///
/// Either `from_amount` or `to_amount` must be provided, but not both.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetConvertQuoteRequest {
    /// From asset symbol (e.g., "BTC").
    pub from_asset: Cow<'static, str>,

    /// To asset symbol (e.g., "USDT").
    pub to_asset: Cow<'static, str>,

    /// From amount (required if `to_amount` is not provided). The amount you will be debited after the conversion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_amount: Option<Cow<'static, str>>,

    /// To amount (required if `from_amount` is not provided). The amount you will be credited after the conversion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_amount: Option<Cow<'static, str>>,

    /// Valid period for this quote (e.g., "10s"). Optional. Default is 10s.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_time: Option<Cow<'static, str>>,

    /// Request timestamp in milliseconds since epoch.
    pub timestamp: u64,

    /// Optional receive window (milliseconds). Cannot be greater than 60000. If not set, API default is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response from the Send Quote Request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertQuoteResponse {
    /// Quote ID for the conversion.
    pub quote_id: Cow<'static, str>,

    /// Quote ratio for conversion.
    pub ratio: Cow<'static, str>,

    /// Inverse ratio for conversion.
    pub inverse_ratio: Cow<'static, str>,

    /// Valid timestamp in milliseconds when quote expires.
    pub valid_timestamp: u64,

    /// To amount for the conversion.
    pub to_amount: Cow<'static, str>,

    /// From amount for the conversion.
    pub from_amount: Cow<'static, str>,
}

impl UsdmClient {
    /// Send Quote Request
    ///
    /// Request a quote for the requested token pairs.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/convert/Send-quote-request
    ///
    /// Rate limit: 50 (IP)
    ///
    /// # Arguments
    /// * `params` - The convert quote request parameters
    ///
    /// # Returns
    /// `ConvertQuoteResponse` - Quote information for the conversion
    pub async fn get_convert_quote(
        &self,
        params: GetConvertQuoteRequest,
    ) -> RestResult<ConvertQuoteResponse> {
        self.send_post_signed_request(
            CONVERT_GET_QUOTE_ENDPOINT,
            params, 50, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_convert_quote_response_deserialization() {
        let json = r#"{
            "quoteId": "12415572564",
            "ratio": "95.65",
            "inverseRatio": "0.01045",
            "validTimestamp": 1625097600000,
            "toAmount": "95.65",
            "fromAmount": "1"
        }"#;

        let response: ConvertQuoteResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.quote_id, "12415572564");
        assert_eq!(response.ratio, "95.65");
        assert_eq!(response.inverse_ratio, "0.01045");
        assert_eq!(response.valid_timestamp, 1625097600000);
        assert_eq!(response.to_amount, "95.65");
        assert_eq!(response.from_amount, "1");
    }

    #[test]
    fn test_get_convert_quote_request_serialization() {
        let request = GetConvertQuoteRequest {
            from_asset: Cow::Borrowed("BTC"),
            to_asset: Cow::Borrowed("USDT"),
            from_amount: Some(Cow::Borrowed("0.1")),
            to_amount: None,
            valid_time: Some(Cow::Borrowed("10s")),
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

    #[test]
    fn test_get_convert_quote_request_either_amount() {
        // Only to_amount provided
        let request = GetConvertQuoteRequest {
            from_asset: Cow::Borrowed("BTC"),
            to_asset: Cow::Borrowed("USDT"),
            from_amount: None,
            to_amount: Some(Cow::Borrowed("100")),
            valid_time: None,
            timestamp: 1625097600000,
            recv_window: None,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("toAmount=100"));
        assert!(!serialized.contains("fromAmount="));
    }

    #[test]
    fn test_get_convert_quote_request_both_amounts_none() {
        // Both from_amount and to_amount are None (should be allowed by struct, but API will reject)
        let request = GetConvertQuoteRequest {
            from_asset: Cow::Borrowed("BTC"),
            to_asset: Cow::Borrowed("USDT"),
            from_amount: None,
            to_amount: None,
            valid_time: None,
            timestamp: 1625097600000,
            recv_window: None,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        // Neither fromAmount nor toAmount present
        assert!(!serialized.contains("fromAmount="));
        assert!(!serialized.contains("toAmount="));
    }
}
