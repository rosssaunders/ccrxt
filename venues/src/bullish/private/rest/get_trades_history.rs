use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::enums::OrderSide;
use crate::bullish::{
    DataOrPaginated, EndpointType, PaginatedResult, PaginationParams, RestResult,
};

/// Endpoint URL for historical trades operations
const TRADES_HISTORY_ENDPOINT: &str = "/v1/history/trades";

/// Historical trade execution details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryTrade {
    /// Unique trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: String,

    /// Order ID associated with this trade
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// Market symbol
    pub symbol: String,

    /// Execution price
    pub price: String,

    /// Executed quantity
    pub quantity: String,

    /// Quote amount
    #[serde(rename = "quoteAmount")]
    pub quote_amount: String,

    /// Base asset fee
    #[serde(rename = "baseFee")]
    pub base_fee: String,

    /// Quote asset fee
    #[serde(rename = "quoteFee")]
    pub quote_fee: String,

    /// Trade side
    pub side: OrderSide,

    /// Whether this is the taker side of the trade
    #[serde(rename = "isTaker")]
    pub is_taker: bool,

    /// Amount of rebate credited for this trade
    #[serde(rename = "tradeRebateAmount")]
    pub trade_rebate_amount: String,

    /// Asset symbol in which rebate is paid
    #[serde(rename = "tradeRebateAssetSymbol")]
    pub trade_rebate_asset_symbol: String,

    /// Trade execution datetime, ISO 8601 with milliseconds
    #[serde(rename = "createdAtDatetime")]
    pub created_at_datetime: String,

    /// Trade execution timestamp (ms since epoch)
    #[serde(rename = "createdAtTimestamp")]
    pub created_at_timestamp: u64,
}

/// Parameters for querying historical trades
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetTradesHistoryParams {
    /// Trading account ID (required for accounts with multiple trading accounts)
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,

    /// Market symbol filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Order ID filter (optional)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Trade ID filter (optional)
    #[serde(rename = "tradeId", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,

    /// Start ISO8601 datetime filter (optional)
    #[serde(
        rename = "createdAtDatetime[gte]",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_datetime_gte: Option<String>,

    /// End ISO8601 datetime filter (optional)
    #[serde(
        rename = "createdAtDatetime[lte]",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_datetime_lte: Option<String>,

    /// Start timestamp (ms) filter (optional)
    #[serde(
        rename = "createdAtTimestamp[gte]",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_timestamp_gte: Option<u64>,

    /// End timestamp (ms) filter (optional)
    #[serde(
        rename = "createdAtTimestamp[lte]",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_timestamp_lte: Option<u64>,

    /// Pagination parameters (flattened)
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

impl RestClient {
    /// Get Historical Trades (v1)
    ///
    /// Retrieves historical trades for a trading account, supporting cursor pagination.
    pub async fn get_trades_history(
        &mut self,
        params: GetTradesHistoryParams,
    ) -> RestResult<PaginatedResult<HistoryTrade>> {
        let wire: DataOrPaginated<HistoryTrade> = self
            .send_get_authenticated_request(
                TRADES_HISTORY_ENDPOINT,
                params,
                EndpointType::PrivateTrades,
            )
            .await?;

        Ok(PaginatedResult::from(wire))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_trades_history_query_serialization() {
        let params = GetTradesHistoryParams {
            trading_account_id: "111000000000001".to_string(),
            symbol: Some("BTCUSDC".to_string()),
            order_id: Some("390755652232282113".to_string()),
            trade_id: Some("555555".to_string()),
            created_at_datetime_gte: Some("2025-05-20T01:01:01.000Z".to_string()),
            created_at_datetime_lte: Some("2025-05-21T01:01:01.000Z".to_string()),
            created_at_timestamp_gte: Some(1700000000000),
            created_at_timestamp_lte: Some(1700000100000),
            pagination: PaginationParams {
                page_size: Some(50),
                meta_data: Some(true),
                next_page: None,
                previous_page: None,
            },
        };

        let qs = serde_urlencoded::to_string(&params).unwrap();
        assert!(qs.contains("tradingAccountId=111000000000001"));
        assert!(qs.contains("symbol=BTCUSDC"));
        assert!(qs.contains("orderId=390755652232282113"));
        assert!(qs.contains("tradeId=555555"));
        assert!(qs.contains("createdAtDatetime%5Bgte%5D=2025-05-20T01%3A01%3A01.000Z"));
        assert!(qs.contains("createdAtDatetime%5Blte%5D=2025-05-21T01%3A01%3A01.000Z"));
        assert!(qs.contains("createdAtTimestamp%5Bgte%5D=1700000000000"));
        assert!(qs.contains("createdAtTimestamp%5Blte%5D=1700000100000"));
        assert!(qs.contains("_pageSize=50"));
        assert!(qs.contains("_metaData=true"));
    }

    #[test]
    fn test_history_trade_deserialization() {
        let json = r#"{
            "tradeId": "123456789",
            "orderId": "987654321",
            "symbol": "BTCUSDC",
            "price": "30000.0",
            "quantity": "1.0",
            "quoteAmount": "30000.0",
            "baseFee": "0.001",
            "quoteFee": "30.0",
            "side": "BUY",
            "isTaker": true,
            "tradeRebateAmount": "0.0",
            "tradeRebateAssetSymbol": "USDC",
            "createdAtDatetime": "2022-01-01T00:00:00.000Z",
            "createdAtTimestamp": 1640995200000
        }"#;

        let trade: HistoryTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.trade_id, "123456789");
        assert_eq!(trade.symbol, "BTCUSDC");
        assert_eq!(trade.side, OrderSide::Buy);
        assert!(trade.is_taker);
        assert_eq!(trade.created_at_timestamp, 1640995200000);
    }
}
