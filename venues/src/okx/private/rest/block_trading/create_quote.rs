use serde::{Deserialize, Serialize};

use crate::okx::{
    EndpointType, OrderSide, PositionSide, RestResult, TargetCurrency, TradeMode,
    private_client::RestClient,
};

/// Endpoint URL for creating quote
const CREATE_QUOTE_ENDPOINT: &str = "api/v5/rfq/create-quote";

/// Quote leg information for request
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteLegRequest {
    /// The instrument ID of quoted leg
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Trade mode
    #[serde(rename = "tdMode", skip_serializing_if = "Option::is_none")]
    pub td_mode: Option<TradeMode>,

    /// Margin currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,

    /// Size of the leg in contracts or spot
    pub sz: String,

    /// The price of the leg
    pub px: String,

    /// The direction of the leg
    pub side: OrderSide,

    /// Position side
    #[serde(rename = "posSide", skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<PositionSide>,

    /// Defines the unit of the "sz" attribute
    #[serde(rename = "tgtCcy", skip_serializing_if = "Option::is_none")]
    pub tgt_ccy: Option<TargetCurrency>,

    /// The quote currency used for trading
    #[serde(rename = "tradeQuoteCcy", skip_serializing_if = "Option::is_none")]
    pub trade_quote_ccy: Option<String>,
}

/// Request to create a quote
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuoteRequest {
    /// RFQ ID created by system
    #[serde(rename = "rfqId", skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,

    /// Client-supplied RFQ ID
    #[serde(rename = "clRfqId", skip_serializing_if = "Option::is_none")]
    pub cl_rfq_id: Option<String>,

    /// Quote tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_tag: Option<String>,

    /// Client-supplied Quote ID
    #[serde(rename = "clQuoteId", skip_serializing_if = "Option::is_none")]
    pub cl_quote_id: Option<String>,

    /// Whether this is a partial quote
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<bool>,

    /// Array of quote legs
    pub legs: Vec<QuoteLegRequest>,

    /// Anonymous quote (true/false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous: Option<bool>,
}

/// Quote leg in response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteLegResponse {
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Trade mode
    #[serde(rename = "tdMode")]
    pub td_mode: String,

    /// Margin currency
    pub ccy: Option<String>,

    /// Size of the leg
    pub sz: String,

    /// Price of the leg
    pub px: String,

    /// Direction of the leg
    pub side: OrderSide,

    /// Position side
    #[serde(rename = "posSide")]
    pub pos_side: Option<String>,

    /// Target currency
    #[serde(rename = "tgtCcy")]
    pub tgt_ccy: Option<String>,

    /// Trade quote currency
    #[serde(rename = "tradeQuoteCcy")]
    pub trade_quote_ccy: Option<String>,
}

/// Response from creating a quote
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuoteResponse {
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
    pub legs: Vec<QuoteLegResponse>,
}

impl RestClient {
    /// Create quote
    ///
    /// Create a quote in response to an RFQ.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-create-quote)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The quote creation request parameters
    ///
    /// # Returns
    /// A result containing the quote creation response
    pub async fn create_quote(
        &self,
        request: CreateQuoteRequest,
    ) -> RestResult<CreateQuoteResponse> {
        self.send_post_request(CREATE_QUOTE_ENDPOINT, request, EndpointType::PrivateAccount)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_create_quote_request_serialization() {
        let request = CreateQuoteRequest {
            rfq_id: Some("rfq_123".to_string()),
            cl_rfq_id: None,
            quote_tag: Some("test_quote".to_string()),
            cl_quote_id: Some("client_quote_123".to_string()),
            partial: Some(false),
            legs: vec![QuoteLegRequest {
                inst_id: "BTC-USDT-SWAP".to_string(),
                td_mode: Some(TradeMode::Cross),
                ccy: None,
                sz: "10".to_string(),
                px: "50000.0".to_string(),
                side: OrderSide::Buy,
                pos_side: Some(PositionSide::Long),
                tgt_ccy: None,
                trade_quote_ccy: None,
            }],
            anonymous: Some(true),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"rfqId\":\"rfq_123\""));
        assert!(json.contains("\"quoteTag\":\"test_quote\""));
        assert!(json.contains("\"partial\":false"));
        assert!(json.contains("\"anonymous\":true"));
        assert!(json.contains("\"px\":\"50000.0\""));
    }

    #[test]
    fn test_create_quote_response_deserialization() {
        let response_json = json!({
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
            "legs": [
                {
                    "instId": "BTC-USDT-SWAP",
                    "tdMode": "cross",
                    "ccy": null,
                    "sz": "10",
                    "px": "50000.0",
                    "side": "buy",
                    "posSide": "long",
                    "tgtCcy": null,
                    "tradeQuoteCcy": null
                }
            ]
        });

        let response: CreateQuoteResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.c_time, "1597026383085");
        assert_eq!(response.state, "active");
        assert_eq!(response.rfq_id, "rfq_123");
        assert_eq!(response.quote_id, "quote_456");
        assert_eq!(response.quote_tag, "test_quote");
        assert_eq!(response.trader_code, "MAKER001");
        assert_eq!(response.legs.len(), 1);
        assert_eq!(response.legs[0].px, "50000.0");
    }

    #[test]
    fn test_create_quote_full_response() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
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
                    "legs": [
                        {
                            "instId": "BTC-USDT-SWAP",
                            "tdMode": "cross",
                            "ccy": "USDT",
                            "sz": "10",
                            "px": "50000.0",
                            "side": "buy",
                            "posSide": "long",
                            "tgtCcy": "base_ccy",
                            "tradeQuoteCcy": "USDT"
                        }
                    ]
                }
            ]
        });

        let response: ApiResponse<CreateQuoteResponse> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let quote = &response.data[0];
        assert_eq!(quote.state, "active");
        assert_eq!(quote.legs[0].ccy, Some("USDT".to_string()));
    }
}
