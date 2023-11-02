use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Error, FnArg, ItemFn, Pat, PatType, Result, ReturnType};

use crate::lru_args::LruArgs;

pub fn lru_cache_impl(parsed_args: LruArgs, parsed_input: ItemFn) -> Result<TokenStream2> {
    let lru_cap = parsed_args.cap.get();

    let fn_name = parsed_input.sig.ident.to_string().to_uppercase();
    let cache_ident = format_ident!("{}_CACHE", fn_name);

    let input_names = parsed_input
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(PatType { pat, .. }) => match **pat {
                Pat::Ident(ref ident) => Some(ident),
                _ => None,
            },
            FnArg::Receiver(_) => None,
        })
        .collect::<Vec<_>>();

    let input_tys = parsed_input.sig.inputs.iter().filter_map(|arg| match arg {
        FnArg::Typed(PatType { ty, .. }) => Some(ty),
        FnArg::Receiver(_) => None,
    });

    let return_ty = match parsed_input.sig.output {
        ReturnType::Type(_, ref ty) => ty,
        ReturnType::Default => {
            return Err(Error::new(
                Span::call_site(),
                // double triple check this is correct, you never know
                "There is no use in memoizing functions that return ()",
            ));
        }
    };

    let cache_tokens = quote! {
        use lru::LruCache;
        use once_cell::sync::Lazy;
        use std::num::NonZeroUsize;

        static mut #cache_ident: Lazy<LruCache<(#(#input_tys),*), #return_ty>> =
            Lazy::new(|| LruCache::new(unsafe { NonZeroUsize::new_unchecked(#lru_cap) }));
    };

    // NB: From here we here we assume that the function
    // returns on the final line of the function w/o any ifs or matches

    // SAFETY: function body cannot be empty
    // since we guard against functions that return () earlier
    let return_stmt = parsed_input.block.stmts.last().unwrap();
    let other_stmts = &parsed_input.block.stmts[..parsed_input.block.stmts.len() - 1];

    let body_tokens = quote! {
        #(#other_stmts)*
        let result = unsafe {
            if let Some(result) = #cache_ident.get(&(#(#input_names),*)) {
                return *result;
            }

            let result = #return_stmt;
            #cache_ident.put((#(#input_names),*), result);

            // SAFETY: We just inserted the value
            FIB_CACHE.get(&n).unwrap_unchecked()
        };

        *result
    };

    let attrs = parsed_input.attrs;
    let sig = parsed_input.sig;
    let tokens = quote! {
        #cache_tokens
        #(#attrs)*
        #sig {
            #body_tokens
        }
    };

    // return Err(Error::new(Span::call_site(), tokens));
    Ok(tokens)
}
