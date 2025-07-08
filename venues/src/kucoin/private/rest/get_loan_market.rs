use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::{ResponseHeaders, RestResponse, Result};

const LOAN_MARKET_ENDPOINT: &str = "/api/v3/project/list";

/// Response data for loan market information
#[derive(Debug, Clone, Deserialize)]
pub struct LoanMarket {
    /// Currency
    pub currency: Option<String>,
    /// Whether purchase is supported
    #[serde(rename = "purchaseEnable")]
    pub purchase_enable: Option<bool>,
    /// Whether redeem is supported
    #[serde(rename = "redeemEnable")]
    pub redeem_enable: Option<bool>,
    /// Increment precision for purchase and redemption
    pub increment: Option<String>,
    /// Minimum purchase amount
    #[serde(rename = "minPurchaseSize")]
    pub min_purchase_size: Option<String>,
    /// Minimum lending rate
    #[serde(rename = "minInterestRate")]
    pub min_interest_rate: Option<String>,
    /// Maximum lending rate
    #[serde(rename = "maxInterestRate")]
    pub max_interest_rate: Option<String>,
    /// Increment precision for interest; default is 0.0001
    #[serde(rename = "interestIncrement")]
    pub interest_increment: Option<String>,
    /// Maximum purchase amount
    #[serde(rename = "maxPurchaseSize")]
    pub max_purchase_size: Option<String>,
    /// Latest market lending rate
    #[serde(rename = "marketInterestRate")]
    pub market_interest_rate: Option<String>,
    /// Whether to allow automatic purchase: True: on; false: off
    #[serde(rename = "autoPurchaseEnable")]
    pub auto_purchase_enable: Option<bool>,
}

/// Request for getting loan market information
#[derive(Debug, Clone, Serialize)]
pub struct GetLoanMarketRequest {
    /// Currency (optional)
    pub currency: Option<String>,
}

impl RestClient {
    /// Get the information about the currencies available for lending
    ///
    /// Reference: https://docs.kucoin.com/margin-credit#get-loan-market
    pub async fn get_loan_market(
        &self,
        request: GetLoanMarketRequest,
    ) -> Result<(Vec<LoanMarket>, ResponseHeaders)> {
        let mut params = std::collections::HashMap::new();
        if let Some(currency) = &request.currency {
            params.insert("currency".to_string(), currency.clone());
        }

        let params = if params.is_empty() {
            None
        } else {
            Some(params)
        };

        let (response, headers): (RestResponse<Vec<LoanMarket>>, ResponseHeaders) =
            self.get(LOAN_MARKET_ENDPOINT, params).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loan_market_request_empty() {
        let request = GetLoanMarketRequest { currency: None };
        assert!(request.currency.is_none());
    }

    #[test]
    fn test_get_loan_market_request_with_currency() {
        let request = GetLoanMarketRequest {
            currency: Some("BTC".to_string()),
        };
        assert_eq!(request.currency, Some("BTC".to_string()));
    }
}
