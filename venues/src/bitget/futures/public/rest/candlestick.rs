use crate::bitget::enums::ProductType;
use rest::Client;
use serde::{Deserialize, Serialize};

/// Get Candlestick Data
///
/// Frequency limit: 20 times/1s (IP)
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CandlestickRequest {
    /// Trading pair
    pub symbol: String,
    /// Product type
    pub product_type: ProductType,
    /// Candlestick type (1min, 5min, 15min, 30min, 1h, 4h, 6h, 12h, 1day, 3day, 1week, 1M)
    pub granularity: String,
    /// Start time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// End time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Number of returned results, maximum 1000, default 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Candlestick data: [timestamp, open, high, low, close, volume, usdtVolume]
pub type CandlestickData = Vec<Vec<String>>;

pub async fn candlestick(
    client: &Client,
    params: &CandlestickRequest,
) -> Result<CandlestickData, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/market/candles";
    client.get(endpoint, Some(params)).await.map_err(Into::into)
}
