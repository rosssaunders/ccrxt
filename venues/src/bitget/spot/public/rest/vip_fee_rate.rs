use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::spot::{ApiError, RestResponse};

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
    /// Returns VIP fee rate information for all levels.
    ///
    /// [docs]: https://www.bitget.com/api-doc/spot/market/Get-VIP-Fee-Rate
    ///
    /// Rate limit: see official docs
    ///
    /// # Returns
    /// * `Result<RestResponse<Vec<VipFeeRate>>, ApiError>` - The VIP fee rate information
    pub async fn get_vip_fee_rate(&self) -> Result<RestResponse<Vec<VipFeeRate>>, ApiError> {
        self.get(VIP_FEE_RATE_ENDPOINT, None).await
    }
}
