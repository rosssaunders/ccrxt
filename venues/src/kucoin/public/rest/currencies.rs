use serde::{Deserialize, Serialize};

use crate::kucoin::{ResponseHeaders, RestResponse, Result};

use super::RestClient;

/// Request for getting currency information
#[derive(Debug, Clone, Serialize)]
pub struct GetCurrencyRequest {
    /// Currency code (e.g., "BTC")
    pub currency: String,
}

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
    /// Get information for a specific currency
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::public::{RestClient, GetCurrencyRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_default();
    ///     let request = GetCurrencyRequest {
    ///         currency: "BTC".to_string(),
    ///     };
    ///     let (response, _headers) = client.get_currency(request).await?;
    ///     println!("Currency: {}", response.name);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_currency(
        &self,
        request: GetCurrencyRequest,
    ) -> Result<(Currency, ResponseHeaders)> {
        let endpoint = format!("/api/v1/currencies/{}", request.currency);

        let (response, headers): (RestResponse<Currency>, ResponseHeaders) =
            self.get(&endpoint, None).await?;

        Ok((response.data, headers))
    }

    /// Get information for all available currencies
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::public::{RestClient, GetAllCurrenciesRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_default();
    ///     let request = GetAllCurrenciesRequest::default();
    ///     let (response, _headers) = client.get_all_currencies(request).await?;
    ///     println!("Found {} currencies", response.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_all_currencies(
        &self,
        _request: GetAllCurrenciesRequest,
    ) -> Result<(Vec<Currency>, ResponseHeaders)> {
        let (response, headers): (RestResponse<Vec<Currency>>, ResponseHeaders) =
            self.get("/api/v1/currencies", None).await?;

        Ok((response.data, headers))
    }
}
