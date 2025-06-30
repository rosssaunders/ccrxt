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
    pub async fn get_server_time(&self) -> crate::gateio::Result<ServerTime> {
        self.get("/spot/time").await
    }
}