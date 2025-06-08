use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PrivateRequest)]
pub fn derive_private_request(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    // This will later check for a timestamp field and generate the impl
    let expanded = quote! {
        impl crate::binance::coinm::private::rest::PrivateRequest for #name {
            fn timestamp(&self) -> u64 {
                self.timestamp
            }
        }
    };
    TokenStream::from(expanded)
}
