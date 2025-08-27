use serde::{Deserialize, Serialize};

use crate::okx::{
    EndpointType, OrderSide, PositionSide, RestResult, RfqState, TargetCurrency, TradeMode,
    private_client::RestClient,
};

/// Endpoint URL for creating RFQ
const CREATE_RFQ_ENDPOINT: &str = "api/v5/rfq/create-rfq";

/// RFQ leg information for request
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RfqLegRequest {
    /// The Instrument ID of each leg (e.g., "BTC-USDT-SWAP")
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Trade mode
    /// Margin mode: "cross", "isolated"
    /// Non-Margin mode: "cash"
    #[serde(rename = "tdMode", skip_serializing_if = "Option::is_none")]
    pub td_mode: Option<TradeMode>,

    /// Margin currency
    /// Only applicable to cross MARGIN orders in Futures mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,

    /// The size of each leg
    pub sz: String,

    /// Taker expected price for the RFQ
    /// If provided, RFQ trade will be automatically executed if the price from the quote
    /// is better than or equal to the price specified
    #[serde(rename = "lmtPx", skip_serializing_if = "Option::is_none")]
    pub lmt_px: Option<String>,

    /// The direction of each leg
    pub side: OrderSide,

    /// Position side
    /// Only applicable to FUTURES/SWAP
    #[serde(rename = "posSide", skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<PositionSide>,

    /// Defines the unit of the "sz" attribute
    /// Only applicable to instType = SPOT
    #[serde(rename = "tgtCcy", skip_serializing_if = "Option::is_none")]
    pub tgt_ccy: Option<TargetCurrency>,

    /// The quote currency used for trading
    /// Only applicable to SPOT
    #[serde(rename = "tradeQuoteCcy", skip_serializing_if = "Option::is_none")]
    pub trade_quote_ccy: Option<String>,
}

/// Request to create a new RFQ
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRfqRequest {
    /// The trader code(s) of the counterparties who receive the RFQ
    pub counterparties: Vec<String>,

    /// Submit RFQ on a disclosed or anonymous basis
    /// Default is false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous: Option<bool>,

    /// Client-supplied RFQ ID
    /// A combination of case-sensitive alpha-numeric, all numbers, or all letters of up to 32 characters
    #[serde(rename = "clRfqId", skip_serializing_if = "Option::is_none")]
    pub cl_rfq_id: Option<String>,

    /// RFQ tag
    /// The block trade associated with the RFQ will have the same tag
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 16 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// Whether the RFQ can be partially filled provided that the shape of legs stays the same
    /// Default is false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_partial_execution: Option<bool>,

    /// Array of objects containing each leg of the RFQ
    /// Maximum 15 legs can be placed per request
    pub legs: Vec<RfqLegRequest>,
}

/// RFQ leg information in response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RfqLegResponse {
    /// Instrument ID (e.g., "BTC-USDT-SWAP")
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Trade mode
    #[serde(rename = "tdMode")]
    pub td_mode: String,

    /// Margin currency
    pub ccy: Option<String>,

    /// Size of the leg in contracts or spot
    pub sz: String,

    /// The direction of the leg
    pub side: OrderSide,

    /// Position side
    #[serde(rename = "posSide")]
    pub pos_side: Option<String>,

    /// Defines the unit of the "sz" attribute
    #[serde(rename = "tgtCcy")]
    pub tgt_ccy: Option<String>,

    /// The quote currency used for trading
    #[serde(rename = "tradeQuoteCcy")]
    pub trade_quote_ccy: Option<String>,
}

/// Response from creating an RFQ
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRfqResponse {
    /// The timestamp the RFQ was created (Unix timestamp in milliseconds)
    #[serde(rename = "cTime")]
    pub c_time: String,

    /// The timestamp the RFQ was last updated (Unix timestamp in milliseconds)
    #[serde(rename = "uTime")]
    pub u_time: String,

    /// The status of the RFQ
    pub state: RfqState,

    /// The list of counterparties traderCode the RFQ was broadcast to
    pub counterparties: Vec<String>,

    /// The timestamp the RFQ expires (Unix timestamp in milliseconds)
    #[serde(rename = "validUntil")]
    pub valid_until: String,

    /// Client-supplied RFQ ID
    #[serde(rename = "clRfqId")]
    pub cl_rfq_id: String,

    /// RFQ tag
    pub tag: String,

    /// Whether the RFQ can be partially filled
    pub allow_partial_execution: bool,

    /// A unique identifier of taker
    #[serde(rename = "traderCode")]
    pub trader_code: String,

    /// The unique identifier of the RFQ generated by system
    #[serde(rename = "rfqId")]
    pub rfq_id: String,

    /// Array of objects containing each leg of the RFQ
    pub legs: Vec<RfqLegResponse>,
}

impl RestClient {
    /// Create RFQ
    ///
    /// Creates a new RFQ for block trading.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-create-rfq)
    ///
    /// Rate limit: 5 requests per 2 seconds; 80 requests per 12 hours
    ///
    /// # Arguments
    /// * `request` - The RFQ creation request parameters
    ///
    /// # Returns
    /// A result containing the RFQ creation response with RFQ ID and status
    pub async fn create_rfq(&self, request: CreateRfqRequest) -> RestResult<CreateRfqResponse> {
        self.send_post_request(CREATE_RFQ_ENDPOINT, request, EndpointType::PrivateAccount)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_create_rfq_request_serialization() {
        let request = CreateRfqRequest {
            counterparties: vec!["MM001".to_string(), "MM002".to_string()],
            anonymous: Some(false),
            cl_rfq_id: Some("client_rfq_123".to_string()),
            tag: Some("test_tag".to_string()),
            allow_partial_execution: Some(true),
            legs: vec![RfqLegRequest {
                inst_id: "BTC-USDT-SWAP".to_string(),
                td_mode: Some(TradeMode::Cross),
                ccy: None,
                sz: "10".to_string(),
                lmt_px: Some("50000.0".to_string()),
                side: OrderSide::Buy,
                pos_side: Some(PositionSide::Long),
                tgt_ccy: None,
                trade_quote_ccy: None,
            }],
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"counterparties\":[\"MM001\",\"MM002\"]"));
        assert!(json.contains("\"anonymous\":false"));
        assert!(json.contains("\"clRfqId\":\"client_rfq_123\""));
        assert!(json.contains("\"allowPartialExecution\":true"));
        assert!(json.contains("\"instId\":\"BTC-USDT-SWAP\""));
        assert!(json.contains("\"tdMode\":\"cross\""));
        assert!(json.contains("\"side\":\"buy\""));
        assert!(json.contains("\"posSide\":\"long\""));
    }

    #[test]
    fn test_create_rfq_response_deserialization() {
        let response_json = json!({
            "cTime": "1597026383085",
            "uTime": "1597026383085",
            "state": "active",
            "counterparties": ["MM001"],
            "validUntil": "1597026503085",
            "clRfqId": "client_rfq_123",
            "tag": "test_tag",
            "allowPartialExecution": true,
            "traderCode": "TAKER001",
            "rfqId": "rfq_123456",
            "legs": [
                {
                    "instId": "BTC-USDT-SWAP",
                    "tdMode": "cross",
                    "ccy": null,
                    "sz": "10",
                    "side": "buy",
                    "posSide": "long",
                    "tgtCcy": null,
                    "tradeQuoteCcy": null
                }
            ]
        });

        let response: CreateRfqResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.c_time, "1597026383085");
        assert_eq!(response.state, RfqState::Active);
        assert_eq!(response.counterparties.len(), 1);
        assert_eq!(response.counterparties[0], "MM001");
        assert_eq!(response.cl_rfq_id, "client_rfq_123");
        assert!(response.allow_partial_execution);
        assert_eq!(response.trader_code, "TAKER001");
        assert_eq!(response.rfq_id, "rfq_123456");
        assert_eq!(response.legs.len(), 1);
        assert_eq!(response.legs[0].inst_id, "BTC-USDT-SWAP");
    }

    #[test]
    fn test_create_rfq_full_response_deserialization() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "cTime": "1597026383085",
                    "uTime": "1597026383085",
                    "state": "active",
                    "counterparties": ["MM001", "MM002"],
                    "validUntil": "1597026503085",
                    "clRfqId": "client_rfq_123",
                    "tag": "test_tag",
                    "allowPartialExecution": false,
                    "traderCode": "TAKER001",
                    "rfqId": "rfq_123456",
                    "legs": [
                        {
                            "instId": "BTC-USDT-SWAP",
                            "tdMode": "cross",
                            "ccy": "USDT",
                            "sz": "10",
                            "side": "buy",
                            "posSide": "long",
                            "tgtCcy": "base_ccy",
                            "tradeQuoteCcy": "USDT"
                        }
                    ]
                }
            ]
        });

        let response: ApiResponse<CreateRfqResponse> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let rfq = &response.data[0];
        assert_eq!(rfq.state, RfqState::Active);
        assert_eq!(rfq.counterparties.len(), 2);
        assert!(!rfq.allow_partial_execution);
        assert_eq!(rfq.legs[0].ccy, Some("USDT".to_string()));
        assert_eq!(rfq.legs[0].tgt_ccy, Some("base_ccy".to_string()));
    }

    #[test]
    fn test_rfq_leg_request_minimal() {
        let leg = RfqLegRequest {
            inst_id: "ETH-USDT".to_string(),
            td_mode: None,
            ccy: None,
            sz: "5".to_string(),
            lmt_px: None,
            side: OrderSide::Sell,
            pos_side: None,
            tgt_ccy: None,
            trade_quote_ccy: None,
        };

        let json = serde_json::to_string(&leg).unwrap();
        assert!(json.contains("\"instId\":\"ETH-USDT\""));
        assert!(json.contains("\"sz\":\"5\""));
        assert!(json.contains("\"side\":\"sell\""));
        // Optional fields should not be present
        assert!(!json.contains("\"tdMode\""));
        assert!(!json.contains("\"ccy\""));
        assert!(!json.contains("\"lmtPx\""));
    }
}
