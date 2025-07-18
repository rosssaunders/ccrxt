use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Get private WebSocket token request (no parameters needed)
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetPrivateTokenRequest;

/// Instance server information for WebSocket connection
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceServer {
    /// WebSocket endpoint URL
    pub endpoint: String,
    /// Encryption flag
    pub encrypt: bool,
    /// Protocol type
    pub protocol: String,
    /// Ping interval in milliseconds
    pub ping_interval: i64,
    /// Ping timeout in milliseconds
    pub ping_timeout: i64,
}

/// WebSocket token response for private connections
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketPrivateToken {
    /// Token for WebSocket authentication
    pub token: String,
    /// List of available instance servers
    pub instance_servers: Vec<InstanceServer>,
}

impl super::RestClient {
    /// Get private WebSocket token for futures
    ///
    /// <https://www.kucoin.com/docs-new/websocket-api/base-info/get-private-token-futures>
    pub async fn get_private_token(
        &self,
    ) -> Result<(RestResponse<WebSocketPrivateToken>, ResponseHeaders)> {
        const PRIVATE_TOKEN_ENDPOINT: &str = "/api/v1/bullet-private";
        // POST requests use empty params for private endpoints
        self.post(PRIVATE_TOKEN_ENDPOINT, &GetPrivateTokenRequest::default())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_server_deserialization() {
        let json = r#"{
            "endpoint": "wss://ws-api-futures.kucoin.com/",
            "encrypt": true,
            "protocol": "websocket",
            "pingInterval": 18000,
            "pingTimeout": 10000
        }"#;

        let server: InstanceServer = serde_json::from_str(json).unwrap();
        assert_eq!(server.endpoint, "wss://ws-api-futures.kucoin.com/");
        assert_eq!(server.encrypt, true);
        assert_eq!(server.protocol, "websocket");
        assert_eq!(server.ping_interval, 18000);
        assert_eq!(server.ping_timeout, 10000);
    }

    #[test]
    fn test_websocket_private_token_deserialization() {
        let json = r#"{
            "token": "2neAiuYvAU61ZD...",
            "instanceServers": [
                {
                    "endpoint": "wss://ws-api-futures.kucoin.com/",
                    "encrypt": true,
                    "protocol": "websocket",
                    "pingInterval": 18000,
                    "pingTimeout": 10000
                }
            ]
        }"#;

        let token: WebSocketPrivateToken = serde_json::from_str(json).unwrap();
        assert!(token.token.starts_with("2neAiuYvAU61ZD"));
        assert_eq!(token.instance_servers.len(), 1);
        assert_eq!(
            token.instance_servers[0].endpoint,
            "wss://ws-api-futures.kucoin.com/"
        );
    }

    #[test]
    fn test_get_private_token_request_default() {
        let request = GetPrivateTokenRequest::default();
        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
    }
}
