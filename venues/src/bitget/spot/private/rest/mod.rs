mod account_info;
mod batch_cancel_orders;
mod batch_cancel_plan_orders;
mod batch_cancel_replace_orders;
mod batch_orders;
mod bgb_deduct_info;
mod bills;
mod cancel_order;
mod cancel_plan_order;
mod cancel_replace_order;
mod cancel_symbol_order;
mod cancel_withdrawal;
mod client;
mod credentials;
mod current_plan_order;
mod deposit_address;
mod deposit_records;
mod get_account_assets;
mod get_current_orders;
mod get_fills;
mod get_order_history;
mod get_order_info;
mod main_sub_transfer_record;
mod modify_deposit_account;
mod modify_plan_order;
mod place_order;
mod place_plan_order;
mod plan_sub_order;
pub mod spot;
mod sub_transfer;
mod subaccount_assets;
mod subaccount_deposit_address;
mod subaccount_deposit_records;
mod switch_bgb_deduct;
mod transfer;
mod transfer_record;
mod transferable_coin_list;
mod withdraw;
mod withdrawal_records;

// Re-export client

// Re-export new account endpoints
pub use account_info::{AccountInfo, AccountInfoRequest, AccountInfoResponse, VipInfo};
// Re-export new wallet endpoints
pub use bgb_deduct_info::{BgbDeductInfo, GetBgbDeductInfoRequest, GetBgbDeductInfoResponse};
pub use bills::{BillInfo, BillsRequest, BillsResponse};
pub use cancel_withdrawal::{CancelWithdrawalRequest, CancelWithdrawalResponse};
pub use client::RestClient;
pub use credentials::Credentials;
pub use deposit_address::{
    DepositAddressInfo, GetDepositAddressRequest, GetDepositAddressResponse,
};
pub use deposit_records::{DepositRecord, GetDepositRecordsRequest, GetDepositRecordsResponse};
// Re-export existing endpoints
pub use get_account_assets::{AssetInfo, GetAccountAssetsRequest, GetAccountAssetsResponse};
pub use main_sub_transfer_record::{
    GetMainSubTransferRecordRequest, GetMainSubTransferRecordResponse, MainSubTransferRecord,
};
pub use modify_deposit_account::{ModifyDepositAccountRequest, ModifyDepositAccountResponse};
pub use sub_transfer::{SubTransferRequest, SubTransferResponse, SubTransferResult};
pub use subaccount_assets::{
    AssetDetail, GetSubaccountAssetsRequest, GetSubaccountAssetsResponse, SubaccountAsset,
};
pub use subaccount_deposit_address::{
    GetSubaccountDepositAddressRequest, GetSubaccountDepositAddressResponse,
};
pub use subaccount_deposit_records::{
    GetSubaccountDepositRecordsRequest, GetSubaccountDepositRecordsResponse,
    SubaccountDepositRecord,
};
pub use switch_bgb_deduct::{SwitchBgbDeductRequest, SwitchBgbDeductResponse};
pub use transfer::{TransferRequest, TransferResponse};
pub use transfer_record::{GetTransferRecordRequest, GetTransferRecordResponse, TransferRecord};
pub use transferable_coin_list::{GetTransferableCoinListRequest, GetTransferableCoinListResponse};
pub use withdraw::{WithdrawRequest, WithdrawResponse, WithdrawResult};
pub use withdrawal_records::{
    GetWithdrawalRecordsRequest, GetWithdrawalRecordsResponse, WithdrawalRecord,
};
