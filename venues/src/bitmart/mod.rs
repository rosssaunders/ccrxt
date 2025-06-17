pub mod enums;
mod errors;

pub mod rate_limit;
pub mod private {
    mod rest;
    pub use self::rest::RestClient;
    pub use self::rest::{GetAccountBalanceRequest, GetAccountBalanceResponse, WalletBalance};
    pub use self::rest::{GetCurrenciesRequest, GetCurrenciesResponse, Currency};
    pub use self::rest::{GetSpotWalletBalanceRequest, GetSpotWalletBalanceResponse, SpotWalletBalance};
    pub use self::rest::{GetDepositAddressRequest, GetDepositAddressResponse};
    pub use self::rest::{GetWithdrawQuotaRequest, GetWithdrawQuotaResponse};
    pub use self::rest::{WithdrawRequest, WithdrawResponse};
    pub use self::rest::{GetWithdrawAddressListRequest, GetWithdrawAddressListResponse, WithdrawAddress};
    pub use self::rest::{GetDepositWithdrawHistoryRequest, GetDepositWithdrawHistoryResponse, DepositWithdrawRecord};
    pub use self::rest::{GetDepositWithdrawDetailRequest, GetDepositWithdrawDetailResponse, DepositWithdrawDetail};
    pub use self::rest::{GetMarginIsolatedAccountRequest, GetMarginIsolatedAccountResponse, MarginIsolatedSymbol, MarginAssetBase, MarginAssetQuote};
    pub use self::rest::{MarginAssetTransferRequest, MarginAssetTransferResponse};
    pub use self::rest::{GetBasicFeeRateRequest, GetBasicFeeRateResponse};
    pub use self::rest::{GetActualTradeFeeRateRequest, GetActualTradeFeeRateResponse};
    pub use self::rest::{
        CancelOrderRequest, CancelOrderResponse, OrderDetails, QueryOrderRequest, QueryOrderResponse,
        QueryOrderTradesRequest, QueryOrderTradesResponse, QueryOrdersRequest, QueryOrdersResponse,
        QueryTradesRequest, QueryTradesResponse, SubmitOrderRequest, SubmitOrderResponse, TradeInfo,
    };
}

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use private::RestClient as PrivateRestClient;
pub use private::{GetAccountBalanceRequest, GetAccountBalanceResponse, WalletBalance};
pub use private::{GetCurrenciesRequest, GetCurrenciesResponse, Currency};
pub use private::{GetSpotWalletBalanceRequest, GetSpotWalletBalanceResponse, SpotWalletBalance};
pub use private::{GetDepositAddressRequest, GetDepositAddressResponse};
pub use private::{GetWithdrawQuotaRequest, GetWithdrawQuotaResponse};
pub use private::{WithdrawRequest, WithdrawResponse};
pub use private::{GetWithdrawAddressListRequest, GetWithdrawAddressListResponse, WithdrawAddress};
pub use private::{GetDepositWithdrawHistoryRequest, GetDepositWithdrawHistoryResponse, DepositWithdrawRecord};
pub use private::{GetDepositWithdrawDetailRequest, GetDepositWithdrawDetailResponse, DepositWithdrawDetail};
pub use private::{GetMarginIsolatedAccountRequest, GetMarginIsolatedAccountResponse, MarginIsolatedSymbol, MarginAssetBase, MarginAssetQuote};
pub use private::{MarginAssetTransferRequest, MarginAssetTransferResponse};
pub use private::{GetBasicFeeRateRequest, GetBasicFeeRateResponse};
pub use private::{GetActualTradeFeeRateRequest, GetActualTradeFeeRateResponse};
pub use private::{
    CancelOrderRequest, CancelOrderResponse, OrderDetails, QueryOrderRequest, QueryOrderResponse,
    QueryOrderTradesRequest, QueryOrderTradesResponse, QueryOrdersRequest, QueryOrdersResponse,
    QueryTradesRequest, QueryTradesResponse, SubmitOrderRequest, SubmitOrderResponse, TradeInfo,
};
pub use rate_limit::{EndpointType, RateLimit, RateLimitError, RateLimiter};

/// Type alias for results returned by BitMart API operations
pub type RestResult<T> = Result<T, Errors>;
