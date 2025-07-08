use serde::{Deserialize, Serialize};

use crate::kucoin::{ResponseHeaders, RestResponse, Result};

use super::RestClient;

const ALL_CURRENCIES_ENDPOINT: &str = "/api/v1/currencies";

/// Request for getting all currencies
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAllCurrenciesRequest {}

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
    /// Get information for all available currencies
    ///
    /// Reference: https://docs.kucoin.com/#get-currencies
    pub async fn get_all_currencies(
        &self,
        _request: GetAllCurrenciesRequest,
    ) -> Result<(Vec<Currency>, ResponseHeaders)> {
        let (response, headers): (RestResponse<Vec<Currency>>, ResponseHeaders) =
            self.get(ALL_CURRENCIES_ENDPOINT, None).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_currencies_request_default() {
        let request = GetAllCurrenciesRequest::default();
        // Just verify it can be created
        assert_eq!(format!("{:?}", request), "GetAllCurrenciesRequest");
    }
}
