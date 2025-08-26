use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const SWAP_ENDPOINT: &str = "/flash_swap/swap";

/// Perform a flash swap
#[derive(Debug, Clone, Serialize)]
pub struct SwapRequest {
    pub from_currency: String,

    pub to_currency: String,

    pub amount: String,

    /// Quote id or price limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
}

/// Swap result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapResponse {
    pub order_id: String,

    pub filled: String,
}

impl RestClient {
    /// Execute a flash swap
    pub async fn flash_swap(&self, req: SwapRequest) -> RestResult<SwapResponse> {
        self.send_post_request(SWAP_ENDPOINT, Some(&req)).await
    }
}
