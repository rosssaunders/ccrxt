use crate::bitget::enums::ProductType;
use rest::Client;
use serde::{Deserialize, Serialize};

/// Get Open Interest
/// 
/// Frequency limit: 20 times/1s (IP)
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestRequest {
    /// Trading pair
    pub symbol: String,
    /// Product type
    pub product_type: ProductType,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestData {
    /// Trading pair
    pub symbol: String,
    /// Open interest in the base currency
    pub amount: String,
    /// Open interest in USDT
    pub size: String,
    /// Timestamp
    pub ts: String,
}

pub async fn open_interest(
    client: &Client,
    params: &OpenInterestRequest,
) -> Result<OpenInterestData, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/market/open-interest";
    client.get(endpoint, Some(params)).await.map_err(Into::into)
}
