use crate::bitget::enums::ProductType;
use rest::Client;
use serde::{Deserialize, Serialize};

/// Get Current Funding Rate
///
/// Frequency limit: 20 times/1s (IP)
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentFundingRateRequest {
    /// Trading pair
    pub symbol: String,
    /// Product type
    pub product_type: ProductType,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentFundingRateData {
    /// Trading pair
    pub symbol: String,
    /// Current funding rate
    pub funding_rate: String,
    /// Next funding time
    pub next_funding_time: String,
}

pub async fn current_funding_rate(
    client: &Client,
    params: &CurrentFundingRateRequest,
) -> Result<CurrentFundingRateData, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/market/current-fund-rate";
    client.get(endpoint, Some(params)).await.map_err(Into::into)
}
