use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{RestResult, SelfTradePreventionMode};

/// Request parameters for getting prevented matches
#[derive(Debug, Clone, Serialize)]
pub struct MyPreventedMatchesRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Prevented match ID
    #[serde(rename = "preventedMatchId", skip_serializing_if = "Option::is_none")]
    pub prevented_match_id: Option<u64>,

    /// Order ID
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// From prevented match ID
    #[serde(
        rename = "fromPreventedMatchId",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_prevented_match_id: Option<u64>,

    /// Default 500; max 1000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Prevented match information
#[derive(Debug, Clone, Deserialize)]
pub struct MyPreventedMatch {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Prevented match ID
    #[serde(rename = "preventedMatchId")]
    pub prevented_match_id: u64,

    /// Taker order ID
    #[serde(rename = "takerOrderId")]
    pub taker_order_id: u64,

    /// Maker symbol
    #[serde(rename = "makerSymbol")]
    pub maker_symbol: String,

    /// Maker order ID
    #[serde(rename = "makerOrderId")]
    pub maker_order_id: u64,

    /// Trade group ID
    #[serde(rename = "tradeGroupId")]
    pub trade_group_id: u64,

    /// Self-trade prevention mode
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,

    /// Match price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Maker prevented quantity
    #[serde(rename = "makerPreventedQuantity")]
    pub maker_prevented_quantity: Decimal,

    /// Taker prevented quantity
    #[serde(rename = "takerPreventedQuantity")]
    pub taker_prevented_quantity: Decimal,

    /// Transaction time
    #[serde(rename = "transactTime")]
    pub transact_time: u64,
}

impl RestClient {
    /// Display orders that were expired due to STP
    ///
    /// Display orders that were expired due to STP (Self-Trade Prevention).
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#query-prevented-matches--user_data)
    /// Method: GET /api/v3/myPreventedMatches
    /// Weight: 20
    /// Security: USER_DATA
    pub async fn get_my_prevented_matches(
        &self,
        params: MyPreventedMatchesRequest,
    ) -> RestResult<Vec<MyPreventedMatch>> {
        let query_string = serde_urlencoded::to_string(&params)
            .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {e}")))?;

        self.send_request(
            "/api/v3/myPreventedMatches",
            reqwest::Method::GET,
            Some(&query_string),
            None,
            20,
            false,
        )
        .await
    }
}
