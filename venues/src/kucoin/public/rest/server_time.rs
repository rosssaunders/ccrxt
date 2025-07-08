use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::{ResponseHeaders, RestResponse, Result};

const SERVER_TIME_ENDPOINT: &str = "/api/v1/timestamp";

/// Request for getting server time
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetServerTimeRequest {}

/// Response for server time
#[derive(Debug, Clone, Deserialize)]
pub struct GetServerTimeResponse {
    /// Server timestamp in milliseconds
    pub timestamp: i64,
}

impl RestClient {
    /// Get the current server time
    pub async fn get_server_time(
        &self,
        _request: GetServerTimeRequest,
    ) -> Result<(GetServerTimeResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<i64>, ResponseHeaders) =
            self.get(SERVER_TIME_ENDPOINT, None).await?;

        let server_time_response = GetServerTimeResponse {
            timestamp: response.data,
        };

        Ok((server_time_response, headers))
    }
}
