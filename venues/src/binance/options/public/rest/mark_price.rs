use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

/// Request parameters for mark price
#[derive(Debug, Clone, Serialize, Default)]
pub struct MarkPriceRequest {
    /// Option trading pair, e.g BTC-200730-9000-C
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Option mark price and Greek information
#[derive(Debug, Clone, Deserialize)]
pub struct MarkPriceResponse {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Mark price
    #[serde(rename = "markPrice")]
    pub mark_price: Decimal,

    /// Implied volatility Buy
    #[serde(rename = "bidIV")]
    pub bid_iv: Decimal,

    /// Implied volatility Sell
    #[serde(rename = "askIV")]
    pub ask_iv: Decimal,

    /// Implied volatility mark
    #[serde(rename = "markIV")]
    pub mark_iv: Decimal,

    /// Delta
    #[serde(rename = "delta")]
    pub delta: Decimal,

    /// Theta
    #[serde(rename = "theta")]
    pub theta: Decimal,

    /// Gamma
    #[serde(rename = "gamma")]
    pub gamma: Decimal,

    /// Vega
    #[serde(rename = "vega")]
    pub vega: Decimal,

    /// Current highest buy price
    #[serde(rename = "highPriceLimit")]
    pub high_price_limit: Decimal,

    /// Current lowest sell price
    #[serde(rename = "lowPriceLimit")]
    pub low_price_limit: Decimal,

    /// Risk free rate
    #[serde(rename = "riskFreeInterest")]
    pub risk_free_interest: Decimal,
}

impl RestClient {
    /// Get option mark price
    ///
    /// Returns option mark price and Greek info.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-data/Option-Mark-Price)
    /// Method: GET /eapi/v1/mark
    /// Weight: 5
    /// Security: None
    pub async fn get_mark_price(
        &self,
        params: MarkPriceRequest,
    ) -> RestResult<Vec<MarkPriceResponse>> {
        let query_string = if params.symbol.is_some() {
            Some(serde_urlencoded::to_string(&params).map_err(|e| {
                crate::binance::options::Errors::Error(format!("URL encoding error: {e}"))
            })?)
        } else {
            None
        };

        self.send_request(
            "/eapi/v1/mark",
            reqwest::Method::GET,
            query_string.as_deref(),
            None,
            5,
        )
        .await
    }
}
