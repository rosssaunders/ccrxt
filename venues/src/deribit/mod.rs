//! Deribit trading platform implementation
//!
//! This module provides rate limiting and private REST API endpoints for the Deribit API.
//! Deribit uses a credit-based rate limiting system with different tiers based
//! on trading volume.
//!
//! # Example Usage
//!
//! ```rust,no_run
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
pub mod rate_limit;

pub mod public {
    pub mod rest;
    pub mod websocket;

    pub use self::rest::RestClient;
    // Specific request/response structs from public::rest
    pub use self::rest::{GetComboIdsRequest, GetComboIdsResponse};
    // WebSocket exports
    pub use self::websocket::{DeribitMessage, DeribitWebSocketClient, HelloRequest, HelloResponse, HelloResult, JsonRpcRequest};
    pub use self::websocket::client::DeribitWebSocketError;
}

pub mod private {
    pub mod rest;

    pub use self::rest::RestClient;

    // Specific request/response structs from private::rest
    pub use self::rest::{
        DepositId, Originator, SendRfqRequest, SendRfqResponse, SetClearanceOriginatorRequest, SetClearanceOriginatorResponse, SetClearanceOriginatorResult,
        Side, SubmitTransferBetweenSubaccountsRequest, SubmitTransferBetweenSubaccountsResponse, SubmitTransferToUserRequest, SubmitTransferToUserResponse, TransferData, WithdrawRequest, WithdrawResponse, WithdrawalData,
    };
}

pub mod message;

pub use message::*;
pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use rate_limit::*;

pub use public::RestClient as PublicRestClient;
pub use public::{
    DeribitMessage, DeribitWebSocketClient, GetComboIdsRequest, GetComboIdsResponse, HelloRequest, HelloResponse, HelloResult,
    JsonRpcRequest
};
pub use public::websocket::client::DeribitWebSocketError;

pub use private::RestClient as PrivateRestClient;
pub use private::{
    DepositId, Originator, SendRfqRequest, SendRfqResponse, SetClearanceOriginatorRequest, SetClearanceOriginatorResponse, SetClearanceOriginatorResult,
    Side, SubmitTransferBetweenSubaccountsRequest, SubmitTransferBetweenSubaccountsResponse, SubmitTransferToUserRequest, SubmitTransferToUserResponse, TransferData, WithdrawRequest, WithdrawResponse, WithdrawalData,
};

/// Type alias for results returned by Deribit API operations
pub type RestResult<T> = Result<T, Errors>;
