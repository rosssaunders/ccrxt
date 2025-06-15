pub mod client;
pub mod update_in_address_book;

pub use client::RestClient;
pub use update_in_address_book::{
    UpdateInAddressBookRequest, UpdateInAddressBookResponse, Currency, AddressType,
};