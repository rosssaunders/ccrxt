use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Market interest rate data point
#[derive(Debug, Clone, Deserialize)]
pub struct MarketInterestRate {
    /// Time: YYYYMMDDHH00
    pub time: String,

    /// Market lending rate
    #[serde(rename = "marketInterestRate")]
    pub market_interest_rate: String,
}

/// Request for getting loan market interest rates
#[derive(Debug, Clone, Serialize)]
pub struct GetLoanMarketInterestRateRequest {
    /// Currency (required)
    pub currency: String,
}

impl RestClient {
    /// Get the interest rates of the margin lending market over the past 7 days
    ///
    /// [docs](https://docs.kucoin.com/margin-credit#get-loan-market-interest-rate)
    pub async fn get_loan_market_interest_rate(
        &self,
        request: GetLoanMarketInterestRateRequest,
    ) -> Result<(Vec<MarketInterestRate>, ResponseHeaders)> {
        let (response, headers): (RestResponse<Vec<MarketInterestRate>>, ResponseHeaders) = self
            .get_with_request("/api/v3/project/marketInterestRate", &request)
            .await?;

        Ok((response.data, headers))
    }
}
