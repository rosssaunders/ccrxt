use super::client::RestClient;
use crate::cryptocom::{EndpointType, RestResult};
use serde_json::Value;

impl RestClient {
    /// Get the list of available trading instruments
    ///
    /// This method calls the public/get-instruments endpoint to retrieve
    /// information about all available trading pairs.
    pub async fn get_instruments(&self) -> RestResult<Value> {
        self.send_request(
            "public/get-instruments",
            reqwest::Method::GET,
            None,
            EndpointType::PublicGetInstruments,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruments_endpoint_type() {
        let instruments_endpoint = EndpointType::PublicGetInstruments;
        assert!(instruments_endpoint.rate_limit().max_requests > 0);
    }
}
