// Existing modules
mod cancel_multiple_orders;
mod cancel_order;
mod client;
mod currency_config;
mod deposit_address;
mod deposit_records;
mod get_all_account_balance;
mod get_asset_transfer;
mod get_balances;
mod get_commission_rate;
mod get_fund_balance;
mod get_open_orders;
mod get_order_history;
mod get_trade_history;
mod get_uid;
mod internal_transfer_apply;
mod internal_transfer_records;
mod place_order;
mod post_transfer;
mod query_order;
mod query_transferable_coins;
mod withdraw;
mod withdraw_records;

// Trading endpoints
mod cancel_all_after;
mod cancel_all_orders;
mod cancel_replace_order;
mod place_multiple_orders;

// OCO endpoints
mod cancel_oco_order;
mod create_oco_order;
mod get_oco_order_history;
mod get_open_oco_orders;
mod query_oco_order;

// Sub-account endpoints
mod authorize_sub_account_transfer;
mod batch_sub_account_assets;
mod create_sub_account;
mod delete_sub_account_api_key;
mod edit_sub_account_api_key;
mod freeze_sub_account;
mod get_sub_account_assets;
mod get_sub_account_list;
mod get_sub_account_transfer_history;
mod sub_account_api_key;
mod sub_account_transfer;

// Sub-account exports
pub use authorize_sub_account_transfer::{
    AuthorizeSubAccountTransferRequest, AuthorizeSubAccountTransferResponse,
};
pub use batch_sub_account_assets::{
    BatchSubAccountAssetsRequest, BatchSubAccountAssetsResponse, SubAccountAssetSummary,
};
// Trading exports
pub use cancel_all_after::{CancelAllAfterRequest, CancelAllAfterResponse};
pub use cancel_all_orders::{CancelAllOrdersRequest, CancelAllOrdersResponse};
// OCO exports
pub use cancel_oco_order::{CancelOcoOrderRequest, CancelOcoOrderResponse};
pub use cancel_replace_order::{CancelReplaceOrderRequest, CancelReplaceOrderResponse};
pub use client::RestClient;
pub use create_oco_order::{CreateOcoOrderRequest, CreateOcoOrderResponse, OcoOrderInfo};
pub use create_sub_account::{CreateSubAccountRequest, CreateSubAccountResponse};
// Wallet/Fund exports
pub use currency_config::{CurrencyConfig, GetCurrencyConfigRequest, GetCurrencyConfigResponse};
pub use delete_sub_account_api_key::{
    DeleteSubAccountApiKeyRequest, DeleteSubAccountApiKeyResponse,
};
pub use deposit_address::{DepositAddress, GetDepositAddressRequest, GetDepositAddressResponse};
pub use deposit_records::{
    DepositRecord, GetDepositRecordsRequest as DepositRecordsRequest,
    GetDepositRecordsResponse as DepositRecordsResponse,
};
pub use edit_sub_account_api_key::{EditSubAccountApiKeyRequest, EditSubAccountApiKeyResponse};
pub use freeze_sub_account::{FreezeSubAccountRequest, FreezeSubAccountResponse};
pub use get_asset_transfer::{
    AssetTransferRecord, GetAssetTransferRecordsRequest as AssetTransferRecordsRequest,
    GetAssetTransferRecordsResponse as AssetTransferRecordsResponse,
};
pub use get_balances::{Balance, GetBalancesRequest, GetBalancesResponse};
pub use get_oco_order_history::{
    GetOcoOrderRequest, GetOcoOrderResponse, OcoOrderInfo as OcoOrderHistoryEntry,
};
pub use get_open_oco_orders::{GetOpenOcoOrdersRequest, GetOpenOcoOrdersResponse, OpenOcoOrder};
pub use get_sub_account_assets::{
    GetSubAccountAssetsRequest, GetSubAccountAssetsResponse, SubAccountAsset,
};
pub use get_sub_account_list::{
    GetSubAccountListRequest, GetSubAccountListResponse, SubAccountInfo,
};
pub use get_sub_account_transfer_history::{
    GetSubAccountTransferHistoryRequest, GetSubAccountTransferHistoryResponse,
    SubAccountTransferRecord,
};
pub use internal_transfer_apply::{InternalTransferApplyRequest, InternalTransferApplyResponse};
pub use internal_transfer_records::{
    InternalTransferRecord, InternalTransferRecordsData, InternalTransferRecordsRequest,
    InternalTransferRecordsResponse,
};
pub use place_multiple_orders::{
    BatchOrderResponse as BatchOrderResult, PlaceMultipleOrdersRequest, PlaceMultipleOrdersResponse,
};
pub use post_transfer::{AssetTransferRequest, AssetTransferResponse};
pub use query_oco_order::{QueryOcoOrderRequest, QueryOcoOrderResponse};
pub use query_transferable_coins::{
    CoinAsset, QueryTransferableCoinsRequest, QueryTransferableCoinsResponse,
};
pub use sub_account_api_key::{
    ApiKeyInfo, CreateSubAccountApiKeyRequest, CreateSubAccountApiKeyResponse, QueryApiKeyRequest,
    QueryApiKeyResponse,
};
pub use sub_account_transfer::{SubAccountTransferRequest, SubAccountTransferResponse};
pub use withdraw::{WithdrawRequest, WithdrawResponse};
pub use withdraw_records::{
    GetWithdrawRecordsRequest as WithdrawRecordsRequest,
    GetWithdrawRecordsResponse as WithdrawRecordsResponse, WithdrawRecord,
};
