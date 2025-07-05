// Minimal library file to satisfy Cargo

pub mod client;
pub mod error;
pub mod rate_limiter;
pub mod request;
pub mod secrets;

// Re-export the main client
pub use client::Client;
