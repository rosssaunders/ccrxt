use serde::Deserialize;

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

// API endpoints
const SERVER_TIME_ENDPOINT: &str = "/api/v1/timestamp";
const SERVICE_STATUS_ENDPOINT: &str = "/api/v1/status";

/// Server time response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    /// Server time (milliseconds)
    pub current_time: i64,
}

/// Service status response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceStatus {
    /// Service status
    pub status: String,
    /// Status message
    pub msg: String,
}

impl super::RestClient {
    /// Get server time
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/market-data/get-server-time>
    pub async fn get_server_time(&self) -> Result<(RestResponse<ServerTime>, ResponseHeaders)> {
        self.send_request(SERVER_TIME_ENDPOINT, None::<&()>).await
    }

    /// Get service status
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/market-data/get-service-status>
    pub async fn get_service_status(
        &self,
    ) -> Result<(RestResponse<ServiceStatus>, ResponseHeaders)> {
        self.send_request(SERVICE_STATUS_ENDPOINT, None::<&()>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_time_deserialization() {
        let json = r#"{
            "currentTime": 1634567890123
        }"#;

        let time: ServerTime = serde_json::from_str(json).unwrap();
        assert_eq!(time.current_time, 1634567890123);
    }

    #[test]
    fn test_service_status_deserialization() {
        let json = r#"{
            "status": "open",
            "msg": "Service is operating normally"
        }"#;

        let status: ServiceStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.status, "open");
        assert_eq!(status.msg, "Service is operating normally");
    }
}
