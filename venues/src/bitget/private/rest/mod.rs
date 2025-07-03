mod client;
mod get_account_assets;
mod spot;

// New wallet and account endpoints
mod account_info;
mod bgb_deduct_info;
mod bills;
mod cancel_withdrawal;
mod deposit_address;
mod deposit_records;
mod main_sub_transfer_record;
mod modify_deposit_account;
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
pub use client::RestClient;

// Re-export existing endpoints
pub use get_account_assets::{AssetInfo, GetAccountAssetsRequest, GetAccountAssetsResponse};

// Re-export new account endpoints
pub use account_info::{AccountInfo, AccountInfoRequest, AccountInfoResponse, VipInfo};
pub use bills::{BillInfo, BillsRequest, BillsResponse};
pub use transfer::{TransferRequest, TransferResponse};

// Re-export new wallet endpoints
pub use bgb_deduct_info::{BgbDeductInfo, GetBgbDeductInfoRequest, GetBgbDeductInfoResponse};
pub use cancel_withdrawal::{CancelWithdrawalRequest, CancelWithdrawalResponse};
pub use deposit_address::{
    DepositAddressInfo, GetDepositAddressRequest, GetDepositAddressResponse,
};
pub use deposit_records::{DepositRecord, GetDepositRecordsRequest, GetDepositRecordsResponse};
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
pub use transfer_record::{GetTransferRecordRequest, GetTransferRecordResponse, TransferRecord};
pub use transferable_coin_list::{GetTransferableCoinListRequest, GetTransferableCoinListResponse};
pub use withdraw::{WithdrawRequest, WithdrawResponse, WithdrawResult};
pub use withdrawal_records::{
    GetWithdrawalRecordsRequest, GetWithdrawalRecordsResponse, WithdrawalRecord,
};

// Re-export spot trading endpoints
pub use spot::*;
