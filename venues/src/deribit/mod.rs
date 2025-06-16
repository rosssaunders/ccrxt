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

mod enums;
mod errors;
pub mod rate_limit;

pub mod public {
    pub mod rest;
    pub mod websocket;

    pub use self::rest::GetComboIdsRequest;
    pub use self::rest::GetComboIdsResponse;
    pub use self::rest::RestClient;
    pub use self::websocket::DeribitMessage;
    pub use self::websocket::DeribitWebSocketClient;
    pub use self::websocket::HelloRequest;
    pub use self::websocket::HelloResponse;
    pub use self::websocket::HelloResult;
    pub use self::websocket::JsonRpcRequest;
    pub use self::websocket::client::DeribitWebSocketError;
}

pub mod private {
    pub mod rest;

    pub use self::rest::AddToAddressBookRequest;
    pub use self::rest::AddToAddressBookResponse;
    pub use self::rest::AddressBookEntry;
    pub use self::rest::CancelAllRequest;
    pub use self::rest::CancelAllResponse;
    pub use self::rest::CancelOrderRequest;
    pub use self::rest::CancelOrderResponse;
    pub use self::rest::CancelledOrder;
    pub use self::rest::DepositId;
    pub use self::rest::Originator;
    pub use self::rest::RestClient;
    pub use self::rest::SendRfqRequest;
    pub use self::rest::SendRfqResponse;
    pub use self::rest::SetClearanceOriginatorRequest;
    pub use self::rest::SetClearanceOriginatorResponse;
    pub use self::rest::SetClearanceOriginatorResult;
    pub use self::rest::Side;
    pub use self::rest::SubmitTransferBetweenSubaccountsRequest;
    pub use self::rest::SubmitTransferBetweenSubaccountsResponse;
    pub use self::rest::SubmitTransferToUserRequest;
    pub use self::rest::SubmitTransferToUserResponse;
    pub use self::rest::TransferData;
    pub use self::rest::WithdrawRequest;
    pub use self::rest::WithdrawResponse;
    pub use self::rest::WithdrawalData;
    pub use self::rest::{IndexName, MmpConfig, ResetMmpRequest, ResetMmpResponse, SetMmpConfigRequest, SetMmpConfigResponse};
}

pub mod message;

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use message::*;
pub use private::AddToAddressBookRequest;
pub use private::AddToAddressBookResponse;
pub use private::AddressBookEntry;
pub use private::CancelAllRequest;
pub use private::CancelAllResponse;
pub use private::CancelOrderRequest;
pub use private::CancelOrderResponse;
pub use private::CancelledOrder;
pub use private::DepositId;
pub use private::IndexName;
pub use private::Originator;
pub use private::ResetMmpRequest;
pub use private::ResetMmpResponse;
pub use private::RestClient as PrivateRestClient;
pub use private::SendRfqRequest;
pub use private::SendRfqResponse;
pub use private::SetClearanceOriginatorRequest;
pub use private::SetClearanceOriginatorResponse;
pub use private::SetClearanceOriginatorResult;
pub use private::Side;
pub use private::SubmitTransferBetweenSubaccountsRequest;
pub use private::SubmitTransferBetweenSubaccountsResponse;
pub use private::SubmitTransferToUserRequest;
pub use private::SubmitTransferToUserResponse;
pub use private::TransferData;
pub use private::WithdrawRequest;
pub use private::WithdrawResponse;
pub use private::WithdrawalData;
pub use public::DeribitMessage;
pub use public::DeribitWebSocketClient;
pub use public::GetComboIdsRequest;
pub use public::GetComboIdsResponse;
pub use public::HelloRequest;
pub use public::HelloResponse;
pub use public::HelloResult;
pub use public::JsonRpcRequest;
pub use public::RestClient as PublicRestClient;
pub use public::websocket::client::DeribitWebSocketError;
pub use rate_limit::*;

/// Type alias for results returned by Deribit API operations
pub type RestResult<T> = Result<T, Errors>;
