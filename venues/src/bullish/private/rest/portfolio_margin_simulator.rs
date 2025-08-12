// Portfolio Margin Simulator endpoint implementation for Bullish
// See: https://api.exchange.bullish.com/trading-api/v1/simulate-portfolio-margin

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::bullish::private::rest::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

const SIMULATE_PORTFOLIO_MARGIN_ENDPOINT: &str = "/trading-api/v1/simulate-portfolio-margin";

/// Request parameters for the portfolio margin simulator endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimulatePortfolioMarginRequest {
    /// If true, include existing portfolio positions in the simulation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_existing: Option<bool>,

    /// Simulation input body. Optional, may be empty for current portfolio.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
}

/// Response from the portfolio margin simulator endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulatePortfolioMarginResponse {
    /// Description of the simulation result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'static, str>>,
}

impl RestClient {
    /// Portfolio Margin Simulator
    ///
    /// Use Portfolio margin simulator to determine your margin requirements and risk levels based on your current portfolio balances. You can also append position details on top of your portfolio specifics to see simulated results.
    ///
    /// [docs]: https://api.exchange.bullish.com/trading-api/v1/simulate-portfolio-margin
    ///
    /// Rate limit: see Bullish API docs
    ///
    /// # Arguments
    /// * `request` - The simulation request parameters
    ///
    /// # Returns
    /// Simulation result
    pub async fn simulate_portfolio_margin(
        &mut self,
        request: SimulatePortfolioMarginRequest,
    ) -> RestResult<SimulatePortfolioMarginResponse> {
        self.send_post_request(
            SIMULATE_PORTFOLIO_MARGIN_ENDPOINT,
            Some(&request),
            EndpointType::PrivateOther,
        )
        .await
    }
}
