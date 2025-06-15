//! Public module for Deribit. 
//! 
//! Contains public endpoints that don't require authentication.

pub mod rest;

pub use rest::RestClient;