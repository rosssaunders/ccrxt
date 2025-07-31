// Minimal library file to satisfy Cargo

pub mod client;
pub mod error;
pub mod http_client;
pub mod rate_limiter;
pub mod request;
pub mod secrets;

#[cfg(feature = "native")]
pub mod native;

// Re-export the main types
pub use client::Client;
pub use http_client::{HttpClient, HttpError, Method, Request, RequestBuilder, Response};

#[cfg(feature = "native")]
pub use native::NativeHttpClient;
