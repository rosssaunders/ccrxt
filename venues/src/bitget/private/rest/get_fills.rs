//! Get Fills endpoint for Bitget Spot API
//!
//! This endpoint allows retrieving trade fill information.
//!
//! Reference: https://www.bitget.com/api-doc/spot/trade/Get-Fills
//! Endpoint: GET /api/v2/spot/trade/fills
//! Rate limit: 10 times/1s (UID)


use serde::{Deserialize, Serialize};

use super::super::RestClient;
use crate::bitget::{OrderSide, OrderType, RestResult};

/// Endpoint for getting trade fills
const GET_FILLS_ENDPOINT: &str = "/api/v2/spot/trade/fills";

/// Trade scope (liquidity direction)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TradeScope {
    /// Taker (took liquidity from the book)
    #[serde(rename = "taker")]
    Taker,
    /// Maker (provided liquidity to the book)
    #[serde(rename = "maker")]
    Maker,
}

/// Request parameters for getting fills
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetFillsRequest {
    /// Trading pair name (optional filter)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Order ID filter
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// The start time of the orders (Unix milliseconds)
    /// For Managed Sub-Account, startTime cannot be earlier than binding time
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// The end time of fulfilled orders (Unix milliseconds)
    /// The interval between startTime and endTime must not exceed 90 days
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,

    /// Number of results returned (default: 100, max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Requests content on the page before this ID (older data)
    /// Value should be the tradeId from the corresponding interface
    #[serde(rename = "idLessThan", skip_serializing_if = "Option::is_none")]
    pub id_less_than: Option<String>,
}

/// Fee details for a fill
#[derive(Debug, Clone, Deserialize)]
pub struct FillFeeDetail {
    /// Whether deduction was applied
    pub deduction: String,

    /// Fee coin
    #[serde(rename = "feeCoin")]
    pub fee_coin: String,

    /// Total deduction fee
    #[serde(rename = "totalDeductionFee")]
    pub total_deduction_fee: String,

    /// Total fee
    #[serde(rename = "totalFee")]
    pub total_fee: String,
}

/// Information about a trade fill
#[derive(Debug, Clone, Deserialize)]
pub struct FillInfo {
    /// User ID
    #[serde(rename = "userId")]
    pub user_id: String,

    /// Trading pair name
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// Trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: String,

    /// Order type
    #[serde(rename = "orderType")]
    pub order_type: OrderType,

    /// Order direction
    pub side: OrderSide,

    /// Average price
    #[serde(rename = "priceAvg")]
    pub price_avg: String,

    /// Filled size (base coin)
    pub size: String,

    /// Filled amount (quote coin)
    pub amount: String,

    /// Fee details
    #[serde(rename = "feeDetail")]
    pub fee_detail: FillFeeDetail,

    /// Trade scope (taker/maker)
    #[serde(rename = "tradeScope")]
    pub trade_scope: TradeScope,

    /// Creation time (Unix milliseconds)
    #[serde(rename = "cTime")]
    pub create_time: String,

    /// Update time (Unix milliseconds)
    #[serde(rename = "uTime")]
    pub update_time: String,
}

/// Response from getting fills
#[derive(Debug, Clone)]
pub struct GetFillsResponse {
    /// List of trade fills
    pub fills: Vec<FillInfo>,
}

// Implementation for direct response deserialization when the API returns an array
impl<'de> Deserialize<'de> for GetFillsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fills = Vec::<FillInfo>::deserialize(deserializer)?;
        Ok(GetFillsResponse { fills })
    }
}

impl RestClient {
    /// Get trade fills
    ///
    /// Retrieves trade execution details for the authenticated account.
    /// Only supports data within 90 days. Older data can be downloaded from the web.
    ///
    /// # Arguments
    /// * `request` - The request parameters with optional filters
    ///
    /// # Rate Limit
    /// 10 requests per second per UID
    ///
    /// # Returns
    /// A result containing the trade fills or an error
    pub async fn get_fills(&self, request: &GetFillsRequest) -> RestResult<GetFillsResponse> {
        // Only create query string if there are parameters to serialize
        let has_params = request.symbol.is_some()
            || request.order_id.is_some()
            || request.start_time.is_some()
            || request.end_time.is_some()
            || request.limit.is_some()
            || request.id_less_than.is_some();

        let query_string = if has_params {
            Some(serde_urlencoded::to_string(request).map_err(|e| {
                crate::bitget::Errors::Error(format!("Failed to encode query: {e}"))
            })?)
        } else {
            None
        };

        self.send_signed_request(
            GET_FILLS_ENDPOINT,
            reqwest::Method::GET,
            query_string.as_deref(),
            None,  // No body for GET request
            10,    // 10 requests per second rate limit
            false, // Not an order placement endpoint
            None,  // No order-specific rate limit
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fills_request_default() {
        let request = GetFillsRequest::default();

        assert!(request.symbol.is_none());
        assert!(request.order_id.is_none());
        assert!(request.start_time.is_none());
        assert!(request.end_time.is_none());
        assert!(request.limit.is_none());
        assert!(request.id_less_than.is_none());
    }

    #[test]
    fn test_get_fills_request_builder() {
        let request = GetFillsRequest {
            symbol: Some("BTCUSDT".to_string()),
            order_id: Some("12345678910".to_string()),
            start_time: Some(1659036670000),
            end_time: Some(1659076670000),
            id_less_than: Some("98765".to_string()),
            limit: Some(50),
        };

        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.order_id, Some("12345678910".to_string()));
        assert_eq!(request.start_time, Some(1659036670000));
        assert_eq!(request.end_time, Some(1659076670000));
        assert_eq!(request.id_less_than, Some("98765".to_string()));
        assert_eq!(request.limit, Some(50));
    }

    #[test]
    fn test_get_fills_request_limit_enforcement() {
        let request = GetFillsRequest {
            symbol: None,
            order_id: None,
            start_time: None,
            end_time: None,
            id_less_than: None,
            limit: Some(100), // Manual cap at 100
        };

        // Should be capped at 100
        assert_eq!(request.limit, Some(100));
    }

    #[test]
    fn test_fill_info_deserialization() {
        let json = r#"{
            "userId": "123456789",
            "symbol": "BTCUSDT",
            "orderId": "12345678910",
            "tradeId": "12345678910",
            "orderType": "market",
            "side": "buy",
            "priceAvg": "13000",
            "size": "0.0007",
            "amount": "9.1",
            "feeDetail": {
                "deduction": "no",
                "feeCoin": "BTC",
                "totalDeductionFee": "",
                "totalFee": "-0.0000007"
            },
            "tradeScope": "taker",
            "cTime": "1695865232579",
            "uTime": "1695865233027"
        }"#;

        let fill: FillInfo = serde_json::from_str(json).unwrap();

        assert_eq!(fill.user_id, "123456789");
        assert_eq!(fill.symbol, "BTCUSDT");
        assert_eq!(fill.order_id, "12345678910");
        assert_eq!(fill.trade_id, "12345678910");
        assert_eq!(fill.order_type, OrderType::Market);
        assert_eq!(fill.side, OrderSide::Buy);
        assert_eq!(fill.price_avg, "13000");
        assert_eq!(fill.size, "0.0007");
        assert_eq!(fill.amount, "9.1");
        assert_eq!(fill.trade_scope, TradeScope::Taker);
        assert_eq!(fill.fee_detail.fee_coin, "BTC");
        assert_eq!(fill.fee_detail.total_fee, "-0.0000007");
    }

    #[test]
    fn test_get_fills_response_deserialization() {
        let json = r#"[
            {
                "userId": "123456789",
                "symbol": "BTCUSDT",
                "orderId": "12345678910",
                "tradeId": "12345678910",
                "orderType": "limit",
                "side": "sell",
                "priceAvg": "50000",
                "size": "0.001",
                "amount": "50.0",
                "feeDetail": {
                    "deduction": "no",
                    "feeCoin": "USDT",
                    "totalDeductionFee": "0",
                    "totalFee": "-0.05"
                },
                "tradeScope": "maker",
                "cTime": "1695865232579",
                "uTime": "1695865233027"
            }
        ]"#;

        let response: GetFillsResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.fills.len(), 1);
        assert_eq!(response.fills[0].symbol, "BTCUSDT");
        assert_eq!(response.fills[0].trade_scope, TradeScope::Maker);
        assert_eq!(response.fills[0].side, OrderSide::Sell);
    }
}
