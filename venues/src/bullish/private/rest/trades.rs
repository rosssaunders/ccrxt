//! Trades endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use crate::bullish::{EndpointType, PrivateRestClient as RestClient, RestResult, enums::*};

/// Endpoint URL path for trades
const TRADES_ENDPOINT: &str = "/v1/trades";

/// Endpoint URL path for single trade (with parameter)
const SINGLE_TRADE_ENDPOINT: &str = "/v1/trades/{}";

/// Trade execution details (matches Bullish API spec)
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
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

    /// Trade execution timestamp (ms since epoch, as string)
    #[serde(rename = "createdAtTimestamp")]
    pub created_at_timestamp: String,
}

/// Parameters for querying trades
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetTradesParams {
    /// Trading account ID (required)
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,

    /// Market symbol filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Order ID filter
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Trade side filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,

    /// Start time filter (timestamp)
    #[serde(
        rename = "createdAtTimestamp[gte]",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_timestamp_gte: Option<u64>,

    /// End time filter (timestamp)
    #[serde(
        rename = "createdAtTimestamp[lte]",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at_timestamp_lte: Option<u64>,

    /// Pagination
    #[serde(flatten)]
    pub pagination: crate::bullish::PaginationParams,
}

impl RestClient {
    /// Get trades with optional filters (paginated)
    ///
    /// Retrieve a list of trade executions for a trading account.
    /// Only the last 24 hours of data is available for querying.
    ///
    /// # Arguments
    /// * `params` - Query parameters for filtering trades
    ///
    /// # Returns
    /// Paginated response of trades matching the filter criteria
    pub async fn get_trades(
        &mut self,
        params: GetTradesParams,
    ) -> RestResult<crate::bullish::ListResponse<Trade>> {
        self.send_get_authenticated_request(TRADES_ENDPOINT, params, EndpointType::PrivateTrades)
            .await
    }

    /// Get specific trade by ID
    ///
    /// Retrieve details for a specific trade by its trade ID.
    ///
    /// # Arguments
    /// * `trade_id` - The trade ID to retrieve
    /// * `trading_account_id` - Trading account ID
    ///
    /// # Returns
    /// Trade details
    pub async fn get_trade(
        &mut self,
        trade_id: &str,
        trading_account_id: &str,
    ) -> RestResult<Trade> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(rename = "tradingAccountId")]
            trading_account_id: &'a str,
        }

        let url = SINGLE_TRADE_ENDPOINT.replace("{}", trade_id);
        self.send_get_authenticated_request(
            &url,
            Query { trading_account_id },
            EndpointType::PrivateTrades,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_trades_params_default() {
        let params = GetTradesParams::default();
        assert!(params.trading_account_id.is_empty());
        assert!(params.symbol.is_none());
        assert!(params.order_id.is_none());
        assert!(params.side.is_none());
        assert!(params.created_at_timestamp_gte.is_none());
        assert!(params.created_at_timestamp_lte.is_none());
    }

    #[test]
    fn test_get_trades_query_serialization() {
        let params = GetTradesParams {
            trading_account_id: "acc-1".into(),
            symbol: Some("BTCUSDC".into()),
            order_id: Some("ord-1".into()),
            side: Some(OrderSide::Buy),
            created_at_timestamp_gte: Some(1700000000000),
            created_at_timestamp_lte: Some(1700000100000),
            pagination: crate::bullish::PaginationParams {
                page_size: Some(25),
                meta_data: Some(true),
                next_page: None,
                previous_page: None,
            },
        };
        let qs = serde_urlencoded::to_string(&params).unwrap();
        assert!(qs.contains("tradingAccountId=acc-1"));
        assert!(qs.contains("symbol=BTCUSDC"));
        assert!(qs.contains("orderId=ord-1"));
        assert!(qs.contains("side=BUY"));
        assert!(qs.contains("createdAtTimestamp%5Bgte%5D=1700000000000"));
        assert!(qs.contains("createdAtTimestamp%5Blte%5D=1700000100000"));
        assert!(qs.contains("_pageSize=25"));
        assert!(qs.contains("_metaData=true"));
    }

    #[test]
    fn test_trade_deserialization() {
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
            "createdAtTimestamp": "1640995200000"
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.trade_id, "123456789");
        assert_eq!(trade.symbol, "BTCUSDC");
        assert_eq!(trade.side, OrderSide::Buy);
        assert!(trade.is_taker);
        assert_eq!(trade.trade_rebate_asset_symbol, "USDC");
        assert_eq!(trade.created_at_timestamp, "1640995200000");
    }
}
