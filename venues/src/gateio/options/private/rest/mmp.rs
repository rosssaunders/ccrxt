use serde::{Deserialize, Serialize};

use super::RestClient;

/// Market Maker Protection (MMP) settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MMPSettings {
    /// User ID
    pub user: i64,

    /// Underlying asset
    pub underlying: String,

    /// Enable MMP
    pub enable: bool,

    /// Window size in seconds
    pub window: i32,

    /// Freeze time in seconds
    pub freeze_time: i32,

    /// Trade limit
    pub trade_limit: i32,

    /// Delta limit
    pub delta_limit: String,

    /// Vega limit
    pub vega_limit: String,
}

/// Request to update MMP settings
#[derive(Debug, Clone, Serialize)]
pub struct UpdateMMPRequest {
    /// Underlying asset
    pub underlying: String,

    /// Enable MMP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,

    /// Window size in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<i32>,

    /// Freeze time in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeze_time: Option<i32>,

    /// Trade limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_limit: Option<i32>,

    /// Delta limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_limit: Option<String>,

    /// Vega limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vega_limit: Option<String>,
}

impl RestClient {
    /// Get MMP settings
    ///
    /// This endpoint returns Market Maker Protection settings for options trading.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `underlying` - Underlying asset
    ///
    /// # Returns
    /// Current MMP settings
    pub async fn get_mmp_settings(
        &self,
        underlying: &str,
    ) -> crate::gateio::options::Result<MMPSettings> {
        let endpoint = format!("/options/mmp?underlying={}", underlying);
        self.get(&endpoint).await
    }

    /// Update MMP settings
    ///
    /// This endpoint updates Market Maker Protection settings.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The MMP settings update request parameters
    ///
    /// # Returns
    /// Updated MMP settings
    pub async fn update_mmp_settings(
        &self,
        request: UpdateMMPRequest,
    ) -> crate::gateio::options::Result<MMPSettings> {
        self.post("/options/mmp", &request).await
    }

    /// Reset MMP
    ///
    /// This endpoint resets the Market Maker Protection state.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `underlying` - Underlying asset to reset MMP for
    ///
    /// # Returns
    /// Empty response indicating success
    pub async fn reset_mmp(&self, underlying: &str) -> crate::gateio::options::Result<()> {
        let request = serde_json::json!({
            "underlying": underlying
        });
        self.post("/options/mmp/reset", &request).await
    }
}
