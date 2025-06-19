//! Private WebSocket endpoints for Deribit API
//!
//! This module provides WebSocket clients and message types for Deribit's private API endpoints.
//! Private endpoints require authentication and are used for trading operations.

pub mod client;
pub mod auth;

// Placeholder modules for future implementation
pub mod buy {
    use serde::{Deserialize, Serialize};
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BuyResponse {
        pub jsonrpc: String,
        pub id: u64,
        // Add actual buy response fields
    }
}

pub mod sell {
    use serde::{Deserialize, Serialize};
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SellResponse {
        pub jsonrpc: String,
        pub id: u64,
        // Add actual sell response fields
    }
}

pub mod cancel {
    use serde::{Deserialize, Serialize};
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CancelResponse {
        pub jsonrpc: String,
        pub id: u64,
        // Add actual cancel response fields
    }
}

// Re-export main types
pub use client::{DeribitPrivateMessage, PrivateWebSocketClient};
pub use auth::{AuthRequest, AuthResponse, AuthResult};