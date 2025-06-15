//! Deribit trading platform implementation
//!
//! This module provides rate limiting and private REST API endpoints for the Deribit API.
//! Deribit uses a credit-based rate limiting system with different tiers based
//! on trading volume.
//!
//! # Example Usage
//!
//! ```rust
//! use venues::deribit::{RateLimiter, AccountTier, EndpointType, PublicRestClient, GetComboIdsRequest, Currency};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a rate limiter for a Tier 3 account (1-25M USD trading volume)
//!     let limiter = RateLimiter::new(AccountTier::Tier3);
//!    
//!     // Create a public REST client
//!     let client = reqwest::Client::new();
//!     let rest_client = PublicRestClient::new("https://www.deribit.com", client, limiter);
//!    
//!     // Get combo IDs for BTC
//!     let request = GetComboIdsRequest {
//!         currency: Currency::BTC,
//!         state: None,
//!     };
//!     let response = rest_client.get_combo_ids(request).await?;
//!     println!("Found {} combo IDs", response.result.len());
//!    
//!     Ok(())
//! }
//! ```

mod errors;
mod enums;
#[cfg(test)]
mod integration_tests;
pub mod rate_limit;
mod usage_example; // Assuming this is a valid module, if not, it might need removal or fixing.

pub mod public {
    pub mod rest;
    pub mod websocket; // Added websocket module here based on later use statements

    pub use self::rest::RestClient;
    // Specific request/response structs from public::rest
    pub use self::rest::{GetComboIdsRequest, GetComboIdsResponse};

    // Specific websocket structs
    pub use self::websocket::{DeribitMessage, DeribitWebSocketClient, HelloRequest, HelloResponse, HelloResult, JsonRpcRequest};
}

pub mod private {
    pub mod rest;
    pub use self::rest::RestClient;
    // Specific request/response structs from private::rest - including both user and subaccount transfers
    pub use self::rest::{
        SubmitTransferToUserRequest, SubmitTransferToUserResponse, TransferData,
        Currency, SubmitTransferToSubaccountRequest, SubmitTransferToSubaccountResponse,
        TransferResult, TransferDirection, TransferState, TransferType,
    };
}

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use rate_limit::*;

pub use public::RestClient as PublicRestClient;
pub use public::{
    DeribitMessage, DeribitWebSocketClient, GetComboIdsRequest, GetComboIdsResponse, HelloRequest, HelloResponse, HelloResult,
    JsonRpcRequest,
};

pub use private::RestClient as PrivateRestClient;
pub use private::{
    SubmitTransferToUserRequest, SubmitTransferToUserResponse, TransferData,
    Currency, SubmitTransferToSubaccountRequest, SubmitTransferToSubaccountResponse,
    TransferResult, TransferDirection, TransferState, TransferType,
};

/// Type alias for results returned by Deribit API operations
pub type RestResult<T> = Result<T, Errors>;
