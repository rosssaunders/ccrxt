use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::enums::OrderSide;
use crate::bullish::{
    DataOrPaginated, EndpointType, PaginatedResult, PaginationParams, RestResult,
};

/// Endpoint URL for historical derivatives settlement operations
const DERIVATIVES_SETTLEMENT_HISTORY_ENDPOINT: &str = "/v1/history/derivatives-settlement";

/// Historical derivatives settlement response item
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DerivativesSettlementResponse {
    /// unique trading account ID
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: Option<String>,

    /// market symbol, e.g., BTC-USDC-PERP or BTC-USDC-20241201
    pub symbol: Option<String>,

    /// side
    pub side: Option<OrderSide>,

    /// position size at the time of the settlement
    #[serde(rename = "settlementQuantity")]
    pub settlement_quantity: Option<String>,

    /// change in position size from trading activities
    #[serde(rename = "deltaTradingQuantity")]
    pub delta_trading_quantity: Option<String>,

    /// mark to market PnL accumulated since the last settlement
    #[serde(rename = "mtmPnl")]
    pub mtm_pnl: Option<String>,

    /// funding PnL accumulated since the last settlement (perpetuals only)
    #[serde(rename = "fundingPnl")]
    pub funding_pnl: Option<String>,

    /// derivatives position update event type
    #[serde(rename = "eventType")]
    pub event_type: Option<String>,

    /// market price at which the position was settled for this cycle
    #[serde(rename = "settlementMarkPrice")]
    pub settlement_mark_price: Option<String>,
}

/// Parameters for querying historical derivatives settlement
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetDerivativesSettlementHistoryParams {
    /// Trading account ID (required for users with multiple trading accounts)
    #[serde(rename = "tradingAccountId", skip_serializing_if = "Option::is_none")]
    pub trading_account_id: Option<String>,

    /// Market symbol filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Start ISO8601 datetime filter (required)
    #[serde(rename = "settlementDatetime[gte]")]
    pub settlement_datetime_gte: String,

    /// End ISO8601 datetime filter (required)
    #[serde(rename = "settlementDatetime[lte]")]
    pub settlement_datetime_lte: String,

    /// Pagination parameters (flattened)
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

impl RestClient {
    /// Get Historical Hourly Derivatives Settlement (v1)
    pub async fn get_derivatives_settlement_history(
        &mut self,
        params: GetDerivativesSettlementHistoryParams,
    ) -> RestResult<PaginatedResult<DerivativesSettlementResponse>> {
        let wire: DataOrPaginated<DerivativesSettlementResponse> = self
            .send_get_authenticated_request(
                DERIVATIVES_SETTLEMENT_HISTORY_ENDPOINT,
                params,
                EndpointType::PrivatePositions,
            )
            .await?;

        Ok(PaginatedResult::from(wire))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derivatives_settlement_query_serialization() {
        let params = GetDerivativesSettlementHistoryParams {
            trading_account_id: Some("111000000000001".to_string()),
            symbol: Some("BTC-USDC-PERP".to_string()),
            settlement_datetime_gte: "2025-05-20T01:01:01.000Z".to_string(),
            settlement_datetime_lte: "2025-05-20T02:01:01.000Z".to_string(),
            pagination: PaginationParams {
                page_size: Some(5),
                meta_data: Some(true),
                next_page: Some("cursorNext".to_string()),
                previous_page: None,
            },
        };

        let qs = serde_urlencoded::to_string(&params).unwrap();
        assert!(qs.contains("tradingAccountId=111000000000001"));
        assert!(qs.contains("symbol=BTC-USDC-PERP"));
        assert!(qs.contains("settlementDatetime%5Bgte%5D=2025-05-20T01%3A01%3A01.000Z"));
        assert!(qs.contains("settlementDatetime%5Blte%5D=2025-05-20T02%3A01%3A01.000Z"));
        assert!(qs.contains("_pageSize=5"));
        assert!(qs.contains("_metaData=true"));
        assert!(qs.contains("_nextPage=cursorNext"));
    }
}
