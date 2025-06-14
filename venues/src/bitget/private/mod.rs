pub mod rest;

// Re-export the main client
pub use rest::RestClient as PrivateRestClient;