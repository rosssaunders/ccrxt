use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result, private_client::RestClient};

/// Endpoint URL for loan market interest rate (past 7 days)
const GET_LOAN_MARKET_INTEREST_RATE_ENDPOINT: &str = "/api/v3/project/marketInterestRate";

/// Market interest rate data point.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketInterestRate {
    /// Time in the format YYYYMMDDHH00.
    pub time: String,

    /// Market lending rate for the period.
    pub market_interest_rate: String,
}

/// Request parameters for getting loan market interest rates.
#[derive(Debug, Clone, Serialize)]
pub struct GetLoanMarketInterestRateRequest {
    /// Currency code (e.g., "USDT"). Required.
    pub currency: String,
}

impl RestClient {
    /// Get Loan Market Interest Rate
    ///
    /// Get the interest rates of the margin lending market over the past 7 days.
    ///
    /// - [docs](https://www.kucoin.com/docs-new/rest/margin-trading/credit/get-loan-market-interest-rate)
    ///
    /// Rate limit: weight 5 (Public)
    ///
    /// # Arguments
    /// * `request` - The request containing the target currency
    ///
    /// # Returns
    /// List of market interest rates for the past 7 days and response headers
    pub async fn get_loan_market_interest_rate(
        &self,
        request: GetLoanMarketInterestRateRequest,
    ) -> Result<(Vec<MarketInterestRate>, ResponseHeaders)> {
        let (response, headers): (RestResponse<Vec<MarketInterestRate>>, ResponseHeaders) = self
            .get_with_request(GET_LOAN_MARKET_INTEREST_RATE_ENDPOINT, &request)
            .await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(
            GET_LOAN_MARKET_INTEREST_RATE_ENDPOINT,
            "/api/v3/project/marketInterestRate"
        );
    }

    #[test]
    fn test_request_serialization() {
        let req = GetLoanMarketInterestRateRequest {
            currency: "USDT".to_string(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["currency"], "USDT");
    }

    #[test]
    fn test_response_deserialization_sample() {
        let json = r#"{
            "code": "200000",
            "data": [
                {"time": "202410170000", "marketInterestRate": "0.005"},
                {"time": "202410170100", "marketInterestRate": "0.006"}
            ]
        }"#;

        let resp: RestResponse<Vec<MarketInterestRate>> = serde_json::from_str(json).unwrap();
        assert_eq!(resp.code, "200000");
        assert_eq!(resp.data.len(), 2);
        assert_eq!(resp.data[0].time, "202410170000");
        assert_eq!(resp.data[0].market_interest_rate, "0.005");
    }
}
