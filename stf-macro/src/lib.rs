#![no_std]

use proc_macro::*;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn simple_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);

    quote!(
        #[test_case]
        #item
    )
    .into()
}
