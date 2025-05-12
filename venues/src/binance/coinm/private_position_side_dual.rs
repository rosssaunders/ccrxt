use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::api_errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::utils::send_request;

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionSideDual {
    pub dual_side_position: bool,
}

impl BinanceCoinMPrivateRest {
    /// Get current position mode (dual/single)
    /// 
    /// # Returns
    /// 
    /// Current position mode information
    pub async fn get_position_side_dual(&self) -> BinanceCoinMResult<PositionSideDual> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let query = format!("timestamp={}", timestamp);
        let signature = self.sign_request(&query);
        let mut query_with_sig = query.clone();
        query_with_sig.push_str(&format!("&signature={}", signature));
        let endpoint = "/dapi/v1/positionSide/dual";
        let response = send_request(
            &self.client,
            &self.base_url,
            endpoint,
            reqwest::Method::GET,
            Some(&query_with_sig),
            Some(self.api_key.expose_secret()),
            || self.rate_limiter.check_weight_limit("positionSideDual", 1)
        ).await?;
        Ok(response.data)
    }

    /// Change position mode (dual/single)
    /// 
    /// # Arguments
    /// 
    /// * `dual_side_position` - true for dual-side position mode, false for single-side position mode
    /// 
    /// # Returns
    /// 
    /// Success response
    pub async fn change_position_side_dual(&self, dual_side_position: bool) -> BinanceCoinMResult<()> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let query = format!("dualSidePosition={}&timestamp={}", dual_side_position, timestamp);
        let signature = self.sign_request(&query);
        let mut query_with_sig = query.clone();
        query_with_sig.push_str(&format!("&signature={}", signature));
        let endpoint = "/dapi/v1/positionSide/dual";
        let _response = send_request::<serde_json::Value, _, _>(
            &self.client,
            &self.base_url,
            endpoint,
            reqwest::Method::POST,
            Some(&query_with_sig),
            Some(self.api_key.expose_secret()),
            || self.rate_limiter.check_weight_limit("positionSideDual", 1)
        ).await?;
        Ok(())
    }
} 