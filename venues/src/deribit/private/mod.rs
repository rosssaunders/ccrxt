mod rest;
pub mod websocket;

pub use rest::RestClient;
pub use rest::{
    AddToAddressBookRequest, AddToAddressBookResponse, AddressBookEntry,
    CancelByLabelRequest, CancelByLabelResponse,
    CancelOnDisconnectResult, CancelOnDisconnectScope,
    CreateDepositAddressRequest, CreateDepositAddressResponse,
    DepositAddress, DisableCancelOnDisconnectRequest, DisableCancelOnDisconnectResponse,
    EnableCancelOnDisconnectRequest, EnableCancelOnDisconnectResponse,
    GetAddressBookRequest, GetAddressBookResponse,
    GetCancelOnDisconnectRequest, GetCancelOnDisconnectResponse,
    GetCurrentDepositAddressRequest, GetCurrentDepositAddressResponse,
    RemoveFromAddressBookRequest, RemoveFromAddressBookResponse,
    SubaccountTransferData, SubmitTransferToSubaccountRequest, SubmitTransferToSubaccountResponse,
    SubmitTransferToUserRequest, SubmitTransferToUserResponse, TransferData,
    UpdateInAddressBookRequest, UpdateInAddressBookResponse,
};