use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::bitget::{ApiError, RestResponse};
use super::RestClient;

/// Endpoint for getting VIP fee rates
const VIP_FEE_RATE_ENDPOINT: &str = "/api/v2/spot/market/vip-fee-rate";

/// VIP fee rate information
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VipFeeRate {
    /// VIP level
    pub level: String,
    /// Total trading volume in last 30 days, USDT
    pub deal_amount: String,
    /// Total assets in USDT
    pub asset_amount: String,
    /// Taker fee rate
    pub taker_fee_rate: String,
    /// Maker fee rate
    pub maker_fee_rate: String,
    /// 24-hour withdrawal limit in BTC
    pub btc_withdraw_amount: String,
    /// 24-hour withdrawal limit in USDT
    pub usdt_withdraw_amount: String,
}

impl RestClient {
    /// Get VIP fee rates
    /// 
    /// # Returns
    /// * `Result<RestResponse<Vec<VipFeeRate>>, ApiError>` - The VIP fee rate information
    /// 
    /// # Example
    /// ```rust
    /// use venues::bitget::public::rest::RestClient;
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RestClient::new("https://api.bitget.com", Default::default(), reqwest::Client::new());
    /// 
    /// let response = client.get_vip_fee_rate().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_vip_fee_rate(&self) -> Result<RestResponse<Vec<VipFeeRate>>, ApiError> {
        let endpoint = VIP_FEE_RATE_ENDPOINT;
        self.get(endpoint, None).await
    }
}
