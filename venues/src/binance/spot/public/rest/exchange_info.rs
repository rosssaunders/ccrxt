use crate::binance::spot::public::rest::RestClient;
use crate::binance::spot::RestResult;
use serde::Deserialize;
use serde_json::Value;

/// Simple exchange info response - using Value for now to avoid defining all structs
#[derive(Debug, Deserialize)]
pub struct ExchangeInfo {
    pub timezone: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "rateLimits")]
    pub rate_limits: Vec<Value>,
    #[serde(rename = "exchangeFilters")]
    pub exchange_filters: Vec<Value>,
    pub symbols: Vec<Value>,
}

impl RestClient {
    /// Get exchange information
    /// 
    /// Current exchange trading rules and symbol information
    /// 
    /// Weight: 20
    pub async fn exchange_info(&self) -> RestResult<ExchangeInfo> {
        self.send_request("/api/v3/exchangeInfo", reqwest::Method::GET, None, None, 20)
            .await
    }
}