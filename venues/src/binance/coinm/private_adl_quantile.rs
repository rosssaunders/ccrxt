use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::common::request::send_request;

#[derive(Debug, Serialize, Deserialize)]
pub struct ADLQuantile {
    pub symbol: String,
    pub adl_quantile: ADLQuantileValues,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ADLQuantileValues {
    pub long: i32,
    pub short: i32,
    pub hedge: i32,
}

impl BinanceCoinMPrivateRest {
    /// Get ADL quantile information
    /// 
    /// # Arguments
    /// 
    /// * `symbol` - Optional symbol to filter ADL quantile information
    /// 
    /// # Returns
    /// 
    /// Vector of ADL quantile information
    pub async fn get_adl_quantile(&self, symbol: Option<&str>) -> BinanceCoinMResult<Vec<ADLQuantile>> {
        let mut params = Vec::with_capacity(2);
        if let Some(s) = symbol {
            params.push(format!("symbol={}", s));
        }
        let timestamp = chrono::Utc::now().timestamp_millis();
        params.push(format!("timestamp={}", timestamp));
        let mut query = params.join("&");
        let signature = self.sign_request(&query);
        query.push_str(&format!("&signature={}", signature));
        let response = send_request::<Vec<ADLQuantile>, _, _>(
            &self.client,
            &self.base_url,
            "/dapi/v1/adlQuantile",
            reqwest::Method::GET,
            Some(&query),
            Some(self.api_key.expose_secret()),
            || async { Ok(()) }, // TODO: Replace with actual rate limit check
        ).await?;
        Ok(response.data)
    }
} 