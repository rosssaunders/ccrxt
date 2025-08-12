//! Nonce endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

/// Endpoint URL path for nonce
const NONCE_ENDPOINT: &str = "/trading-api/v1/nonce";

/// Nonce range information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nonce {
    /// Lower bound of nonce range
    pub lower_bound: u64,

    /// Upper bound of nonce range
    pub upper_bound: u64,
}

impl RestClient {
    /// Get the current nonce range
    ///
    /// The lower bound of nonce range is EPOCH start of day in microseconds,
    /// and upper bound of nonce range is EPOCH end of day in microseconds.
    ///
    /// [docs]: https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/nonce
    ///
    /// # Returns
    /// A `RestResult<Nonce>` containing the nonce range information
    ///
    /// # Errors
    /// Returns an error if the request fails or the response cannot be parsed
    ///
    /// # Example
    /// ```no_run
    /// # use venues::bullish::public::rest::RestClient;
    /// # async fn example(client: RestClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let nonce = client.get_nonce().await?;
    /// println!("Nonce range: {} - {}", nonce.lower_bound, nonce.upper_bound);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_nonce(&self) -> RestResult<Nonce> {
        self.send_get_request(NONCE_ENDPOINT, EndpointType::PublicOther)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nonce_deserialization() {
        let json = r#"
        {
            "lowerBound": 1639440000000000,
            "upperBound": 1639526399999999
        }
        "#;

        let nonce: Nonce = serde_json::from_str(json).unwrap();
        assert_eq!(nonce.lower_bound, 1639440000000000);
        assert_eq!(nonce.upper_bound, 1639526399999999);
    }
}
