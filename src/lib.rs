mod expand;
mod lru_args;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Error, ItemFn};

use lru_args::LruArgs;

/// A macro for creating a static lazy LRU cache with a specified maximum size.
///
/// This macro can be applied to functions to enable memoization using an LRU (Least Recently Used)
/// cache. It will generate a static lazy LRU cache that stores function results based on
/// the provided maximum size. When the cache is full, the least recently used items will be
/// evicted.
///
/// # Example
/// ```
/// # use my_crate::lru_cache;
/// #[lru_cache(max = 10)]
/// fn fib(n: u128) -> u128 {
///     if n < 2 {
///         return n;
///     }
///     fib(n - 1) + fib(n - 2)
/// }
///
/// let very_big_number = fib(186); // Computes and caches the result
/// ```
#[proc_macro_attribute]
pub fn lru_cache(args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as ItemFn);
    let parsed_args = parse_macro_input!(args as LruArgs);

    expand::lru_cache_impl(parsed_args, parsed_input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
