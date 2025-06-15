//! Public endpoints for Deribit API

pub mod jsonrpc;

// Re-export commonly used types
pub use jsonrpc::{JsonRpcClient, StatusResult};