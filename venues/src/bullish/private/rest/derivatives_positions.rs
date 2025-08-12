//! Derivatives positions endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult, enums::OrderSide};

/// Endpoint URL path for derivatives positions
const DERIVATIVES_POSITIONS_ENDPOINT: &str = "/v1/derivatives-positions";

/// Query params for derivatives positions
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDerivativesPositionsParams {
    /// Id of the trading account. Mandatory if the user has multiple trading accounts.
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,

    /// Market symbol filter (e.g. BTC-USDC-PERP)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Derivatives Position of one market for the trading account
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DerivativesPosition {
    /// unique trading account ID
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,

    /// market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market
    pub symbol: String,

    /// BUY or SELL
    pub side: OrderSide,

    /// Current size of the position
    pub quantity: String,

    /// Notional value of the current position, calculated using the mark price
    pub notional: String,

    /// Notional value of the position, using the average entry price
    #[serde(rename = "entryNotional")]
    pub entry_notional: String,

    /// Mark-to-market PnL accumulated since the last settlement
    #[serde(rename = "mtmPnl")]
    pub mtm_pnl: String,

    /// Profit/loss from net price change since last absolute quantity decrease
    #[serde(rename = "reportedMtmPnl")]
    pub reported_mtm_pnl: String,

    /// Sum of all funding payments received since the position was opened
    #[serde(rename = "reportedFundingPnl")]
    pub reported_funding_pnl: String,

    /// Total profits realized since first opening this position
    #[serde(rename = "realizedPnl")]
    pub realized_pnl: String,

    /// Settlement Asset Symbol (e.g., USDC)
    #[serde(rename = "settlementAssetSymbol")]
    pub settlement_asset_symbol: String,

    /// Position created time (ISO 8601 with millisecond)
    #[serde(rename = "createdAtDatetime")]
    pub created_at_datetime: String,

    /// Position created time in milliseconds since EPOCH (as string per API)
    #[serde(rename = "createdAtTimestamp")]
    pub created_at_timestamp: String,

    /// Position last updated time (ISO 8601 with millisecond)
    #[serde(rename = "updatedAtDatetime")]
    pub updated_at_datetime: String,

    /// Position last updated time in milliseconds since EPOCH (as string per API)
    #[serde(rename = "updatedAtTimestamp")]
    pub updated_at_timestamp: String,
}

impl RestClient {
    /// Get derivatives positions
    ///
    /// Retrieve derivatives positions for a trading account with an optional symbol filter.
    ///
    /// [docs]: https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/derivatives-positions
    ///
    /// # Arguments
    /// * `params` - Query parameters
    ///
    /// # Returns
    /// Vector of derivatives positions
    pub async fn get_derivatives_positions(
        &mut self,
        params: GetDerivativesPositionsParams,
    ) -> RestResult<Vec<DerivativesPosition>> {
        self.send_get_authenticated_request(
            DERIVATIVES_POSITIONS_ENDPOINT,
            params,
            EndpointType::PrivatePositions,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_params_query_serialization() {
        let params = GetDerivativesPositionsParams {
            trading_account_id: "111000000000001".to_string(),
            symbol: Some("BTC-USDC-PERP".to_string()),
        };

        let qs = serde_urlencoded::to_string(&params).unwrap();
        assert!(qs.contains("tradingAccountId=111000000000001"));
        assert!(qs.contains("symbol=BTC-USDC-PERP"));
    }

    #[test]
    fn test_position_deserialization() {
        let json = r#"{
            "tradingAccountId": "111000000000001",
            "symbol": "BTC-USDC-PERP",
            "side": "BUY",
            "quantity": "1.00000000",
            "notional": "65000.00",
            "entryNotional": "64000.00",
            "mtmPnl": "1000.00",
            "reportedMtmPnl": "500.00",
            "reportedFundingPnl": "-10.00",
            "realizedPnl": "200.00",
            "settlementAssetSymbol": "USDC",
            "createdAtDatetime": "2025-05-20T01:01:01.000Z",
            "createdAtTimestamp": "1621490985000",
            "updatedAtDatetime": "2025-05-21T01:01:01.000Z",
            "updatedAtTimestamp": "1621577385000"
        }"#;

        let pos: DerivativesPosition = serde_json::from_str(json).unwrap();
        assert_eq!(pos.trading_account_id, "111000000000001");
        assert_eq!(pos.symbol, "BTC-USDC-PERP");
        assert_eq!(pos.side, OrderSide::Buy);
        assert_eq!(pos.settlement_asset_symbol, "USDC");
        assert_eq!(pos.created_at_timestamp, "1621490985000");
    }
}
