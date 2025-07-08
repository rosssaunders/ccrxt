use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, private::rest::client::RestClient},
    shared,
};

/// Request parameters for notional and leverage brackets.
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotionalBracketRequest {
    /// Trading pair, e.g. BTCUSD_PERP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,

    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Bracket information for notional brackets.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotionalBracket {
    /// Bracket
    pub bracket: u32,

    /// Initial leverage
    pub initial_leverage: u32,

    /// Max notional value of the bracket
    pub notional_cap: String,

    /// Min notional value of the bracket
    pub notional_floor: String,

    /// Maintenance margin ratio
    pub maint_margin_ratio: String,

    /// Auxiliary number for quick calculation
    pub cum: String,
}

/// Response for notional and leverage brackets.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotionalBracketResponse {
    /// Trading pair
    pub pair: String,

    /// Bracket information
    pub brackets: Vec<NotionalBracket>,
}

impl RestClient {
    /// Get notional and leverage brackets on Binance Coin-M Futures.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
    /// GET /dapi/v1/leverageBracket
    /// Weight: 1 if pair provided, 40 otherwise
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`NotionalBracketRequest`])
    ///
    /// # Returns
    /// A list of [`NotionalBracketResponse`] objects with bracket information.
    pub async fn get_notional_brackets(
        &self,
        params: NotionalBracketRequest,
    ) -> RestResult<Vec<NotionalBracketResponse>> {
        let weight = if params.pair.is_some() { 1 } else { 40 };
        shared::send_signed_request(
            self,
            "/dapi/v1/leverageBracket",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
