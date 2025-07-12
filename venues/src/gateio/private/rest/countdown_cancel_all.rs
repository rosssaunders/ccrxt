use serde::{Deserialize, Serialize};

use super::RestClient;

/// Countdown cancel all request
#[derive(Debug, Clone, Serialize)]
pub struct CountdownCancelAllRequest {
    /// Countdown time in seconds (0 to disable)
    pub timeout: u32,

    /// Currency pair (optional, all pairs if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,
}

/// Countdown cancel all response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownCancelAllResponse {
    /// Trigger time (Unix timestamp)
    pub trigger_time: i64,
}

impl RestClient {
    /// Set up countdown cancel all
    ///
    /// This endpoint sets up an automatic order cancellation after a specified timeout.
    /// Setting timeout to 0 disables the countdown.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#countdown-cancel-orders>
    pub async fn countdown_cancel_all(
        &self,
        request: CountdownCancelAllRequest,
    ) -> crate::gateio::Result<CountdownCancelAllResponse> {
        self.post("/spot/countdown_cancel_all", &request).await
    }
}
