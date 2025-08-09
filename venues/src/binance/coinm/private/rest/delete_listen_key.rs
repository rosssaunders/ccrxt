use serde::Serialize;

use crate::binance::coinm::{RestResult, private::rest::client::RestClient};
use crate::binance::coinm::private::rest::extend_listen_key::ListenKeyResponse;

const LISTEN_KEY_ENDPOINT: &str = "/dapi/v1/listenKey";

/// Request parameters for deleting listen key.
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteListenKeyRequest {
    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

impl RestClient {
    /// Delete a listen key for user data stream on Binance Coin-M Futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/user-data-streams/Close-User-Data-Stream
    ///
    /// DELETE /dapi/v1/listenKey
    /// Weight: 1
    /// Requires API key.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`DeleteListenKeyRequest`])
    ///
    /// # Returns
    /// A [`ListenKeyResponse`] object (empty response indicates success).
    pub async fn delete_listen_key(
        &self,
        params: DeleteListenKeyRequest,
    ) -> RestResult<ListenKeyResponse> {
        let weight = 1;
        self.send_delete_signed_request(LISTEN_KEY_ENDPOINT, params, weight, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_listen_key_request_serialization() {
        let request = DeleteListenKeyRequest { recv_window: None };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_delete_listen_key_request_serialization_with_recv_window() {
        let request = DeleteListenKeyRequest {
            recv_window: Some(5000),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "recvWindow=5000");
    }

    #[test]
    fn test_listen_key_response_deserialization() {
        let json = r#"{}"#;
        let response: ListenKeyResponse = serde_json::from_str(json).unwrap();
        let _ = response;
    }
}
