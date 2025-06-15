pub mod client;
pub mod update_in_address_book;

#[cfg(test)]
mod integration_tests;

pub use client::RestClient;
pub use update_in_address_book::{
    UpdateInAddressBookRequest, UpdateInAddressBookResponse, Currency, AddressType,
};