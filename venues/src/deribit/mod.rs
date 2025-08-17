//! Deribit trading platform implementation
//!
//! This module provides rate limiting and private REST API endpoints for the Deribit API.
//! Deribit uses a credit-based rate limiting system with different tiers based
//! on trading volume.
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use std::sync::Arc;
//! use venues::deribit::{RateLimiter, AccountTier, EndpointType, PublicRestClient, GetComboIdsRequest, Currency};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a rate limiter for a Tier 3 account (1-25M USD trading volume)
//!     let limiter = RateLimiter::new(AccountTier::Tier3);
//!    
//!     // Create a public REST client
//!     let http_client = Arc::new(rest::native::NativeHttpClient::default());
//!     let rest_client = PublicRestClient::new("https://www.deribit.com", http_client, limiter);
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
//!     let status_response = rest_client.get_status().await?;
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

    pub use self::{
        rest::{
            AprHistoryData, BookSummary, BookSummaryByInstrument, DeliveryPriceRecord,
            FundingChartDataPoint, FundingRateHistoryData, GetAprHistoryRequest,
            GetAprHistoryResponse, GetAprHistoryResult, GetBookSummaryByCurrencyRequest,
            GetBookSummaryByCurrencyResponse, GetBookSummaryByInstrumentRequest,
            GetBookSummaryByInstrumentResponse, GetComboDetailsRequest, GetComboDetailsResponse,
            GetComboIdsRequest, GetComboIdsResponse, GetCombosRequest, GetCombosResponse,
            GetContractSizeRequest, GetContractSizeResponse, GetContractSizeResult,
            GetDeliveryPricesRequest, GetDeliveryPricesResponse, GetDeliveryPricesResult,
            GetExpirationsRequest, GetExpirationsResponse, GetExpirationsResult,
            GetFundingChartDataRequest, GetFundingChartDataResponse, GetFundingChartDataResult,
            GetFundingRateHistoryRequest, GetFundingRateHistoryResponse,
            GetFundingRateHistoryResult, GetFundingRateValueRequest,
            GetHistoricalVolatilityRequest, GetHistoricalVolatilityResponse,
            GetHistoricalVolatilityResult, GetIndexPriceNamesRequest, GetIndexPriceNamesResponse,
            GetIndexPriceNamesResult, GetIndexPriceRequest, GetIndexPriceResponse,
            GetIndexPriceResult, GetIndexRequest, GetIndexResponse, GetIndexResult,
            GetInstrumentRequest, GetInstrumentResponse, GetInstrumentsRequest,
            GetInstrumentsResponse, GetLastSettlementsByCurrencyRequest,
            GetLastSettlementsByCurrencyResponse, GetLastSettlementsByCurrencyResult,
            GetLastSettlementsByInstrumentRequest, GetLastSettlementsByInstrumentResponse,
            GetLastSettlementsByInstrumentResult, GetLastTradesByCurrencyAndTimeRequest,
            GetLastTradesByCurrencyAndTimeResponse, GetLastTradesByCurrencyAndTimeResult,
            GetLastTradesByCurrencyRequest, GetLastTradesByCurrencyResponse,
            GetLastTradesByCurrencyResult, GetLastTradesByInstrumentAndTimeRequest,
            GetLastTradesByInstrumentAndTimeResponse, GetLastTradesByInstrumentAndTimeResult,
            GetLastTradesByInstrumentRequest, GetLastTradesByInstrumentResponse,
            GetLastTradesByInstrumentResult, GetMarkPriceHistoryRequest,
            GetMarkPriceHistoryResponse, GetOrderBookByInstrumentIdRequest,
            GetOrderBookByInstrumentIdResponse, GetOrderBookByInstrumentIdResult,
            GetOrderBookRequest, GetOrderBookResponse, GetOrderBookResult, GetRfqsRequest,
            GetRfqsResponse, GetStatusResponse, GetStatusResult, GetTimeResponse,
            GetTradeVolumesRequest, GetTradeVolumesResponse, GetTradingviewChartDataRequest,
            GetTradingviewChartDataResponse, GetTradingviewChartDataResult,
            GetVolatilityIndexDataRequest, GetVolatilityIndexDataResponse,
            GetVolatilityIndexDataResult, InstrumentData, MarkPriceEntry, RestClient, RfqEntry,
            SettlementEntry, TradeEntry, TradeVolumeEntry,
        },
        websocket::{
            HelloRequest, HelloResponse, HelloResult, PrivateWebSocketClient, SubscribeRequest,
            SubscribeResponse, client::DeribitWebSocketError,
        },
    };
}

pub mod private {
    pub mod rest;

    pub use self::rest::{
        AddToAddressBookRequest, AddToAddressBookResponse, AddressBookEntry,
        CancelAllByCurrencyPairRequest, CancelAllByCurrencyPairResponse,
        CancelAllByCurrencyRequest, CancelAllByCurrencyResponse, CancelAllRequest,
        CancelAllResponse, CancelBlockRfqRequest, CancelBlockRfqResponse, CancelOnDisconnectResult,
        CancelOnDisconnectScope, CancelOrderRequest, CancelOrderResponse, CancelQuotesRequest,
        CancelQuotesResponse, CancelType, CancelWithdrawalRequest, CancelWithdrawalResponse,
        CancelledOrder, CreateComboLeg, CreateComboRequest, CreateComboResponse, CreateComboResult,
        CreateComboTrade, CreateDepositAddressRequest, CreateDepositAddressResponse,
        DepositAddress, DepositData, DepositId, DisableCancelOnDisconnectRequest,
        DisableCancelOnDisconnectResponse, EnableCancelOnDisconnectRequest,
        EnableCancelOnDisconnectResponse, GetAddressBookRequest, GetAddressBookResponse,
        GetCancelOnDisconnectRequest, GetCancelOnDisconnectResponse,
        GetCurrentDepositAddressRequest, GetCurrentDepositAddressResponse, GetDepositsRequest,
        GetDepositsResponse, GetDepositsResult, GetOpenOrdersByCurrencyRequest,
        GetOpenOrdersByCurrencyResponse, GetOrderMarginByIdsRequest, GetOrderMarginByIdsResponse,
        GetUserTradesByCurrencyAndTimeRequest, GetUserTradesByCurrencyAndTimeResponse,
        GetUserTradesByCurrencyAndTimeResult, GetUserTradesByCurrencyRequest,
        GetUserTradesByCurrencyResponse, GetUserTradesByCurrencyResult, IndexName,
        InvalidateBlockTradeSignatureRequest, InvalidateBlockTradeSignatureResponse, MmpConfig,
        MovePositionTrade, MovePositionTradeResult, MovePositionsRequest, MovePositionsResponse,
        OpenOrder, OpenOrderType, OrderMarginInfo, Originator, RemoveFromAddressBookRequest,
        RemoveFromAddressBookResponse, ResetMmpRequest, ResetMmpResponse, RestClient,
        SendRfqRequest, SendRfqResponse, SetClearanceOriginatorRequest,
        SetClearanceOriginatorResponse, SetClearanceOriginatorResult, SetMmpConfigRequest,
        SetMmpConfigResponse, Side, SubaccountTransferData,
        SubmitTransferBetweenSubaccountsRequest, SubmitTransferBetweenSubaccountsResponse,
        SubmitTransferToSubaccountRequest, SubmitTransferToSubaccountResponse,
        SubmitTransferToUserRequest, SubmitTransferToUserResponse, Trade, TransferData,
        UpdateInAddressBookRequest, UpdateInAddressBookResponse, WithdrawRequest, WithdrawResponse,
        WithdrawalData,
    };
}

pub mod message;

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use message::{JsonRpcResult, *};
pub use private::{
    AddToAddressBookRequest, AddToAddressBookResponse, AddressBookEntry,
    CancelAllByCurrencyPairRequest, CancelAllByCurrencyPairResponse, CancelAllByCurrencyRequest,
    CancelAllByCurrencyResponse, CancelAllRequest, CancelAllResponse, CancelBlockRfqRequest,
    CancelBlockRfqResponse, CancelOnDisconnectResult, CancelOnDisconnectScope, CancelOrderRequest,
    CancelOrderResponse, CancelQuotesRequest, CancelQuotesResponse, CancelType,
    CancelWithdrawalRequest, CancelWithdrawalResponse, CancelledOrder, CreateComboLeg,
    CreateComboRequest, CreateComboResponse, CreateComboResult, CreateComboTrade,
    CreateDepositAddressRequest, CreateDepositAddressResponse, DepositAddress, DepositData,
    DepositId, DisableCancelOnDisconnectRequest, DisableCancelOnDisconnectResponse,
    EnableCancelOnDisconnectRequest, EnableCancelOnDisconnectResponse, GetAddressBookRequest,
    GetAddressBookResponse, GetCancelOnDisconnectRequest, GetCancelOnDisconnectResponse,
    GetCurrentDepositAddressRequest, GetCurrentDepositAddressResponse, GetDepositsRequest,
    GetDepositsResponse, GetDepositsResult, GetOpenOrdersByCurrencyRequest,
    GetOpenOrdersByCurrencyResponse, GetUserTradesByCurrencyAndTimeRequest,
    GetUserTradesByCurrencyAndTimeResponse, GetUserTradesByCurrencyAndTimeResult,
    GetUserTradesByCurrencyRequest, GetUserTradesByCurrencyResponse, GetUserTradesByCurrencyResult,
    IndexName, InvalidateBlockTradeSignatureRequest, InvalidateBlockTradeSignatureResponse,
    MovePositionTrade, MovePositionTradeResult, MovePositionsRequest, MovePositionsResponse,
    OpenOrder, OpenOrderType, Originator, RemoveFromAddressBookRequest,
    RemoveFromAddressBookResponse, ResetMmpRequest, ResetMmpResponse,
    RestClient as PrivateRestClient, SendRfqRequest, SendRfqResponse,
    SetClearanceOriginatorRequest, SetClearanceOriginatorResponse, SetClearanceOriginatorResult,
    Side, SubaccountTransferData, SubmitTransferBetweenSubaccountsRequest,
    SubmitTransferBetweenSubaccountsResponse, SubmitTransferToSubaccountRequest,
    SubmitTransferToSubaccountResponse, SubmitTransferToUserRequest, SubmitTransferToUserResponse,
    Trade, TransferData, UpdateInAddressBookRequest, UpdateInAddressBookResponse, WithdrawRequest,
    WithdrawResponse, WithdrawalData,
};
// Re-export specialized types from get_expirations
pub use public::rest::get_expirations::{ExpirationsCurrency, ExpirationsInstrumentKind};
pub use public::{
    AprHistoryData, BookSummary, BookSummaryByInstrument, DeliveryPriceRecord,
    FundingChartDataPoint, FundingRateHistoryData, GetAprHistoryRequest, GetAprHistoryResponse,
    GetAprHistoryResult, GetBookSummaryByCurrencyRequest, GetBookSummaryByCurrencyResponse,
    GetBookSummaryByInstrumentRequest, GetBookSummaryByInstrumentResponse, GetComboDetailsRequest,
    GetComboDetailsResponse, GetComboIdsRequest, GetComboIdsResponse, GetCombosRequest,
    GetCombosResponse, GetContractSizeRequest, GetContractSizeResponse, GetContractSizeResult,
    GetDeliveryPricesRequest, GetDeliveryPricesResponse, GetDeliveryPricesResult,
    GetExpirationsRequest, GetExpirationsResponse, GetExpirationsResult,
    GetFundingChartDataRequest, GetFundingChartDataResponse, GetFundingChartDataResult,
    GetFundingRateHistoryRequest, GetFundingRateHistoryResponse, GetFundingRateHistoryResult,
    GetFundingRateValueRequest, GetHistoricalVolatilityRequest, GetHistoricalVolatilityResponse,
    GetHistoricalVolatilityResult, GetIndexPriceNamesRequest, GetIndexPriceNamesResponse,
    GetIndexPriceNamesResult, GetIndexPriceRequest, GetIndexPriceResponse, GetIndexPriceResult,
    GetIndexRequest, GetIndexResponse, GetIndexResult, GetInstrumentRequest, GetInstrumentResponse,
    GetInstrumentsRequest, GetInstrumentsResponse, GetLastSettlementsByCurrencyRequest,
    GetLastSettlementsByCurrencyResponse, GetLastSettlementsByCurrencyResult,
    GetLastSettlementsByInstrumentRequest, GetLastSettlementsByInstrumentResponse,
    GetLastSettlementsByInstrumentResult, GetLastTradesByCurrencyAndTimeRequest,
    GetLastTradesByCurrencyAndTimeResponse, GetLastTradesByCurrencyAndTimeResult,
    GetLastTradesByCurrencyRequest, GetLastTradesByCurrencyResponse, GetLastTradesByCurrencyResult,
    GetLastTradesByInstrumentAndTimeRequest, GetLastTradesByInstrumentAndTimeResponse,
    GetLastTradesByInstrumentAndTimeResult, GetLastTradesByInstrumentRequest,
    GetLastTradesByInstrumentResponse, GetLastTradesByInstrumentResult, GetMarkPriceHistoryRequest,
    GetMarkPriceHistoryResponse, GetOrderBookByInstrumentIdRequest,
    GetOrderBookByInstrumentIdResponse, GetOrderBookByInstrumentIdResult, GetOrderBookRequest,
    GetOrderBookResponse, GetOrderBookResult, GetRfqsRequest, GetRfqsResponse, GetStatusResponse,
    GetStatusResult, GetTimeResponse, GetTradeVolumesRequest, GetTradeVolumesResponse,
    GetTradingviewChartDataRequest, GetTradingviewChartDataResponse, GetTradingviewChartDataResult,
    GetVolatilityIndexDataRequest, GetVolatilityIndexDataResponse, GetVolatilityIndexDataResult,
    HelloRequest, HelloResponse, HelloResult, InstrumentData, MarkPriceEntry,
    PrivateWebSocketClient, RestClient as PublicRestClient, RfqEntry, SettlementEntry,
    SubscribeRequest, SubscribeResponse, TradeEntry, TradeVolumeEntry,
    websocket::client::DeribitWebSocketError,
};
pub use rate_limit::*;

/// Type alias for results returned by Deribit API operations
pub type RestResult<T> = Result<T, Errors>;
