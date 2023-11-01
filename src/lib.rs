mod expand;
mod replace;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Error, ItemFn};

#[proc_macro_attribute]
pub fn memo(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as ItemFn);

    expand::memo_impl(parsed_input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
