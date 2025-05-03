use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

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
        let url = format!("{}/dapi/v1/positionSide/dual?{}&signature={}", 
            self.base_url, query, signature);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let position_mode: PositionSideDual = response.json().await?;
        Ok(position_mode)
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
        let url = format!("{}/dapi/v1/positionSide/dual?{}&signature={}", 
            self.base_url, query, signature);

        let response = self.client
            .post(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        Ok(())
    }
} 