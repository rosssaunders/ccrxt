use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult};

const TRADE_HISTORY_ENDPOINT: &str = "/openApi/spot/v1/trade/myTrades";

/// Trade information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    /// Trading symbol
    pub symbol: String,

    /// Trade ID
    pub id: i64,

    /// Order ID
    pub order_id: i64,

    /// Price of the trade
    pub price: String,

    /// Quantity of the trade
    pub qty: String,

    /// Quote asset quantity traded
    pub quote_qty: String,

    /// Commission amount
    pub commission: f64,

    /// Commission asset type
    pub commission_asset: String,

    /// Trade time
    pub time: i64,

    /// Whether the buyer
    pub is_buyer: bool,

    /// Whether the maker
    pub is_maker: bool,
}

/// Request to get trade history
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradeHistoryRequest {
    /// Trading pair, e.g., BTC-USDT (must use uppercase letters)
    pub symbol: String,

    /// Order ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,

    /// Start timestamp, unit: ms (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// End timestamp, unit: ms (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,

    /// Starting trade ID, by default the latest trade will be retrieved (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_id: Option<i64>,

    /// Default 500, maximum 1000 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Request valid time window, unit: milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Timestamp of initiating the request, Unit: milliseconds
    pub timestamp: i64,
}

/// Response from getting trade history
#[derive(Debug, Clone)]
pub struct GetTradeHistoryResponse {
    /// List of trades
    pub trades: Vec<Trade>,
}

// We need to implement custom deserialization since the response is a direct array
impl<'de> Deserialize<'de> for GetTradeHistoryResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let trades = Vec::<Trade>::deserialize(deserializer)?;
        Ok(GetTradeHistoryResponse { trades })
    }
}

impl RestClient {
    /// Get trade history
    ///
    /// Retrieves trade details for the account.
    /// Rate limit: 5/s by UID
    ///
    /// # Arguments
    /// * `request` - The get trade history request
    ///
    /// # Returns
    /// A result containing the trade history or an error
    ///
    /// # Notes
    /// - Can only check data within the past 7 days range
    /// - If start_time/end_time is not filled, data of the past 24 hours is returned by default
    /// - Maximum 1000 records can be returned
    /// - Results are sorted by time field, from smallest to largest
    pub async fn get_trade_history(
        &self,
        request: &GetTradeHistoryRequest,
    ) -> RestResult<GetTradeHistoryResponse> {
        self.send_request(
            TRADE_HISTORY_ENDPOINT,
            reqwest::Method::GET,
            Some(request),
            EndpointType::Trading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_trade_history_request_serialization() {
        let request = GetTradeHistoryRequest {
            symbol: "BTC-USDT".to_string(),
            order_id: Some(123456789),
            start_time: Some(1658748648000),
            end_time: Some(1658748648400),
            from_id: Some(1000),
            limit: Some(100),
            recv_window: Some(5000),
            timestamp: 1658748648396,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-USDT"));
        assert!(serialized.contains("orderId=123456789"));
        assert!(serialized.contains("startTime=1658748648000"));
        assert!(serialized.contains("endTime=1658748648400"));
        assert!(serialized.contains("fromId=1000"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_get_trade_history_request_minimal() {
        let request = GetTradeHistoryRequest {
            symbol: "BTC-USDT".to_string(),
            order_id: None,
            start_time: None,
            end_time: None,
            from_id: None,
            limit: None,
            recv_window: None,
            timestamp: 1658748648396,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-USDT"));
        assert!(!serialized.contains("orderId"));
        assert!(!serialized.contains("startTime"));
        assert!(!serialized.contains("endTime"));
        assert!(!serialized.contains("fromId"));
        assert!(!serialized.contains("limit"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_trade_history_response_deserialization() {
        let json = r#"[
            {
                "symbol": "BTC-USDT",
                "id": 12345,
                "orderId": 123456789,
                "price": "50000.0",
                "qty": "0.001",
                "quoteQty": "50.0",
                "commission": 0.05,
                "commissionAsset": "USDT",
                "time": 1658748648396,
                "isBuyer": true,
                "isMaker": false
            },
            {
                "symbol": "BTC-USDT",
                "id": 12346,
                "orderId": 123456790,
                "price": "50100.0",
                "qty": "0.002",
                "quoteQty": "100.2",
                "commission": 0.1002,
                "commissionAsset": "USDT",
                "time": 1658748650000,
                "isBuyer": false,
                "isMaker": true
            }
        ]"#;

        let response: GetTradeHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.trades.len(), 2);

        let first_trade = &response.trades[0];
        assert_eq!(first_trade.symbol, "BTC-USDT");
        assert_eq!(first_trade.id, 12345);
        assert_eq!(first_trade.order_id, 123456789);
        assert_eq!(first_trade.price, "50000.0");
        assert_eq!(first_trade.qty, "0.001");
        assert_eq!(first_trade.quote_qty, "50.0");
        assert_eq!(first_trade.commission, 0.05);
        assert_eq!(first_trade.commission_asset, "USDT");
        assert_eq!(first_trade.time, 1658748648396);
        assert!(first_trade.is_buyer);
        assert!(!first_trade.is_maker);

        let second_trade = &response.trades[1];
        assert_eq!(second_trade.symbol, "BTC-USDT");
        assert_eq!(second_trade.id, 12346);
        assert_eq!(second_trade.order_id, 123456790);
        assert_eq!(second_trade.price, "50100.0");
        assert_eq!(second_trade.qty, "0.002");
        assert_eq!(second_trade.quote_qty, "100.2");
        assert_eq!(second_trade.commission, 0.1002);
        assert_eq!(second_trade.commission_asset, "USDT");
        assert_eq!(second_trade.time, 1658748650000);
        assert!(!second_trade.is_buyer);
        assert!(second_trade.is_maker);
    }

    #[test]
    fn test_trade_deserialization() {
        let json = r#"{
            "symbol": "BTC-USDT",
            "id": 12345,
            "orderId": 123456789,
            "price": "50000.0",
            "qty": "0.001",
            "quoteQty": "50.0",
            "commission": 0.05,
            "commissionAsset": "USDT",
            "time": 1658748648396,
            "isBuyer": true,
            "isMaker": false
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.symbol, "BTC-USDT");
        assert_eq!(trade.id, 12345);
        assert_eq!(trade.order_id, 123456789);
        assert_eq!(trade.price, "50000.0");
        assert_eq!(trade.qty, "0.001");
        assert_eq!(trade.quote_qty, "50.0");
        assert_eq!(trade.commission, 0.05);
        assert_eq!(trade.commission_asset, "USDT");
        assert_eq!(trade.time, 1658748648396);
        assert!(trade.is_buyer);
        assert!(!trade.is_maker);
    }
}
