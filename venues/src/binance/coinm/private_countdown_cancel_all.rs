use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

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

        let url = format!("{}/dapi/v1/countdownCancelAll?{}", self.base_url, query_str);

        let response = self.client
            .post(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let result: CountdownCancelAllResponse = response.json().await?;
        Ok(result)
    }
} 