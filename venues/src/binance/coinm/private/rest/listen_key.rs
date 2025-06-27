use crate::binance::coinm::RestResult;
use crate::binance::coinm::private::rest::client::RestClient;
use crate::binance::shared;
use serde::{Deserialize, Serialize};

/// Request parameters for creating listen key.
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateListenKeyRequest {
    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Request parameters for extending listen key.
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExtendListenKeyRequest {
    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Request parameters for deleting listen key.
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteListenKeyRequest {
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

/// Response for extending or deleting listen key (empty response).
#[derive(Debug, Deserialize)]
pub struct ListenKeyResponse {}

impl RestClient {
    /// Create a listen key for user data stream on Binance Coin-M Futures.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
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
        shared::send_signed_request(
            self,
            "/dapi/v1/listenKey",
            reqwest::Method::POST,
            params,
            weight,
            false,
        )
        .await
    }

    /// Extend a listen key for user data stream on Binance Coin-M Futures.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
    /// PUT /dapi/v1/listenKey
    /// Weight: 1
    /// Requires API key.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`ExtendListenKeyRequest`])
    ///
    /// # Returns
    /// A [`ListenKeyResponse`] object (empty response indicates success).
    pub async fn extend_listen_key(
        &self,
        params: ExtendListenKeyRequest,
    ) -> RestResult<ListenKeyResponse> {
        let weight = 1;
        shared::send_signed_request(
            self,
            "/dapi/v1/listenKey",
            reqwest::Method::PUT,
            params,
            weight,
            false,
        )
        .await
    }

    /// Delete a listen key for user data stream on Binance Coin-M Futures.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
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
        shared::send_signed_request(
            self,
            "/dapi/v1/listenKey",
            reqwest::Method::DELETE,
            params,
            weight,
            false,
        )
        .await
    }
}
