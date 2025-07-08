use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::{ResponseHeaders, RestResponse, Result};

const ADD_MARGIN_ORDER_ENDPOINT: &str = "/api/v3/hf/margin/order";

/// Order side (buy or sell)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MarginOrderSide {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

/// Order type (limit or market)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MarginOrderType {
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "market")]
    Market,
}

/// Self Trade Prevention strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MarginOrderStp {
    #[serde(rename = "CN")]
    CancelNewest,
    #[serde(rename = "CO")]
    CancelOldest,
    #[serde(rename = "CB")]
    CancelBoth,
    #[serde(rename = "DC")]
    DecrementAndCancel,
}

/// Time in force for margin order
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MarginOrderTimeInForce {
    #[serde(rename = "GTC")]
    GoodTillCancelled,
    #[serde(rename = "IOC")]
    ImmediateOrCancel,
    #[serde(rename = "FOK")]
    FillOrKill,
}

/// Request for placing a margin order
#[derive(Debug, Clone, Serialize)]
pub struct AddMarginOrderRequest {
    #[serde(rename = "clientOid")]
    pub client_oid: String,
    pub side: MarginOrderSide,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<MarginOrderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp: Option<MarginOrderStp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<MarginOrderTimeInForce>,
    #[serde(rename = "postOnly", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<bool>,
}

/// Response for placing a margin order
#[derive(Debug, Clone, Deserialize)]
pub struct AddMarginOrderResponse {
    #[serde(rename = "orderId")]
    pub order_id: String,
}

impl RestClient {
    /// Place a margin order (cross or isolated)
    ///
    /// This endpoint allows placing a limit or market order in the margin trading system.
    pub async fn add_margin_order(
        &self,
        request: AddMarginOrderRequest,
    ) -> Result<(AddMarginOrderResponse, ResponseHeaders)> {
        let body = serde_json::to_string(&request).map_err(|e| {
            crate::kucoin::ApiError::JsonParsing(format!("Failed to serialize request: {}", e))
        })?;
        let (response, headers): (RestResponse<AddMarginOrderResponse>, ResponseHeaders) =
            self.post(ADD_MARGIN_ORDER_ENDPOINT, &body).await?;
        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_margin_order_request_creation() {
        let req = AddMarginOrderRequest {
            client_oid: "test-uuid".to_string(),
            side: MarginOrderSide::Buy,
            symbol: "BTC-USDT".to_string(),
            r#type: Some(MarginOrderType::Limit),
            stp: Some(MarginOrderStp::CancelNewest),
            price: Some("30000.0".to_string()),
            size: Some("0.01".to_string()),
            time_in_force: Some(MarginOrderTimeInForce::GoodTillCancelled),
            post_only: Some(true),
            hidden: Some(false),
            iceberg: Some(false),
        };
        assert_eq!(req.client_oid, "test-uuid");
        assert_eq!(req.side, MarginOrderSide::Buy);
        assert_eq!(req.symbol, "BTC-USDT");
        assert_eq!(req.r#type, Some(MarginOrderType::Limit));
    }

    #[test]
    fn test_margin_order_side_serialization() {
        assert_eq!(
            serde_json::to_string(&MarginOrderSide::Buy).unwrap(),
            "\"buy\""
        );
        assert_eq!(
            serde_json::to_string(&MarginOrderSide::Sell).unwrap(),
            "\"sell\""
        );
    }
}
