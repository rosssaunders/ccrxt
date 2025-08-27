use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for getting quotes
const GET_QUOTES_ENDPOINT: &str = "api/v5/rfq/quotes";

/// Request parameters for getting quotes
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetQuotesRequest {
    /// RFQ ID created by system
    #[serde(rename = "rfqId", skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,

    /// Client-supplied RFQ ID
    #[serde(rename = "clRfqId", skip_serializing_if = "Option::is_none")]
    pub cl_rfq_id: Option<String>,

    /// Quote ID created by system
    #[serde(rename = "quoteId", skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,

    /// Client-supplied Quote ID
    #[serde(rename = "clQuoteId", skip_serializing_if = "Option::is_none")]
    pub cl_quote_id: Option<String>,

    /// Quote state filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Begin time (Unix timestamp in milliseconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,

    /// End time (Unix timestamp in milliseconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    /// Number of results per request (max 100, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Quote information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteInfo {
    /// The timestamp the quote was created (Unix timestamp in milliseconds)
    #[serde(rename = "cTime")]
    pub c_time: String,

    /// The timestamp the quote was last updated (Unix timestamp in milliseconds)
    #[serde(rename = "uTime")]
    pub u_time: String,

    /// The status of the quote
    pub state: String,

    /// RFQ ID
    #[serde(rename = "rfqId")]
    pub rfq_id: String,

    /// Client-supplied RFQ ID
    #[serde(rename = "clRfqId")]
    pub cl_rfq_id: String,

    /// Quote ID
    #[serde(rename = "quoteId")]
    pub quote_id: String,

    /// Client-supplied Quote ID
    #[serde(rename = "clQuoteId")]
    pub cl_quote_id: String,

    /// Quote tag
    pub quote_tag: String,

    /// Quote timeout
    pub quote_time: String,

    /// A unique identifier of maker
    #[serde(rename = "traderCode")]
    pub trader_code: String,

    /// The legs of the quote
    pub legs: Vec<serde_json::Value>,
}

impl RestClient {
    /// Get quotes
    ///
    /// Retrieve quotes that you have sent or received.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-get-quotes)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The quotes request parameters
    ///
    /// # Returns
    /// Response containing quote information
    pub async fn get_quotes(&self, request: GetQuotesRequest) -> RestResult<QuoteInfo> {
        self.send_get_request(
            GET_QUOTES_ENDPOINT,
            Some(&request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_get_quotes_request_builder() {
        let request = GetQuotesRequest {
            rfq_id: Some("rfq_123".to_string()),
            cl_rfq_id: None,
            quote_id: None,
            cl_quote_id: None,
            state: Some("active".to_string()),
            begin: None,
            end: None,
            limit: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"rfqId\":\"rfq_123\""));
        assert!(json.contains("\"state\":\"active\""));
    }

    #[test]
    fn test_quote_info_deserialization() {
        let quote_json = json!({
            "cTime": "1597026383085",
            "uTime": "1597026383085",
            "state": "active",
            "rfqId": "rfq_123",
            "clRfqId": "client_rfq_123",
            "quoteId": "quote_456",
            "clQuoteId": "client_quote_456",
            "quoteTag": "test_quote",
            "quoteTime": "30000",
            "traderCode": "MAKER001",
            "legs": []
        });

        let quote_info: QuoteInfo = serde_json::from_value(quote_json).unwrap();
        assert_eq!(quote_info.state, "active");
        assert_eq!(quote_info.quote_id, "quote_456");
        assert_eq!(quote_info.trader_code, "MAKER001");
    }
}
