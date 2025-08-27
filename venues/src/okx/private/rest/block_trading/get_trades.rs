use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for getting trades
const GET_TRADES_ENDPOINT: &str = "api/v5/rfq/trades";

/// Request parameters for getting trades
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradesRequest {
    /// RFQ ID created by system
    #[serde(rename = "rfqId", skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,

    /// Client-supplied RFQ ID
    #[serde(rename = "clRfqId", skip_serializing_if = "Option::is_none")]
    pub cl_rfq_id: Option<String>,

    /// Quote ID
    #[serde(rename = "quoteId", skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,

    /// Block trade ID
    #[serde(rename = "blockTdId", skip_serializing_if = "Option::is_none")]
    pub block_td_id: Option<String>,

    /// Client-supplied Quote ID
    #[serde(rename = "clQuoteId", skip_serializing_if = "Option::is_none")]
    pub cl_quote_id: Option<String>,

    /// The starting rfq id the request to begin with. Pagination of data to return records
    /// newer than the requested blockTdId, not including beginId.
    #[serde(rename = "beginId", skip_serializing_if = "Option::is_none")]
    pub begin_id: Option<String>,

    /// The last rfq id the request to end with. Pagination of data to return records
    /// earlier than the requested blockTdId, not including endId.
    #[serde(rename = "endId", skip_serializing_if = "Option::is_none")]
    pub end_id: Option<String>,

    /// Filter trade execution time with a begin timestamp (UTC timezone).
    /// Unix timestamp format in milliseconds
    #[serde(rename = "beginTs", skip_serializing_if = "Option::is_none")]
    pub begin_ts: Option<String>,

    /// Filter trade execution time with an end timestamp (UTC timezone).
    /// Unix timestamp format in milliseconds
    #[serde(rename = "endTs", skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<String>,

    /// Number of results per request. The maximum is 100 which is also the default value.
    /// If the number of trades in the requested range is bigger than 100, the latest 100 trades
    /// in the range will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,

    /// Whether the trade is filled successfully. true: the default value. false.
    #[serde(rename = "isSuccessful", skip_serializing_if = "Option::is_none")]
    pub is_successful: Option<bool>,
}

/// Trade leg information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeLeg {
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// The price the leg executed
    pub px: String,

    /// Size of the leg in contracts or spot
    pub sz: String,

    /// The direction of the leg. Valid value can be buy or sell.
    pub side: String,

    /// Fee. Negative number represents the user transaction fee charged by the platform.
    /// Positive number represents rebate.
    pub fee: String,

    /// Fee currency
    #[serde(rename = "feeCcy")]
    pub fee_ccy: String,

    /// Last traded ID
    #[serde(rename = "tradeId")]
    pub trade_id: String,

    /// The quote currency used for trading. Only applicable to SPOT.
    #[serde(rename = "tradeQuoteCcy")]
    pub trade_quote_ccy: String,
}

/// Trade information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeInfo {
    /// The time the trade was executed (Unix timestamp in milliseconds)
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

    /// Trade tag. The block trade will have the tag of the RFQ or Quote it corresponds to.
    pub tag: String,

    /// A unique identifier of the Taker. Empty if the anonymous parameter of the RFQ
    /// is set to be true.
    #[serde(rename = "tTraderCode")]
    pub t_trader_code: String,

    /// A unique identifier of the Maker. Empty if the anonymous parameter of the Quote
    /// is set to be true.
    #[serde(rename = "mTraderCode")]
    pub m_trader_code: String,

    /// Whether the trade is filled successfully
    #[serde(rename = "isSuccessful")]
    pub is_successful: bool,

    /// Error code for unsuccessful trades. It is "" for successful trade.
    #[serde(rename = "errorCode")]
    pub error_code: String,

    /// Legs of trade
    pub legs: Vec<TradeLeg>,
}

impl RestClient {
    /// Get trades
    ///
    /// Retrieves the executed trades that the user is a counterparty to (either as the
    /// creator or the receiver).
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-get-trades)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The trades request parameters
    ///
    /// # Returns
    /// Response containing trade information
    pub async fn get_trades(&self, request: GetTradesRequest) -> RestResult<TradeInfo> {
        self.send_get_request(
            GET_TRADES_ENDPOINT,
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
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_get_trades_request_builder() {
        let request = GetTradesRequest {
            rfq_id: Some("rfq_123".to_string()),
            cl_rfq_id: None,
            quote_id: None,
            block_td_id: None,
            cl_quote_id: None,
            begin_id: None,
            end_id: None,
            begin_ts: None,
            end_ts: None,
            limit: Some("50".to_string()),
            is_successful: Some(true),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"rfqId\":\"rfq_123\""));
        assert!(json.contains("\"limit\":\"50\""));
        assert!(json.contains("\"isSuccessful\":true"));
        assert!(!json.contains("\"quoteId\""));
    }

    #[test]
    fn test_trade_leg_deserialization() {
        let leg_json = json!({
            "instId": "BTC-USDT-SWAP",
            "px": "50000.0",
            "sz": "1",
            "side": "buy",
            "fee": "-0.5",
            "feeCcy": "USDT",
            "tradeId": "12345",
            "tradeQuoteCcy": "USDT"
        });

        let leg: TradeLeg = serde_json::from_value(leg_json).unwrap();
        assert_eq!(leg.inst_id, "BTC-USDT-SWAP");
        assert_eq!(leg.px, "50000.0");
        assert_eq!(leg.sz, "1");
        assert_eq!(leg.side, "buy");
        assert_eq!(leg.fee, "-0.5");
        assert_eq!(leg.fee_ccy, "USDT");
        assert_eq!(leg.trade_id, "12345");
        assert_eq!(leg.trade_quote_ccy, "USDT");
    }

    #[test]
    fn test_trade_info_deserialization() {
        let trade_json = json!({
            "cTime": "1597026383085",
            "rfqId": "rfq_123",
            "clRfqId": "cl_rfq_123",
            "quoteId": "quote_456",
            "clQuoteId": "cl_quote_456",
            "blockTdId": "block_789",
            "tag": "test_tag",
            "tTraderCode": "TAKER001",
            "mTraderCode": "MAKER001",
            "isSuccessful": true,
            "errorCode": "",
            "legs": [
                {
                    "instId": "BTC-USDT-SWAP",
                    "px": "50000.0",
                    "sz": "1",
                    "side": "buy",
                    "fee": "-0.5",
                    "feeCcy": "USDT",
                    "tradeId": "12345",
                    "tradeQuoteCcy": "USDT"
                }
            ]
        });

        let trade_info: TradeInfo = serde_json::from_value(trade_json).unwrap();
        assert_eq!(trade_info.c_time, "1597026383085");
        assert_eq!(trade_info.rfq_id, "rfq_123");
        assert_eq!(trade_info.cl_rfq_id, "cl_rfq_123");
        assert_eq!(trade_info.quote_id, "quote_456");
        assert_eq!(trade_info.block_td_id, "block_789");
        assert_eq!(trade_info.tag, "test_tag");
        assert_eq!(trade_info.t_trader_code, "TAKER001");
        assert_eq!(trade_info.m_trader_code, "MAKER001");
        assert!(trade_info.is_successful);
        assert_eq!(trade_info.error_code, "");
        assert_eq!(trade_info.legs.len(), 1);
        assert_eq!(trade_info.legs[0].inst_id, "BTC-USDT-SWAP");
    }

    #[test]
    fn test_get_trades_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "cTime": "1597026383085",
                    "rfqId": "rfq_123",
                    "clRfqId": "",
                    "quoteId": "quote_456",
                    "clQuoteId": "",
                    "blockTdId": "block_789",
                    "tag": "",
                    "tTraderCode": "",
                    "mTraderCode": "MAKER001",
                    "isSuccessful": false,
                    "errorCode": "51008",
                    "legs": []
                }
            ]
        });

        let response: ApiResponse<TradeInfo> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].rfq_id, "rfq_123");
        assert!(!response.data[0].is_successful);
        assert_eq!(response.data[0].error_code, "51008");
    }

    #[test]
    fn test_get_trades_request_empty() {
        let request = GetTradesRequest::default();
        let json = serde_json::to_string(&request).unwrap();
        // Should serialize to empty object
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_get_trades_request_with_timestamps() {
        let request = GetTradesRequest {
            rfq_id: None,
            cl_rfq_id: None,
            quote_id: None,
            block_td_id: None,
            cl_quote_id: None,
            begin_id: None,
            end_id: None,
            begin_ts: Some("1597026383085".to_string()),
            end_ts: Some("1597026403085".to_string()),
            limit: None,
            is_successful: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"beginTs\":\"1597026383085\""));
        assert!(json.contains("\"endTs\":\"1597026403085\""));
    }
}
