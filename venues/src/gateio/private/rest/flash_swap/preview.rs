use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const PREVIEW_ENDPOINT: &str = "/flash_swap/orders/preview";

/// Flash swap order preview request
#[derive(Debug, Clone, Serialize)]
pub struct PreviewRequest {
    /// Currency to sell
    pub sell_currency: String,

    /// Amount to sell (use either sell_amount or buy_amount, not both)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_amount: Option<String>,

    /// Currency to buy
    pub buy_currency: String,

    /// Amount to buy (use either sell_amount or buy_amount, not both)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_amount: Option<String>,
}

/// Flash swap order preview response
#[derive(Debug, Clone, Deserialize)]
pub struct PreviewResponse {
    /// Preview ID for order creation
    pub preview_id: String,

    /// Currency to sell
    pub sell_currency: String,

    /// Amount to sell
    pub sell_amount: String,

    /// Currency to buy
    pub buy_currency: String,

    /// Amount to buy
    pub buy_amount: String,

    /// Exchange rate
    pub price: String,

    /// Fee amount
    pub fee: String,

    /// Fee currency
    pub fee_currency: String,

    /// Preview expiration time
    pub expire_time: i64,
}

impl RestClient {
    /// Preview a flash swap order to get estimated amounts and price
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#preview-flash-swap-order)
    pub async fn preview_flash_swap_order(
        &self,
        req: PreviewRequest,
    ) -> RestResult<PreviewResponse> {
        self.send_post_request(PREVIEW_ENDPOINT, Some(&req)).await
    }
}
