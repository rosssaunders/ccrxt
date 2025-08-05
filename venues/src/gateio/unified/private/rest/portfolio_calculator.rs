use serde::{Deserialize, Serialize};

use super::RestClient;

const PORTFOLIO_CALCULATOR_ENDPOINT: &str = "/unified/portfolio_calculator";

/// Portfolio calculator request
#[derive(Debug, Clone, Serialize)]
pub struct PortfolioCalculatorRequest {
    /// Spot balances
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_balances: Option<Vec<BalanceEntry>>,

    /// Futures positions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub futures_positions: Option<Vec<PositionEntry>>,

    /// Options positions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options_positions: Option<Vec<PositionEntry>>,
}

/// Balance entry for portfolio calculation
#[derive(Debug, Clone, Serialize)]
pub struct BalanceEntry {
    /// Currency
    pub currency: String,

    /// Amount
    pub amount: String,
}

/// Position entry for portfolio calculation
#[derive(Debug, Clone, Serialize)]
pub struct PositionEntry {
    /// Contract
    pub contract: String,

    /// Size
    pub size: String,
}

/// Portfolio calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioCalculationResult {
    /// Total balance
    pub total_balance: String,

    /// Total margin
    pub total_margin: String,

    /// Available margin
    pub available_margin: String,

    /// Risk level
    pub risk_level: String,

    /// Maintenance margin
    pub maintenance_margin: String,
}

impl RestClient {
    /// Calculate portfolio metrics
    ///
    /// This endpoint calculates portfolio metrics based on provided positions.
    pub async fn portfolio_calculator(
        &self,
        request: PortfolioCalculatorRequest,
    ) -> crate::gateio::unified::RestResult<PortfolioCalculationResult> {
        self.post(PORTFOLIO_CALCULATOR_ENDPOINT, &request).await
    }
}
