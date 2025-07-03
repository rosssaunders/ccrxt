use crate::bitget::enums::ProductType;
use rest::Client;
use serde::{Deserialize, Serialize};

/// Get Interest exchange rate
///
/// Rate limit: 5 requests/sec/IP
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRateRequest {}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRateData {
    /// Assets
    pub coin: String,
    /// Tier exchange rate
    pub exchange_rate_list: Vec<ExchangeRateItem>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRateItem {
    /// Tier
    pub tier: String,
    /// Min
    pub min_amount: String,
    /// Max (-1 means there is no limitation)
    pub max_amount: String,
    /// Exchange rate
    pub exchange_rate: String,
}

pub async fn exchange_rate(
    client: &Client,
    _params: Option<&ExchangeRateRequest>,
) -> Result<Vec<ExchangeRateData>, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/market/exchange-rate";
    client.get(endpoint, None::<&()>).await.map_err(Into::into)
}
