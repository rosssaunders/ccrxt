//! Deribit trading platform implementation
//!
//! This module provides rate limiting and private REST API endpoints for the Deribit API.
//! Deribit uses a credit-based rate limiting system with different tiers based
//! on trading volume.
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use venues::deribit::{RateLimiter, AccountTier, EndpointType, PublicRestClient, GetComboIdsRequest, GetStatusRequest, Currency};
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
//!     // Get platform status
//!     let status_request = GetStatusRequest {};
//!     let status_response = rest_client.get_status(status_request).await?;
//!     println!("Platform locked status: {}", status_response.result.locked);
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
    pub use self::rest::GetStatusRequest;
    pub use self::rest::GetStatusResponse;
    pub use self::rest::GetStatusResult;
    pub use self::rest::GetTimeRequest;
    pub use self::rest::GetTimeResponse;
    pub use self::rest::RestClient;
    pub use self::websocket::HelloRequest;
    pub use self::websocket::HelloResponse;
    pub use self::websocket::HelloResult;
    pub use self::websocket::PrivateWebSocketClient;
    pub use self::websocket::SubscribeRequest;
    pub use self::websocket::SubscribeResponse;
    pub use self::websocket::client::DeribitWebSocketError;
}

pub mod private {
    pub mod rest;

    pub use self::rest::AddToAddressBookRequest;
    pub use self::rest::AddToAddressBookResponse;
    pub use self::rest::AddressBookEntry;
    pub use self::rest::CancelAllByCurrencyPairRequest;
    pub use self::rest::CancelAllByCurrencyPairResponse;
    pub use self::rest::CancelAllByCurrencyRequest;
    pub use self::rest::CancelAllByCurrencyResponse;
    pub use self::rest::CancelAllRequest;
    pub use self::rest::CancelAllResponse;
    pub use self::rest::CancelBlockRfqRequest;
    pub use self::rest::CancelBlockRfqResponse;
    pub use self::rest::CancelOnDisconnectResult;
    pub use self::rest::CancelOnDisconnectScope;
    pub use self::rest::CancelOrderRequest;
    pub use self::rest::CancelOrderResponse;
    pub use self::rest::CancelQuotesRequest;
    pub use self::rest::CancelQuotesResponse;
    pub use self::rest::CancelType;
    pub use self::rest::CancelWithdrawalRequest;
    pub use self::rest::CancelWithdrawalResponse;
    pub use self::rest::CancelledOrder;
    pub use self::rest::CreateComboLeg;
    pub use self::rest::CreateComboRequest;
    pub use self::rest::CreateComboResponse;
    pub use self::rest::CreateComboResult;
    pub use self::rest::CreateComboTrade;
    pub use self::rest::CreateDepositAddressRequest;
    pub use self::rest::CreateDepositAddressResponse;
    pub use self::rest::DepositAddress;
    pub use self::rest::DepositData;
    pub use self::rest::DepositId;
    pub use self::rest::DisableCancelOnDisconnectRequest;
    pub use self::rest::DisableCancelOnDisconnectResponse;
    pub use self::rest::EnableCancelOnDisconnectRequest;
    pub use self::rest::EnableCancelOnDisconnectResponse;
    pub use self::rest::GetAddressBookRequest;
    pub use self::rest::GetAddressBookResponse;
    pub use self::rest::GetCancelOnDisconnectRequest;
    pub use self::rest::GetCancelOnDisconnectResponse;
    pub use self::rest::GetCurrentDepositAddressRequest;
    pub use self::rest::GetCurrentDepositAddressResponse;
    pub use self::rest::GetDepositsRequest;
    pub use self::rest::GetDepositsResponse;
    pub use self::rest::GetDepositsResult;
    pub use self::rest::GetOpenOrdersByCurrencyRequest;
    pub use self::rest::GetOpenOrdersByCurrencyResponse;
    pub use self::rest::OpenOrder;
    pub use self::rest::OpenOrderType;
    pub use self::rest::GetUserTradesByCurrencyAndTimeRequest;
    pub use self::rest::GetUserTradesByCurrencyAndTimeResponse;
    pub use self::rest::GetUserTradesByCurrencyAndTimeResult;
    pub use self::rest::GetUserTradesByCurrencyRequest;
    pub use self::rest::GetUserTradesByCurrencyResponse;
    pub use self::rest::GetUserTradesByCurrencyResult;
    pub use self::rest::InvalidateBlockTradeSignatureRequest;
    pub use self::rest::InvalidateBlockTradeSignatureResponse;
    pub use self::rest::MovePositionTrade;
    pub use self::rest::MovePositionTradeResult;
    pub use self::rest::MovePositionsRequest;
    pub use self::rest::MovePositionsResponse;
    pub use self::rest::MovePositionsResult;
    pub use self::rest::Originator;
    pub use self::rest::RemoveFromAddressBookRequest;
    pub use self::rest::RemoveFromAddressBookResponse;
    pub use self::rest::RestClient;
    pub use self::rest::SendRfqRequest;
    pub use self::rest::SendRfqResponse;
    pub use self::rest::SetClearanceOriginatorRequest;
    pub use self::rest::SetClearanceOriginatorResponse;
    pub use self::rest::SetClearanceOriginatorResult;
    pub use self::rest::Side;
    pub use self::rest::SubaccountTransferData;
    pub use self::rest::SubmitTransferBetweenSubaccountsRequest;
    pub use self::rest::SubmitTransferBetweenSubaccountsResponse;
    pub use self::rest::SubmitTransferToSubaccountRequest;
    pub use self::rest::SubmitTransferToSubaccountResponse;
    pub use self::rest::SubmitTransferToUserRequest;
    pub use self::rest::SubmitTransferToUserResponse;
    pub use self::rest::Trade;
    pub use self::rest::TransferData;
    pub use self::rest::UpdateInAddressBookRequest;
    pub use self::rest::UpdateInAddressBookResponse;
    pub use self::rest::WithdrawRequest;
    pub use self::rest::WithdrawResponse;
    pub use self::rest::WithdrawalData;
    pub use self::rest::{IndexName, MmpConfig, ResetMmpRequest, ResetMmpResponse, SetMmpConfigRequest, SetMmpConfigResponse};
    pub use self::rest::GetOrderMarginByIdsRequest;
    pub use self::rest::GetOrderMarginByIdsResponse;
    pub use self::rest::OrderMarginInfo;
}

pub mod message;

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use message::*;
pub use private::AddToAddressBookRequest;
pub use private::AddToAddressBookResponse;
pub use private::AddressBookEntry;
pub use private::CancelAllByCurrencyPairRequest;
pub use private::CancelAllByCurrencyPairResponse;
pub use private::CancelAllByCurrencyRequest;
pub use private::CancelAllByCurrencyResponse;
pub use private::CancelAllRequest;
pub use private::CancelAllResponse;
pub use private::CancelBlockRfqRequest;
pub use private::CancelBlockRfqResponse;
pub use private::CancelOnDisconnectResult;
pub use private::CancelOnDisconnectScope;
pub use private::CancelOrderRequest;
pub use private::CancelOrderResponse;
pub use private::CancelQuotesRequest;
pub use private::CancelQuotesResponse;
pub use private::CancelType;
pub use private::CancelWithdrawalRequest;
pub use private::CancelWithdrawalResponse;
pub use private::CancelledOrder;
pub use private::CreateComboLeg;
pub use private::CreateComboRequest;
pub use private::CreateComboResponse;
pub use private::CreateComboResult;
pub use private::CreateComboTrade;
pub use private::CreateDepositAddressRequest;
pub use private::CreateDepositAddressResponse;
pub use private::DepositAddress;
pub use private::DepositData;
pub use private::DepositId;
pub use private::DisableCancelOnDisconnectRequest;
pub use private::DisableCancelOnDisconnectResponse;
pub use private::EnableCancelOnDisconnectRequest;
pub use private::EnableCancelOnDisconnectResponse;
pub use private::GetAddressBookRequest;
pub use private::GetAddressBookResponse;
pub use private::GetCancelOnDisconnectRequest;
pub use private::GetCancelOnDisconnectResponse;
pub use private::GetCurrentDepositAddressRequest;
pub use private::GetCurrentDepositAddressResponse;
pub use private::GetDepositsRequest;
pub use private::GetDepositsResponse;
pub use private::GetDepositsResult;
pub use private::GetOpenOrdersByCurrencyRequest;
pub use private::GetOpenOrdersByCurrencyResponse;
pub use private::OpenOrder;
pub use private::OpenOrderType;
pub use private::GetUserTradesByCurrencyAndTimeRequest;
pub use private::GetUserTradesByCurrencyAndTimeResponse;
pub use private::GetUserTradesByCurrencyAndTimeResult;
pub use private::GetUserTradesByCurrencyRequest;
pub use private::GetUserTradesByCurrencyResponse;
pub use private::GetUserTradesByCurrencyResult;
pub use private::IndexName;
pub use private::InvalidateBlockTradeSignatureRequest;
pub use private::InvalidateBlockTradeSignatureResponse;
pub use private::MovePositionTrade;
pub use private::MovePositionTradeResult;
pub use private::MovePositionsRequest;
pub use private::MovePositionsResponse;
pub use private::MovePositionsResult;
pub use private::Originator;
pub use private::RemoveFromAddressBookRequest;
pub use private::RemoveFromAddressBookResponse;
pub use private::ResetMmpRequest;
pub use private::ResetMmpResponse;
pub use private::RestClient as PrivateRestClient;
pub use private::SendRfqRequest;
pub use private::SendRfqResponse;
pub use private::SetClearanceOriginatorRequest;
pub use private::SetClearanceOriginatorResponse;
pub use private::SetClearanceOriginatorResult;
pub use private::Side;
pub use private::SubaccountTransferData;
pub use private::SubmitTransferBetweenSubaccountsRequest;
pub use private::SubmitTransferBetweenSubaccountsResponse;
pub use private::SubmitTransferToSubaccountRequest;
pub use private::SubmitTransferToSubaccountResponse;
pub use private::SubmitTransferToUserRequest;
pub use private::SubmitTransferToUserResponse;
pub use private::Trade;
pub use private::TransferData;
pub use private::UpdateInAddressBookRequest;
pub use private::UpdateInAddressBookResponse;
pub use private::WithdrawRequest;
pub use private::WithdrawResponse;
pub use private::WithdrawalData;
pub use public::GetComboIdsRequest;
pub use public::GetComboIdsResponse;
pub use public::GetStatusRequest;
pub use public::GetStatusResponse;
pub use public::GetStatusResult;
pub use public::GetTimeRequest;
pub use public::GetTimeResponse;
pub use public::HelloRequest;
pub use public::HelloResponse;
pub use public::HelloResult;
pub use public::PrivateWebSocketClient;
pub use public::RestClient as PublicRestClient;
pub use public::SubscribeRequest;
pub use public::SubscribeResponse;
pub use public::websocket::client::DeribitWebSocketError;
pub use rate_limit::*;

/// Type alias for results returned by Deribit API operations
pub type RestResult<T> = Result<T, Errors>;
