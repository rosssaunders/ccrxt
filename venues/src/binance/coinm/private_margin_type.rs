use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

#[derive(Debug, Serialize, Deserialize)]
pub enum MarginType {
    #[serde(rename = "ISOLATED")]
    Isolated,
    #[serde(rename = "CROSSED")]
    Crossed,
}

impl BinanceCoinMPrivateRest {
    /// Change margin type
    /// 
    /// # Arguments
    /// 
    /// * `symbol` - Symbol to change margin type for
    /// * `margin_type` - New margin type (ISOLATED or CROSSED)
    /// 
    /// # Returns
    /// 
    /// Success response
    pub async fn change_margin_type(&self, symbol: &str, margin_type: MarginType) -> BinanceCoinMResult<()> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let query = format!(
            "symbol={}&marginType={}&timestamp={}",
            symbol,
            serde_json::to_string(&margin_type)?,
            timestamp
        );
        let signature = self.sign_request(&query);
        let url = format!("{}/dapi/v1/marginType?{}&signature={}", 
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