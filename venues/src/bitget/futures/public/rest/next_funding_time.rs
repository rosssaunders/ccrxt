use crate::bitget::enums::ProductType;
use rest::Client;
use serde::{Deserialize, Serialize};

/// Get Next Funding Time
/// 
/// Frequency limit: 20 times/1s (IP)
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NextFundingTimeRequest {
    /// Trading pair
    pub symbol: String,
    /// Product type
    pub product_type: ProductType,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextFundingTimeData {
    /// Trading pair
    pub symbol: String,
    /// Next funding time
    pub funding_time: String,
}

pub async fn next_funding_time(
    client: &Client,
    params: &NextFundingTimeRequest,
) -> Result<NextFundingTimeData, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/market/funding-time";
    client.get(endpoint, Some(params)).await.map_err(Into::into)
}
