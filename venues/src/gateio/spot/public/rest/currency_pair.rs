use serde::{Deserialize, Serialize};

use super::RestClient;

/// Currency pair information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyPair {
    /// Currency pair ID
    pub id: String,

    /// Base currency
    pub base: String,

    /// Quote currency
    pub quote: String,

    /// Trading fee rate
    pub fee: String,

    /// Minimum base currency amount per order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_base_amount: Option<String>,

    /// Minimum quote currency amount per order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_quote_amount: Option<String>,

    /// Maximum base currency amount per order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_base_amount: Option<String>,

    /// Maximum quote currency amount per order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_quote_amount: Option<String>,

    /// Amount precision
    pub amount_precision: i32,

    /// Price precision
    pub precision: i32,

    /// Trading status (0: disabled, 1: enabled)
    pub trade_status: String,

    /// Sell start timestamp
    pub sell_start: i64,

    /// Buy start timestamp
    pub buy_start: i64,
}

impl RestClient {
    /// Get specific currency pair details
    ///
    /// This endpoint returns detailed information about a specific currency pair
    /// including trading fees, precision, and trading status.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-details-of-a-specifc-currency-pair>
    pub async fn get_currency_pair(
        &self,
        currency_pair: &str,
    ) -> crate::gateio::spot::Result<CurrencyPair> {
        let endpoint = format!("/spot/currency_pairs/{}", currency_pair);
        self.get(&endpoint).await
    }
}
