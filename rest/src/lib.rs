// Minimal library file to satisfy Cargo

pub mod client;
pub mod error;
pub mod rate_limiter;
pub mod request;
pub mod secrets;

// Re-export the main client
pub use client::Client;

/// Trait for Bitget API requests
pub trait BitgetRequest: Send + Sync + std::fmt::Debug {
    type Response: Send + Sync + std::fmt::Debug;
    
    /// Get the API endpoint path
    fn path(&self) -> String;
    
    /// Get the HTTP method (GET, POST, etc.)
    fn method(&self) -> String;
    
    /// Whether this request needs authentication/signature
    fn need_signature(&self) -> bool;
}
