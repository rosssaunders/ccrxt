use serde::Serialize;

use crate::bullish::{
    DataOrPaginated, EndpointType, PaginatedResult, PaginationParams,
    PrivateRestClient as RestClient, RestResult,
};

/// Endpoint URL for AMM instructions
const AMM_INSTRUCTIONS_ENDPOINT: &str = "/v2/amm-instructions";

/// AMM Instruction list request params
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAmmInstructionsParams {
    /// Trading account ID (required)
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,

    /// Optional market symbol filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Optional status filter (e.g., OPEN, CLOSED). Keep as string for forward-compat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Pagination controls
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

impl RestClient {
    /// Get AMM Instructions (list)
    ///
    /// Returns active and/or historical AMM instructions for a trading account, optionally filtered by symbol.
    /// Supports Bullish-style cursor pagination via PaginationParams.
    ///
    /// [docs](https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v2/amm-instructions)
    pub async fn get_amm_instructions(
        &mut self,
        params: GetAmmInstructionsParams,
    ) -> RestResult<PaginatedResult<super::amm_types::AmmInstruction>> {
        let wire: DataOrPaginated<super::amm_types::AmmInstruction> = self
            .send_get_authenticated_request(
                AMM_INSTRUCTIONS_ENDPOINT,
                params,
                EndpointType::PrivateOrders,
            )
            .await?;

        Ok(PaginatedResult::from(wire))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_amm_instructions_params_query_serialization() {
        let params = GetAmmInstructionsParams {
            trading_account_id: "acc-1".into(),
            symbol: Some("BTCUSDC".into()),
            status: None,
            pagination: PaginationParams {
                page_size: Some(50),
                meta_data: Some(true),
                next_page: Some("cursor123".into()),
                previous_page: None,
            },
        };
        let qs = serde_urlencoded::to_string(&params).unwrap();
        assert!(qs.contains("tradingAccountId=acc-1"));
        assert!(qs.contains("symbol=BTCUSDC"));
        assert!(qs.contains("_pageSize=50"));
        assert!(qs.contains("_metaData=true"));
        assert!(qs.contains("_nextPage=cursor123"));
    }
}
