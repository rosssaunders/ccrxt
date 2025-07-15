use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult};

const INS_MARGIN_COIN_INFO_ENDPOINT: &str = "/v5/ins-loan/ensure-tokens-convert";

/// Request parameters for getting institutional loan margin coin info
#[derive(Debug, Clone, Serialize)]
pub struct GetInsMarginCoinInfoRequest {
    /// Product ID
    #[serde(rename = "productId")]
    pub product_id: String,
}

/// Margin coin information
#[derive(Debug, Clone, Deserialize)]
pub struct InsMarginCoinInfo {
    /// Margin coin
    #[serde(rename = "marginCoin")]
    pub margin_coin: String,
    
    /// Conversion rate
    #[serde(rename = "conversionRate")]
    pub conversion_rate: String,
    
    /// Liquidation order
    #[serde(rename = "liquidationOrder")]
    pub liquidation_order: String,
}

/// Institutional loan margin coin info data
#[derive(Debug, Clone, Deserialize)]
pub struct GetInsMarginCoinInfoData {
    /// Product ID
    #[serde(rename = "productId")]
    pub product_id: String,
    
    /// Parent UID
    #[serde(rename = "parentUid")]
    pub parent_uid: String,
    
    /// Loan coin
    #[serde(rename = "loanCoin")]
    pub loan_coin: String,
    
    /// List of margin coin information
    #[serde(rename = "marginCoinInfoList")]
    pub margin_coin_info_list: Vec<InsMarginCoinInfo>,
}

/// Response from the institutional loan margin coin info endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetInsMarginCoinInfoResponse {
    /// Success/Error code (0: success, 1: error)
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    
    /// Success/Error message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    
    /// Business data result
    pub result: GetInsMarginCoinInfoData,
    
    /// Extended information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    
    /// Current timestamp in milliseconds
    pub time: u64,
}

impl RestClient {
    /// Get institutional loan margin coin info
    ///
    /// Query for institutional loan product margin coin information, including margin coin list,
    /// conversion rate, and liquidation order.
    ///
    /// [API Documentation](https://bybit-exchange.github.io/docs/v5/otc/margin-coin-convert-info)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The margin coin info request parameters including:
    ///   - `product_id`: Product ID for the institutional loan
    ///
    /// # Returns
    /// A result containing the margin coin info response with conversion rates and liquidation order or an error
    pub async fn get_ins_margin_coin_info(
        &self,
        request: GetInsMarginCoinInfoRequest,
    ) -> RestResult<GetInsMarginCoinInfoResponse> {
        self.send_public_request(
            INS_MARGIN_COIN_INFO_ENDPOINT,
            Some(&request),
            EndpointType::Market,
        )
        .await
    }
}
