use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::utils::send_request;

#[derive(Debug, Serialize, Deserialize)]
pub struct CountdownCancelAllRequest {
    pub symbol: String,
    pub countdown_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountdownCancelAllResponse {
    pub symbol: String,
    pub countdown_time: i64,
}

impl BinanceCoinMPrivateRest {
    /// Cancel all open orders after a countdown
    /// 
    /// # Arguments
    /// 
    /// * `request` - Request containing the symbol and countdown time in milliseconds
    /// 
    /// # Returns
    /// 
    /// Response confirming the countdown cancellation request
    pub async fn countdown_cancel_all(&self, request: CountdownCancelAllRequest) -> BinanceCoinMResult<CountdownCancelAllResponse> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let mut query_str = format!("timestamp={}", timestamp);
        let signature = self.sign_request(&query_str);
        query_str.push_str(&format!("&signature={}", signature));
        let endpoint = "/dapi/v1/countdownCancelAll";
        let response = send_request(
            &self.client,
            &self.base_url,
            endpoint,
            reqwest::Method::POST,
            Some(&query_str),
            Some(self.api_key.expose_secret()),
            || self.rate_limiter.check_weight_limit("countdownCancelAll", 1)
        ).await?;
        Ok(response.data)
    }
} 