pub mod enums;
mod errors;

pub mod rate_limit;
pub mod public {
    mod rest;
    pub use self::rest::RestClient;
    pub use self::rest::{Currency, GetCurrencyListRequest, GetCurrencyListResponse};
    pub use self::rest::{DepthData, GetDepthRequest, GetDepthResponse, OrderBookEntry};
    pub use self::rest::{GetHistoryKlineRequest, GetHistoryKlineResponse, HistoryKlineData};
    pub use self::rest::{GetLatestKlineRequest, GetLatestKlineResponse, LatestKlineData};
    pub use self::rest::{GetRecentTradesRequest, GetRecentTradesResponse, TradeData};
    pub use self::rest::{GetTickerAllPairsRequest, GetTickerAllPairsResponse, TickerArrayData};
    pub use self::rest::{GetTickerRequest, GetTickerResponse, TickerData};
    pub use self::rest::{GetTradingPairDetailsRequest, GetTradingPairDetailsResponse, TradingPairDetail};
    pub use self::rest::{GetTradingPairsListRequest, GetTradingPairsListResponse};
}
pub mod private {
    mod rest;
    pub use self::rest::RestClient;
    pub use self::rest::{
        CancelOrderRequest, CancelOrderResponse, OrderDetails, QueryOrderRequest, QueryOrderResponse, QueryOrderTradesRequest, QueryOrderTradesResponse,
        QueryOrdersRequest, QueryOrdersResponse, QueryTradesRequest, QueryTradesResponse, SubmitOrderRequest, SubmitOrderResponse, TradeInfo,
    };
    pub use self::rest::{Currency, GetCurrenciesRequest, GetCurrenciesResponse};
    pub use self::rest::{DepositWithdrawDetail, GetDepositWithdrawDetailRequest, GetDepositWithdrawDetailResponse};
    pub use self::rest::{DepositWithdrawRecord, GetDepositWithdrawHistoryRequest, GetDepositWithdrawHistoryResponse};
    pub use self::rest::{GetAccountBalanceRequest, GetAccountBalanceResponse, WalletBalance};
    pub use self::rest::{GetActualTradeFeeRateRequest, GetActualTradeFeeRateResponse};
    pub use self::rest::{GetBasicFeeRateRequest, GetBasicFeeRateResponse};
    pub use self::rest::{GetDepositAddressRequest, GetDepositAddressResponse};
    pub use self::rest::{GetMarginIsolatedAccountRequest, GetMarginIsolatedAccountResponse, MarginAssetBase, MarginAssetQuote, MarginIsolatedSymbol};
    pub use self::rest::{GetSpotWalletBalanceRequest, GetSpotWalletBalanceResponse, SpotWalletBalance};
    pub use self::rest::{GetWithdrawAddressListRequest, GetWithdrawAddressListResponse, WithdrawAddress};
    pub use self::rest::{GetWithdrawQuotaRequest, GetWithdrawQuotaResponse};
    pub use self::rest::{MarginAssetTransferRequest, MarginAssetTransferResponse};
    pub use self::rest::{WithdrawRequest, WithdrawResponse};
}

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use private::RestClient as PrivateRestClient;
pub use private::{
    CancelOrderRequest, CancelOrderResponse, OrderDetails, QueryOrderRequest, QueryOrderResponse, QueryOrderTradesRequest, QueryOrderTradesResponse,
    QueryOrdersRequest, QueryOrdersResponse, QueryTradesRequest, QueryTradesResponse, SubmitOrderRequest, SubmitOrderResponse, TradeInfo,
};
pub use private::{Currency, GetCurrenciesRequest, GetCurrenciesResponse};
pub use private::{DepositWithdrawDetail, GetDepositWithdrawDetailRequest, GetDepositWithdrawDetailResponse};
pub use private::{DepositWithdrawRecord, GetDepositWithdrawHistoryRequest, GetDepositWithdrawHistoryResponse};
pub use private::{GetAccountBalanceRequest, GetAccountBalanceResponse, WalletBalance};
pub use private::{GetActualTradeFeeRateRequest, GetActualTradeFeeRateResponse};
pub use private::{GetBasicFeeRateRequest, GetBasicFeeRateResponse};
pub use private::{GetDepositAddressRequest, GetDepositAddressResponse};
pub use private::{GetMarginIsolatedAccountRequest, GetMarginIsolatedAccountResponse, MarginAssetBase, MarginAssetQuote, MarginIsolatedSymbol};
pub use private::{GetSpotWalletBalanceRequest, GetSpotWalletBalanceResponse, SpotWalletBalance};
pub use private::{GetWithdrawAddressListRequest, GetWithdrawAddressListResponse, WithdrawAddress};
pub use private::{GetWithdrawQuotaRequest, GetWithdrawQuotaResponse};
pub use private::{MarginAssetTransferRequest, MarginAssetTransferResponse};
pub use private::{WithdrawRequest, WithdrawResponse};
pub use public::RestClient as PublicRestClient;
pub use public::{Currency as PublicCurrency, GetCurrencyListRequest, GetCurrencyListResponse};
pub use public::{DepthData, GetDepthRequest, GetDepthResponse, OrderBookEntry};
pub use public::{GetHistoryKlineRequest, GetHistoryKlineResponse, HistoryKlineData};
pub use public::{GetLatestKlineRequest, GetLatestKlineResponse, LatestKlineData};
pub use public::{GetRecentTradesRequest, GetRecentTradesResponse, TradeData};
pub use public::{GetTickerAllPairsRequest, GetTickerAllPairsResponse, TickerArrayData};
pub use public::{GetTickerRequest, GetTickerResponse, TickerData as PublicTickerData};
pub use public::{GetTradingPairDetailsRequest, GetTradingPairDetailsResponse, TradingPairDetail};
pub use public::{GetTradingPairsListRequest, GetTradingPairsListResponse};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by BitMart API operations
pub type RestResult<T> = Result<T, Errors>;
