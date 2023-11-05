mod add_fn_arg;
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
/// ```ignore
/// #[lru_cache(max = 10)]
/// fn fib(n: u128) -> u128 {
///     if n < 2 {
///         return n;
///     }
///     fib(n - 1) + fib(n - 2)
/// }
///
/// let big_number = fib(186);
/// println!("{}", big_number);
/// // Output: 332825110087067562321196029789634457848
/// ```
#[proc_macro_attribute]
pub fn lru_cache(args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as ItemFn);
    let parsed_args = parse_macro_input!(args as LruArgs);

    expand::lru_cache_impl(parsed_args, parsed_input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// A macro for automatically adding a memoization table for functions in an **explicit** way
/// 
/// Memoization is a technique that caches the results of expensive function calls
/// and reuses them when the same inputs occur again.
/// This can significantly improve the performance of recursive or repetitive functions.
/// 
/// # Usage
/// To use the memo macro, annotate a function with `#[memo]`.
/// The macro will generate code to handle memoization by adding a `&mut HashMap` argument to the function
/// and modifying the function's block to check the cache before performing the computation.
/// ```ignore
/// #[memo]
/// fn fib(n: u128) -> u128 {
///     if n < 2 {
///         return n;
///     }
///     fib(n - 1) + fib(n - 2)
/// }
/// 
/// let mut memo = HashMap::new();
/// let result = fib(186, &mut memo);
/// println!("{}", result);
/// // 332825110087067562321196029789634457848
/// ```
#[proc_macro_attribute]
pub fn memo(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as ItemFn);

    expand::memo_impl(parsed_input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
