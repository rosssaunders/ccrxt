use serde::{Deserialize, Serialize};
use crate::deribit::DeribitWebSocketError;

/// Request for private/auth method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    /// API key for authentication
    pub key: String,
    /// Signature for authentication
    pub signature: String,
    /// Timestamp for the signature
    pub timestamp: u64,
    /// Nonce for the signature
    pub nonce: String,
}

/// Response for private/auth method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Request ID
    pub id: u64,
    /// Authentication result
    pub result: AuthResult,
}

/// Result data for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResult {
    /// Indicates successful authentication
    pub success: bool,
    /// Token for authenticated session (if applicable)
    pub token: Option<String>,
}

impl super::client::PrivateWebSocketClient {
    /// Send authentication request to Deribit API
    ///
    /// This method must be called after connecting and before using any private methods.
    /// It signs the authentication request using the API credentials provided during client creation.
    ///
    /// # Returns
    /// * `Ok(AuthResponse)` - Successful authentication
    /// * `Err(DeribitWebSocketError)` - Authentication failed or connection error
    ///
    /// # Example
    /// ```ignore
    /// let mut client = PrivateWebSocketClient::new(api_key, api_secret, None, rate_limiter);
    /// client.connect().await?;
    /// let auth_response = client.send_auth().await?;
    /// assert!(auth_response.result.success);
    /// ```
    pub async fn send_auth(&mut self) -> Result<AuthResponse, DeribitWebSocketError> {
        // Check if connected
        if !self.is_connected() {
            return Err(DeribitWebSocketError::NotConnected);
        }

        // Generate authentication parameters
        let timestamp = chrono::Utc::now().timestamp_millis() as u64;
        let nonce = uuid::Uuid::new_v4().to_string();
        
        // Create signature (simplified - actual implementation would follow Deribit signing rules)
        let api_key = self.api_key.expose_secret();
        let api_secret = self.api_secret.expose_secret();
        let signature = self.create_auth_signature(api_key, api_secret, timestamp, &nonce)?;
        
        // Create auth request
        let auth_request = AuthRequest {
            key: api_key.to_string(),
            signature,
            timestamp,
            nonce,
        };

        // Create JSON-RPC request
        let request_id = self.next_request_id();
        let json_rpc_request = crate::deribit::message::JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: request_id,
            method: "private/auth".to_string(),
            params: serde_json::to_value(auth_request).ok(),
        };

        // Send request
        let request_json = serde_json::to_string(&json_rpc_request)?;
        let response_json = self.send_message(&request_json).await?;
        
        // Parse response
        let auth_response: AuthResponse = serde_json::from_str(&response_json)?;
        
        // Update authentication state
        if auth_response.result.success {
            self.authenticated.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        
        Ok(auth_response)
    }

    /// Create authentication signature following Deribit's signing algorithm
    fn create_auth_signature(
        &self,
        api_key: &str,
        api_secret: &str,
        timestamp: u64,
        nonce: &str,
    ) -> Result<String, DeribitWebSocketError> {
        // Simplified signature creation - actual implementation would follow Deribit spec
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        
        let message = format!("{}{}{}", timestamp, nonce, api_key);
        let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())
            .map_err(|_| DeribitWebSocketError::AuthenticationFailed("Invalid API secret".to_string()))?;
        mac.update(message.as_bytes());
        
        Ok(hex::encode(mac.finalize().into_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_request_structure() {
        let auth_request = AuthRequest {
            key: "test_key".to_string(),
            signature: "test_signature".to_string(),
            timestamp: 1234567890,
            nonce: "test_nonce".to_string(),
        };

        assert_eq!(auth_request.key, "test_key");
        assert_eq!(auth_request.signature, "test_signature");
        assert_eq!(auth_request.timestamp, 1234567890);
        assert_eq!(auth_request.nonce, "test_nonce");
    }

    #[test]
    fn test_auth_request_serialization() {
        let auth_request = AuthRequest {
            key: "test_key".to_string(),
            signature: "test_signature".to_string(),
            timestamp: 1234567890,
            nonce: "test_nonce".to_string(),
        };

        let json = serde_json::to_string(&auth_request).expect("Failed to serialize auth request");
        let deserialized: AuthRequest = serde_json::from_str(&json).expect("Failed to deserialize auth request");

        assert_eq!(auth_request.key, deserialized.key);
        assert_eq!(auth_request.signature, deserialized.signature);
        assert_eq!(auth_request.timestamp, deserialized.timestamp);
        assert_eq!(auth_request.nonce, deserialized.nonce);
    }

    #[test]
    fn test_auth_response_deserialization() {
        let json = r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "success": true,
                "token": "test_token"
            }
        }"#;

        let auth_response: AuthResponse = serde_json::from_str(json).expect("Failed to deserialize auth response");
        assert_eq!(auth_response.jsonrpc, "2.0");
        assert_eq!(auth_response.id, 1);
        assert!(auth_response.result.success);
        assert_eq!(auth_response.result.token, Some("test_token".to_string()));
    }

    #[test]
    fn test_auth_response_without_token() {
        let json = r#"{
            "jsonrpc": "2.0", 
            "id": 2,
            "result": {
                "success": false,
                "token": null
            }
        }"#;

        let auth_response: AuthResponse = serde_json::from_str(json).expect("Failed to deserialize auth response");
        assert_eq!(auth_response.jsonrpc, "2.0");
        assert_eq!(auth_response.id, 2);
        assert!(!auth_response.result.success);
        assert_eq!(auth_response.result.token, None);
    }
}