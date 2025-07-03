use crate::bitget::enums::ProductType;
use rest::Client;
use serde::{Deserialize, Serialize};

/// Get Interest rate history
/// 
/// Frequency limit: 5 times/1s (IP)
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestRateHistoryRequest {
    /// Coin asset
    pub coin: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestRateData {
    /// Assets
    pub coin: String,
    /// History Interest list
    pub history_interest_rate_list: Vec<InterestRateItem>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestRateItem {
    /// Annual interest rate
    pub annual_interest_rate: String,
    /// Daily interest rate
    pub daily_interest_rate: String,
    /// Interest time
    pub ts: String,
}

pub async fn interest_rate_history(
    client: &Client,
    params: &InterestRateHistoryRequest,
) -> Result<InterestRateData, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/market/union-interest-rate-history";
    client.get(endpoint, Some(params)).await.map_err(Into::into)
}
