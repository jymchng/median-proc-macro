use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use proc_macro_error::proc_macro_error;

mod implement;

#[proc_macro_error]
#[proc_macro]
pub fn median(ts: TokenStream) -> TokenStream {
    if ts.is_empty() {
        return quote!(
            0
        ).into()
    }
    implement::median_impl(ts).into()
}