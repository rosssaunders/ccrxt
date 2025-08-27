use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for executing quote
const EXECUTE_QUOTE_ENDPOINT: &str = "api/v5/rfq/execute-quote";

/// Quote leg execution information
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteQuoteLeg {
    /// The Instrument ID (e.g., "BTC-USDT-SWAP")
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// The size of each leg
    pub sz: String,
}

/// Request to execute a quote
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteQuoteRequest {
    /// RFQ ID created by system
    #[serde(rename = "rfqId", skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,

    /// Client-supplied RFQ ID
    /// Either rfqId or clRfqId is required. If both are passed, rfqId will be used
    #[serde(rename = "clRfqId", skip_serializing_if = "Option::is_none")]
    pub cl_rfq_id: Option<String>,

    /// Quote ID created by system
    #[serde(rename = "quoteId", skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,

    /// Client-supplied Quote ID
    /// Either quoteId or clQuoteId is required. If both are passed, quoteId will be used
    #[serde(rename = "clQuoteId", skip_serializing_if = "Option::is_none")]
    pub cl_quote_id: Option<String>,

    /// Block trade tag
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 16 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_td_tag: Option<String>,

    /// Array of objects containing the execution size of each leg of the RFQ
    /// The ratio of the leg sizes needs to be the same as the RFQ
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legs: Option<Vec<ExecuteQuoteLeg>>,
}

/// Trade leg execution result
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutedTradeLeg {
    /// Instrument ID (e.g., "BTC-USDT-SWAP")
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// The price the leg executed
    pub px: String,

    /// Size of the leg in contracts or spot
    pub sz: String,

    /// The direction of the leg from the Takers perspective
    pub side: String,

    /// Fee for the individual leg
    /// Negative fee represents the user transaction fee charged by the platform
    /// Positive fee represents rebate
    pub fee: String,

    /// Fee currency
    #[serde(rename = "feeCcy")]
    pub fee_ccy: String,

    /// Trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: String,
}

/// Response from executing a quote
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteQuoteResponse {
    /// The timestamp the trade was executed (Unix timestamp in milliseconds)
    #[serde(rename = "cTime")]
    pub c_time: String,

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

    /// Block trade ID
    #[serde(rename = "blockTdId")]
    pub block_td_id: String,

    /// Block trade tag
    pub tag: String,

    /// Taker trader code
    #[serde(rename = "tTraderCode")]
    pub t_trader_code: String,

    /// Maker trader code
    #[serde(rename = "mTraderCode")]
    pub m_trader_code: String,

    /// Legs of trade
    pub legs: Vec<ExecutedTradeLeg>,
}

impl RestClient {
    /// Execute quote
    ///
    /// Execute a quote from the RFQ process to complete a block trade.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-execute-quote)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The quote execution request parameters
    ///
    /// # Returns
    /// A result containing the quote execution response with trade details
    pub async fn execute_quote(
        &self,
        request: ExecuteQuoteRequest,
    ) -> RestResult<ExecuteQuoteResponse> {
        self.send_post_request(
            EXECUTE_QUOTE_ENDPOINT,
            request,
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_execute_quote_request_by_ids() {
        let request = ExecuteQuoteRequest {
            rfq_id: Some("rfq_123".to_string()),
            cl_rfq_id: None,
            quote_id: Some("quote_456".to_string()),
            cl_quote_id: None,
            block_td_tag: Some("tag_123".to_string()),
            legs: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"rfqId\":\"rfq_123\""));
        assert!(json.contains("\"quoteId\":\"quote_456\""));
        assert!(json.contains("\"blockTdTag\":\"tag_123\""));
        assert!(!json.contains("\"clRfqId\""));
        assert!(!json.contains("\"clQuoteId\""));
    }

    #[test]
    fn test_execute_quote_request_by_client_ids() {
        let legs = vec![ExecuteQuoteLeg {
            inst_id: "BTC-USDT-SWAP".to_string(),
            sz: "10".to_string(),
        }];

        let request = ExecuteQuoteRequest {
            rfq_id: None,
            cl_rfq_id: Some("client_rfq_123".to_string()),
            quote_id: None,
            cl_quote_id: Some("client_quote_456".to_string()),
            block_td_tag: None,
            legs: Some(legs),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"clRfqId\":\"client_rfq_123\""));
        assert!(json.contains("\"clQuoteId\":\"client_quote_456\""));
        assert!(json.contains("\"legs\":[{\"instId\":\"BTC-USDT-SWAP\",\"sz\":\"10\"}]"));
        assert!(!json.contains("\"rfqId\""));
        assert!(!json.contains("\"quoteId\""));
    }

    #[test]
    fn test_execute_quote_response_deserialization() {
        let response_json = json!({
            "cTime": "1597026383085",
            "rfqId": "rfq_123",
            "clRfqId": "client_rfq_123",
            "quoteId": "quote_456",
            "clQuoteId": "client_quote_456",
            "blockTdId": "block_trade_789",
            "tag": "test_tag",
            "tTraderCode": "TAKER001",
            "mTraderCode": "MAKER001",
            "legs": [
                {
                    "instId": "BTC-USDT-SWAP",
                    "px": "50000.0",
                    "sz": "10",
                    "side": "buy",
                    "fee": "-5.0",
                    "feeCcy": "USDT",
                    "tradeId": "trade_123"
                }
            ]
        });

        let response: ExecuteQuoteResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.c_time, "1597026383085");
        assert_eq!(response.rfq_id, "rfq_123");
        assert_eq!(response.quote_id, "quote_456");
        assert_eq!(response.block_td_id, "block_trade_789");
        assert_eq!(response.t_trader_code, "TAKER001");
        assert_eq!(response.m_trader_code, "MAKER001");
        assert_eq!(response.legs.len(), 1);
        assert_eq!(response.legs[0].inst_id, "BTC-USDT-SWAP");
        assert_eq!(response.legs[0].px, "50000.0");
        assert_eq!(response.legs[0].fee, "-5.0");
    }

    #[test]
    fn test_execute_quote_full_response() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "cTime": "1597026383085",
                    "rfqId": "rfq_123",
                    "clRfqId": "client_rfq_123",
                    "quoteId": "quote_456",
                    "clQuoteId": "client_quote_456",
                    "blockTdId": "block_trade_789",
                    "tag": "test_tag",
                    "tTraderCode": "TAKER001",
                    "mTraderCode": "MAKER001",
                    "legs": [
                        {
                            "instId": "BTC-USDT-SWAP",
                            "px": "50000.0",
                            "sz": "10",
                            "side": "buy",
                            "fee": "-5.0",
                            "feeCcy": "USDT",
                            "tradeId": "trade_123"
                        },
                        {
                            "instId": "ETH-USDT-SWAP",
                            "px": "3000.0",
                            "sz": "20",
                            "side": "sell",
                            "fee": "2.5",
                            "feeCcy": "USDT",
                            "tradeId": "trade_124"
                        }
                    ]
                }
            ]
        });

        let response: ApiResponse<ExecuteQuoteResponse> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let exec_response = &response.data[0];
        assert_eq!(exec_response.legs.len(), 2);
        assert_eq!(exec_response.legs[1].inst_id, "ETH-USDT-SWAP");
        assert_eq!(exec_response.legs[1].side, "sell");
        assert_eq!(exec_response.legs[1].fee, "2.5");
    }

    #[test]
    fn test_execute_quote_leg_serialization() {
        let leg = ExecuteQuoteLeg {
            inst_id: "BTC-USD-FUTURES".to_string(),
            sz: "100".to_string(),
        };

        let json = serde_json::to_string(&leg).unwrap();
        assert!(json.contains("\"instId\":\"BTC-USD-FUTURES\""));
        assert!(json.contains("\"sz\":\"100\""));
    }
}
