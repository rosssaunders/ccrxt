use serde::{Deserialize, Serialize};

use super::RestClient;

/// Server time response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTime {
    /// Current server timestamp in seconds
    pub server_time: i64,
}

impl RestClient {
    /// Get current server time
    ///
    /// This endpoint returns the current server time as a Unix timestamp.
    /// Useful for synchronizing client time with the server.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-server-current-time>
    pub async fn get_server_time(&self) -> crate::gateio::spotandmargin::Result<ServerTime> {
        self.get("/spot/time").await
    }
}
