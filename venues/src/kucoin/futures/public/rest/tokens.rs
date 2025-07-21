use serde::Deserialize;

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

// API endpoints
const PUBLIC_TOKEN_ENDPOINT: &str = "/api/v1/bullet-public";

/// Server information for WebSocket connection
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceServer {
    /// WebSocket endpoint URL
    pub endpoint: String,
    /// Encryption protocol (usually "wss")
    pub protocol: String,
    /// Whether encryption is enabled
    pub encrypt: bool,
    /// Ping interval (milliseconds)
    pub ping_interval: i64,
    /// Ping timeout (milliseconds)
    pub ping_timeout: i64,
}

/// WebSocket token response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketToken {
    /// WebSocket connection token
    pub token: String,
    /// List of available instance servers
    pub instance_servers: Vec<InstanceServer>,
}

impl super::RestClient {
    /// Get public WebSocket token for futures
    /// 
    /// <https://www.kucoin.com/docs-new/websocket-api/base-info/get-public-token-futures>
    pub async fn get_public_token(&self) -> Result<(RestResponse<WebSocketToken>, ResponseHeaders)> {
        // POST requests use empty params for public endpoints
        self.send_request(PUBLIC_TOKEN_ENDPOINT, None::<&()>).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_server_deserialization() {
        let json = r#"{
            "endpoint": "wss://push1.kucoin.com/endpoint",
            "protocol": "wss",
            "encrypt": true,
            "pingInterval": 18000,
            "pingTimeout": 10000
        }"#;

        let server: InstanceServer = serde_json::from_str(json).unwrap();
        assert_eq!(server.endpoint, "wss://push1.kucoin.com/endpoint");
        assert_eq!(server.protocol, "wss");
        assert_eq!(server.encrypt, true);
        assert_eq!(server.ping_interval, 18000);
        assert_eq!(server.ping_timeout, 10000);
    }

    #[test]
    fn test_websocket_token_deserialization() {
        let json = r#"{
            "token": "2neKiuYvAU61ZDXANAGAsiL4-iAExhsBXZxftpOeh_55i3Ysy2q2LEsEWU64mdzUOPusi34M_wGoSf7iNyEWJz8fXXX-0GUfLZ2Z2Z",
            "instanceServers": [
                {
                    "endpoint": "wss://push1.kucoin.com/endpoint",
                    "protocol": "wss",
                    "encrypt": true,
                    "pingInterval": 18000,
                    "pingTimeout": 10000
                }
            ]
        }"#;

        let token_response: WebSocketToken = serde_json::from_str(json).unwrap();
        assert_eq!(token_response.token, "2neKiuYvAU61ZDXANAGAsiL4-iAExhsBXZxftpOeh_55i3Ysy2q2LEsEWU64mdzUOPusi34M_wGoSf7iNyEWJz8fXXX-0GUfLZ2Z2Z");
        assert_eq!(token_response.instance_servers.len(), 1);
        assert_eq!(token_response.instance_servers[0].endpoint, "wss://push1.kucoin.com/endpoint");
    }
}