//! Trades endpoint for Bullish Exchange API

use serde::Deserialize;

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult, enums::*};

/// Trade execution details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    /// Unique trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: String,
    /// Order ID associated with this trade
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    /// Market symbol
    pub symbol: String,
    /// Execution price
    pub price: String,
    /// Executed quantity
    pub quantity: String,
    /// Quote amount
    #[serde(rename = "quoteAmount")]
    pub quote_amount: String,
    /// Trade side
    pub side: OrderSide,
    /// Base asset fee
    #[serde(rename = "baseFee")]
    pub base_fee: String,
    /// Quote asset fee
    #[serde(rename = "quoteFee")]
    pub quote_fee: String,
    /// Whether this was a liquidation trade
    #[serde(rename = "isLiquidation")]
    pub is_liquidation: bool,
    /// Whether this was a market maker trade
    #[serde(rename = "isMaker")]
    pub is_maker: bool,
    /// Trade execution timestamp
    #[serde(rename = "createdAtTimestamp")]
    pub created_at_timestamp: u64,
    /// Trade execution datetime
    #[serde(rename = "createdAtDatetime")]
    pub created_at_datetime: String,
}

/// Parameters for querying trades
#[derive(Debug, Clone, Default)]
pub struct GetTradesParams {
    /// Trading account ID (required)
    pub trading_account_id: String,
    /// Market symbol filter
    pub symbol: Option<String>,
    /// Order ID filter
    pub order_id: Option<String>,
    /// Trade side filter
    pub side: Option<OrderSide>,
    /// Start time filter (timestamp)
    pub start_time: Option<u64>,
    /// End time filter (timestamp)
    pub end_time: Option<u64>,
}

impl RestClient {
    /// Get trades with optional filters
    ///
    /// Retrieve a list of trade executions for a trading account.
    /// Only the last 24 hours of data is available for querying.
    ///
    /// # Arguments
    /// * `params` - Query parameters for filtering trades
    ///
    /// # Returns
    /// List of trades matching the filter criteria
    pub async fn get_trades(&mut self, params: GetTradesParams) -> RestResult<Vec<Trade>> {
        let mut query_params = vec![("tradingAccountId", params.trading_account_id)];

        if let Some(symbol) = params.symbol {
            query_params.push(("symbol", symbol));
        }
        if let Some(order_id) = params.order_id {
            query_params.push(("orderId", order_id));
        }
        if let Some(side) = params.side {
            query_params.push(("side", format!("{:?}", side).to_uppercase()));
        }
        if let Some(start_time) = params.start_time {
            query_params.push(("startTime", start_time.to_string()));
        }
        if let Some(end_time) = params.end_time {
            query_params.push(("endTime", end_time.to_string()));
        }

        let mut url = "/v1/trades".to_string();
        if !query_params.is_empty() {
            url.push('?');
            let query_string: Vec<String> = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            url.push_str(&query_string.join("&"));
        }

        self.send_authenticated_request(
            &url,
            reqwest::Method::GET,
            None::<&()>,
            EndpointType::PrivateTrades,
        )
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
        let url = format!(
            "/v1/trades/{}?tradingAccountId={}",
            trade_id, trading_account_id
        );

        self.send_authenticated_request(
            &url,
            reqwest::Method::GET,
            None::<&()>,
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
        assert!(params.start_time.is_none());
        assert!(params.end_time.is_none());
    }

    #[test]
    fn test_trade_deserialization() {
        let json = r#"{
            "tradeId": "123456789",
            "orderId": "987654321",
            "clientOrderId": "client123",
            "symbol": "BTCUSDC",
            "price": "30000.0",
            "quantity": "1.0",
            "quoteAmount": "30000.0",
            "side": "BUY",
            "baseFee": "0.001",
            "quoteFee": "30.0",
            "isLiquidation": false,
            "isMaker": true,
            "createdAtTimestamp": 1640995200000,
            "createdAtDatetime": "2022-01-01T00:00:00Z"
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.trade_id, "123456789");
        assert_eq!(trade.symbol, "BTCUSDC");
        assert_eq!(trade.side, OrderSide::Buy);
        assert!(trade.is_maker);
        assert!(!trade.is_liquidation);
    }
}
