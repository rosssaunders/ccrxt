pub mod http_client;

#[cfg(feature = "native")]
pub mod native;

// Re-export the main types
pub use http_client::{HttpClient, HttpError, Method, Request, RequestBuilder, Response};
