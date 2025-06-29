use serde::{Deserialize, Serialize};

use crate::kucoin::{ResponseHeaders, RestResponse, Result};

use super::RestClient;

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
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::public::RestClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_default();
    ///     let request = GetServerTimeRequest::default();
    ///     let (response, _headers) = client.get_server_time(request).await?;
    ///     println!("Server time: {}", response.timestamp);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_server_time(
        &self,
        _request: GetServerTimeRequest,
    ) -> Result<(GetServerTimeResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<i64>, ResponseHeaders) =
            self.get("/api/v1/timestamp", None).await?;

        let server_time_response = GetServerTimeResponse {
            timestamp: response.data,
        };

        Ok((server_time_response, headers))
    }
}
