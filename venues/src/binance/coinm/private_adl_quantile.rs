use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

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
        let mut query = String::new();
        if let Some(s) = symbol {
            query.push_str(&format!("symbol={}", s));
        }

        let timestamp = chrono::Utc::now().timestamp_millis();
        if !query.is_empty() {
            query.push('&');
        }
        query.push_str(&format!("timestamp={}", timestamp));

        let signature = self.sign_request(&query);
        query.push_str(&format!("&signature={}", signature));

        let url = format!("{}/dapi/v1/adlQuantile?{}", self.base_url, query);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let adl_quantiles: Vec<ADLQuantile> = response.json().await?;
        Ok(adl_quantiles)
    }
} 