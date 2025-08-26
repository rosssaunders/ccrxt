use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const QUOTE_ENDPOINT: &str = "/flash_swap/quote";

/// Request a quote for a flash swap
#[derive(Debug, Clone, Serialize)]
pub struct QuoteRequest {
    pub from_currency: String,

    pub to_currency: String,

    pub amount: String,
}

/// Quote response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteResponse {
    pub price: String,

    pub fee: String,
}

impl RestClient {
    /// Get a flash swap quote
    pub async fn get_flash_swap_quote(&self, req: QuoteRequest) -> RestResult<QuoteResponse> {
        self.send_post_request(QUOTE_ENDPOINT, Some(&req)).await
    }
}
