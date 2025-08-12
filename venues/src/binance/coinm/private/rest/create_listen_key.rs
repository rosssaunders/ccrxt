use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, private::rest::client::RestClient};

const LISTEN_KEY_ENDPOINT: &str = "/dapi/v1/listenKey";

/// Request parameters for creating listen key.
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateListenKeyRequest {
    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response for creating listen key.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateListenKeyResponse {
    /// Listen key
    pub listen_key: String,
}

impl RestClient {
    /// Create a listen key for user data stream on Binance Coin-M Futures.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/user-data-streams/Start-User-Data-Stream)
    ///
    /// POST /dapi/v1/listenKey
    /// Weight: 1
    /// Requires API key.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`CreateListenKeyRequest`])
    ///
    /// # Returns
    /// A [`CreateListenKeyResponse`] object with the listen key.
    pub async fn create_listen_key(
        &self,
        params: CreateListenKeyRequest,
    ) -> RestResult<CreateListenKeyResponse> {
        let weight = 1;
        self.send_post_signed_request(LISTEN_KEY_ENDPOINT, params, weight, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_listen_key_request_serialization() {
        let request = CreateListenKeyRequest { recv_window: None };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_create_listen_key_request_serialization_with_recv_window() {
        let request = CreateListenKeyRequest {
            recv_window: Some(5000),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "recvWindow=5000");
    }

    #[test]
    fn test_create_listen_key_response_deserialization() {
        let json = r#"{
            "listenKey": "pqia91ma19a5s61cv6a81va65sdf19v8a65a1a5s61cv6a81va65sdf19v8a65a1"
        }"#;
        let response: CreateListenKeyResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response.listen_key,
            "pqia91ma19a5s61cv6a81va65sdf19v8a65a1a5s61cv6a81va65sdf19v8a65a1"
        );
    }
}
