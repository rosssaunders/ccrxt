//! JSON-RPC client and endpoints for Deribit public API

pub mod client;
pub mod status;

// Re-export the main client and status types
pub use client::JsonRpcClient;
pub use status::StatusResult;