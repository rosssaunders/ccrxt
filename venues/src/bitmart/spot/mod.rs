pub mod enums;
mod errors;

pub mod rate_limit;
pub mod public {
    pub mod rest;
    pub use self::rest::{
        Currency, DepthData, GetCurrencyListRequest, GetCurrencyListResponse, GetDepthRequest,
        GetDepthResponse, GetHistoryKlineRequest, GetHistoryKlineResponse, GetLatestKlineRequest,
        GetLatestKlineResponse, GetRecentTradesRequest, GetRecentTradesResponse,
        GetTickerAllPairsRequest, GetTickerAllPairsResponse, GetTickerRequest, GetTickerResponse,
        GetTradingPairDetailsRequest, GetTradingPairDetailsResponse, GetTradingPairsListRequest,
        GetTradingPairsListResponse, HistoryKlineData, LatestKlineData, OrderBookEntry, RestClient,
        TickerArrayData, TickerData, TradeData, TradingPairDetail,
    };
}
pub mod private {
    mod rest;
    pub use self::rest::{
        BatchOrderData, BatchOrderParam, CancelAllOrdersRequest, CancelAllOrdersResponse,
        CancelBatchOrderRequest, CancelBatchOrderResponse, CancelOrderRequest, CancelOrderResponse,
        Currency, DepositWithdrawDetail, DepositWithdrawRecord, GetAccountBalanceRequest,
        GetAccountBalanceResponse, GetActualTradeFeeRateRequest, GetActualTradeFeeRateResponse,
        GetBasicFeeRateRequest, GetBasicFeeRateResponse, GetCurrenciesRequest,
        GetCurrenciesResponse, GetDepositAddressRequest, GetDepositAddressResponse,
        GetDepositWithdrawDetailRequest, GetDepositWithdrawDetailResponse,
        GetDepositWithdrawHistoryRequest, GetDepositWithdrawHistoryResponse,
        GetMarginIsolatedAccountRequest, GetMarginIsolatedAccountResponse,
        GetSpotWalletBalanceRequest, GetSpotWalletBalanceResponse, GetWithdrawAddressListRequest,
        GetWithdrawAddressListResponse, GetWithdrawQuotaRequest, GetWithdrawQuotaResponse,
        MarginAssetBase, MarginAssetQuote, MarginAssetTransferRequest, MarginAssetTransferResponse,
        MarginIsolatedSymbol, OrderDetails, QueryOrderRequest, QueryOrderResponse,
        QueryOrderTradesRequest, QueryOrderTradesResponse, QueryOrdersRequest, QueryOrdersResponse,
        QueryTradesRequest, QueryTradesResponse, RestClient, SpotWalletBalance,
        SubmitBatchOrderRequest, SubmitBatchOrderResponse, SubmitMarginOrderRequest,
        SubmitMarginOrderResponse, SubmitOrderRequest, SubmitOrderResponse, TradeInfo,
        WalletBalance, WithdrawAddress, WithdrawRequest, WithdrawResponse,
    };
}

pub mod error;

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use private::{
    BatchOrderData, BatchOrderParam, CancelAllOrdersRequest, CancelAllOrdersResponse,
    CancelBatchOrderRequest, CancelBatchOrderResponse, CancelOrderRequest, CancelOrderResponse,
    Currency, DepositWithdrawDetail, DepositWithdrawRecord, GetAccountBalanceRequest,
    GetAccountBalanceResponse, GetActualTradeFeeRateRequest, GetActualTradeFeeRateResponse,
    GetBasicFeeRateRequest, GetBasicFeeRateResponse, GetCurrenciesRequest, GetCurrenciesResponse,
    GetDepositAddressRequest, GetDepositAddressResponse, GetDepositWithdrawDetailRequest,
    GetDepositWithdrawDetailResponse, GetDepositWithdrawHistoryRequest,
    GetDepositWithdrawHistoryResponse, GetMarginIsolatedAccountRequest,
    GetMarginIsolatedAccountResponse, GetSpotWalletBalanceRequest, GetSpotWalletBalanceResponse,
    GetWithdrawAddressListRequest, GetWithdrawAddressListResponse, GetWithdrawQuotaRequest,
    GetWithdrawQuotaResponse, MarginAssetBase, MarginAssetQuote, MarginAssetTransferRequest,
    MarginAssetTransferResponse, MarginIsolatedSymbol, OrderDetails, QueryOrderRequest,
    QueryOrderResponse, QueryOrderTradesRequest, QueryOrderTradesResponse, QueryOrdersRequest,
    QueryOrdersResponse, QueryTradesRequest, QueryTradesResponse, RestClient as PrivateRestClient,
    SpotWalletBalance, SubmitBatchOrderRequest, SubmitBatchOrderResponse, SubmitMarginOrderRequest,
    SubmitMarginOrderResponse, SubmitOrderRequest, SubmitOrderResponse, TradeInfo, WalletBalance,
    WithdrawAddress, WithdrawRequest, WithdrawResponse,
};
pub use public::{
    Currency as PublicCurrency, DepthData, GetCurrencyListRequest, GetCurrencyListResponse,
    GetDepthRequest, GetDepthResponse, GetHistoryKlineRequest, GetHistoryKlineResponse,
    GetLatestKlineRequest, GetLatestKlineResponse, GetRecentTradesRequest, GetRecentTradesResponse,
    GetTickerAllPairsRequest, GetTickerAllPairsResponse, GetTickerRequest, GetTickerResponse,
    GetTradingPairDetailsRequest, GetTradingPairDetailsResponse, GetTradingPairsListRequest,
    GetTradingPairsListResponse, HistoryKlineData, LatestKlineData, OrderBookEntry,
    RestClient as PublicRestClient, TickerArrayData, TickerData as PublicTickerData, TradeData,
    TradingPairDetail,
};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by BitMart API operations
pub type RestResult<T> = Result<T, Errors>;
