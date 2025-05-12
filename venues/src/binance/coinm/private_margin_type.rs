use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::api_errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::send_request;

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
        let mut query_with_sig = query.clone();
        query_with_sig.push_str(&format!("&signature={}", signature));
        let endpoint = "/dapi/v1/marginType";
        let _response = send_request::<serde_json::Value, _, _>(
            &self.client,
            &self.base_url,
            endpoint,
            reqwest::Method::POST,
            Some(&query_with_sig),
            Some(self.api_key.expose_secret()),
            || self.rate_limiter.check_weight_limit("marginType", 1)
        ).await?;
        Ok(())
    }
} 