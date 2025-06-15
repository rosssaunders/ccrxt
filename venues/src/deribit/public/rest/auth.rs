use serde::{Deserialize, Serialize};

/// Grant type for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    ClientCredentials,
    ClientSignature,
    RefreshToken,
}

/// Request for the /public/auth endpoint
#[derive(Debug, Clone, Serialize)]
pub struct AuthRequest {
    /// Method of authentication
    pub grant_type: GrantType,
    
    /// Required for grant type client_credentials and client_signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    
    /// Required for grant type client_credentials
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    
    /// Required for grant type refresh_token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    
    /// Required for grant type client_signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    
    /// Required for grant type client_signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    
    /// Optional for grant type client_signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    
    /// Optional for grant type client_signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    
    /// Will be passed back in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    
    /// Describes type of the access for assigned token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl AuthRequest {
    /// Create a new auth request with client credentials
    pub fn client_credentials(client_id: String, client_secret: String) -> Self {
        Self {
            grant_type: GrantType::ClientCredentials,
            client_id: Some(client_id),
            client_secret: Some(client_secret),
            refresh_token: None,
            timestamp: None,
            signature: None,
            nonce: None,
            data: None,
            state: None,
            scope: None,
        }
    }
    
    /// Create a new auth request with refresh token
    pub fn refresh_token(refresh_token: String) -> Self {
        Self {
            grant_type: GrantType::RefreshToken,
            client_id: None,
            client_secret: None,
            refresh_token: Some(refresh_token),
            timestamp: None,
            signature: None,
            nonce: None,
            data: None,
            state: None,
            scope: None,
        }
    }
    
    /// Create a new auth request with client signature
    pub fn client_signature(
        client_id: String,
        timestamp: i64,
        signature: String,
    ) -> Self {
        Self {
            grant_type: GrantType::ClientSignature,
            client_id: Some(client_id),
            client_secret: None,
            refresh_token: None,
            timestamp: Some(timestamp),
            signature: Some(signature),
            nonce: None,
            data: None,
            state: None,
            scope: None,
        }
    }
    
    /// Set optional nonce for client signature
    pub fn with_nonce(mut self, nonce: String) -> Self {
        self.nonce = Some(nonce);
        self
    }
    
    /// Set optional data for client signature
    pub fn with_data(mut self, data: String) -> Self {
        self.data = Some(data);
        self
    }
    
    /// Set optional state
    pub fn with_state(mut self, state: String) -> Self {
        self.state = Some(state);
        self
    }
    
    /// Set optional scope
    pub fn with_scope(mut self, scope: String) -> Self {
        self.scope = Some(scope);
        self
    }
}

/// Enabled advanced on-key features
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnabledFeature {
    RestrictedBlockTrades,
    BlockTradeApproval,
}

/// Response from the /public/auth endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct AuthResponse {
    /// The access token
    pub access_token: String,
    
    /// List of enabled advanced on-key features
    pub enabled_features: Vec<EnabledFeature>,
    
    /// Token lifetime in seconds
    pub expires_in: i32,
    
    /// The access token was acquired by logging in through Google
    pub google_login: bool,
    
    /// 2FA is required for privileged methods
    pub mandatory_tfa_status: String,
    
    /// Can be used to request a new token (with a new lifetime)
    pub refresh_token: String,
    
    /// Type of the access for assigned token
    pub scope: String,
    
    /// Optional Session id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    
    /// Copied from the input (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    
    /// Authorization type, allowed value - "bearer"
    pub token_type: String,
}

/// Standard Deribit JSON-RPC response wrapper for auth endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct AuthJsonRpcResponse {
    /// The id that was sent in the request
    pub id: Option<serde_json::Value>,
    
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    
    /// The result containing the auth response
    pub result: AuthResponse,
}