// Existing modules
mod cancel_multiple_orders;
mod cancel_order;
mod client;
mod get_all_account_balance;
mod get_balances;
mod get_commission_rate;
mod get_fund_balance;
mod get_open_orders;
mod get_order_history;
mod get_trade_history;
mod get_uid;
mod place_order;
mod query_order;

// Wallet/Fund endpoints
mod asset_transfer;
mod asset_transfer_records;
mod currency_config;
mod deposit_address;
mod deposit_records;
mod internal_transfer_records;
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

// Margin endpoints
mod get_margin_account;

// Risk and monitoring endpoints
mod risk_records;

pub use client::RestClient;
pub use get_balances::{Balance, GetBalancesRequest, GetBalancesResponse};

// Wallet/Fund exports
pub use asset_transfer::{AssetTransferRequest, AssetTransferResponse};
pub use asset_transfer_records::{GetAssetTransferRecordsRequest as AssetTransferRecordsRequest, GetAssetTransferRecordsResponse as AssetTransferRecordsResponse, AssetTransferRecord};
pub use currency_config::{GetCurrencyConfigRequest, GetCurrencyConfigResponse, CurrencyConfig};
pub use deposit_address::{GetDepositAddressRequest, GetDepositAddressResponse, DepositAddress};
pub use deposit_records::{GetDepositRecordsRequest as DepositRecordsRequest, GetDepositRecordsResponse as DepositRecordsResponse, DepositRecord};
pub use internal_transfer_records::{InternalTransferRecordsRequest, InternalTransferRecordsResponse, InternalTransferRecord};
pub use withdraw::{WithdrawRequest, WithdrawResponse};
pub use withdraw_records::{GetWithdrawRecordsRequest as WithdrawRecordsRequest, GetWithdrawRecordsResponse as WithdrawRecordsResponse, WithdrawRecord};

// Trading exports
pub use cancel_all_after::{CancelAllAfterRequest, CancelAllAfterResponse};
pub use cancel_all_orders::{CancelAllOrdersRequest, CancelAllOrdersResponse};
pub use cancel_replace_order::{CancelReplaceOrderRequest, CancelReplaceOrderResponse};
pub use place_multiple_orders::{PlaceMultipleOrdersRequest, PlaceMultipleOrdersResponse, BatchOrderResponse as BatchOrderResult};

// OCO exports
pub use cancel_oco_order::{CancelOcoOrderRequest, CancelOcoOrderResponse};
pub use create_oco_order::{CreateOcoOrderRequest, CreateOcoOrderResponse};
pub use get_oco_order_history::{GetOcoOrderHistoryRequest, GetOcoOrderHistoryResponse, OcoOrderHistory as OcoOrderHistoryEntry};
pub use get_open_oco_orders::{GetOpenOcoOrdersRequest, GetOpenOcoOrdersResponse, OpenOcoOrder};
pub use query_oco_order::{QueryOcoOrderRequest, QueryOcoOrderResponse};
pub use create_oco_order::OcoOrderInfo;

// Sub-account exports
pub use authorize_sub_account_transfer::{AuthorizeSubAccountTransferRequest, AuthorizeSubAccountTransferResponse};
pub use batch_sub_account_assets::{BatchSubAccountAssetsRequest, BatchSubAccountAssetsResponse, SubAccountAssetSummary};
pub use create_sub_account::{CreateSubAccountRequest, CreateSubAccountResponse};
pub use delete_sub_account_api_key::{DeleteSubAccountApiKeyRequest, DeleteSubAccountApiKeyResponse};
pub use edit_sub_account_api_key::{EditSubAccountApiKeyRequest, EditSubAccountApiKeyResponse, ApiKeyInfo};
pub use freeze_sub_account::{FreezeSubAccountRequest, FreezeSubAccountResponse};
pub use get_sub_account_assets::{GetSubAccountAssetsRequest, GetSubAccountAssetsResponse, SubAccountAsset};
pub use get_sub_account_list::{GetSubAccountListRequest, GetSubAccountListResponse, SubAccountInfo};
pub use get_sub_account_transfer_history::{GetSubAccountTransferHistoryRequest, GetSubAccountTransferHistoryResponse, SubAccountTransferRecord};
pub use sub_account_api_key::{CreateSubAccountApiKeyRequest, CreateSubAccountApiKeyResponse};
pub use sub_account_transfer::{SubAccountTransferRequest, SubAccountTransferResponse};

// Margin exports
pub use get_margin_account::{GetMarginAccountRequest, GetMarginAccountResponse, MarginAccountInfo, MarginAsset};

// Risk exports
pub use risk_records::{RiskRecordsRequest, RiskRecordsResponse, RiskRecord};
