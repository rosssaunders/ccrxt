use serde::{Deserialize, Serialize};

use crate::kucoin::{ResponseHeaders, RestResponse, Result};

use super::RestClient;

const CURRENCY_ENDPOINT: &str = "/api/v1/currencies/{currency}";

/// Request for getting currency information
#[derive(Debug, Clone, Serialize)]
pub struct GetCurrencyRequest {
    /// Currency code (e.g., "BTC")
    pub currency: String,
}

/// Currency information
#[derive(Debug, Clone, Deserialize)]
pub struct Currency {
    /// Currency code
    pub currency: String,

    /// Currency name
    pub name: String,

    /// Full name
    #[serde(rename = "fullName")]
    pub full_name: String,

    /// Precision
    pub precision: i32,

    /// Confirms required for deposit
    pub confirms: Option<i32>,

    /// Contract address (for tokens)
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<String>,

    /// Whether currency is marginable
    #[serde(rename = "isMarginEnabled")]
    pub is_margin_enabled: bool,

    /// Whether currency is debit enabled
    #[serde(rename = "isDebitEnabled")]
    pub is_debit_enabled: bool,
}

impl RestClient {
    /// Get information for a specific currency
    ///
    /// Reference: https://docs.kucoin.com/#get-currency-detail
    pub async fn get_currency(
        &self,
        request: GetCurrencyRequest,
    ) -> Result<(Currency, ResponseHeaders)> {
        let endpoint = CURRENCY_ENDPOINT.replace("{currency}", &request.currency);

        let (response, headers): (RestResponse<Currency>, ResponseHeaders) =
            self.get(&endpoint, None).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_request_creation() {
        let request = GetCurrencyRequest {
            currency: "BTC".to_string(),
        };
        assert_eq!(request.currency, "BTC");
    }
}
