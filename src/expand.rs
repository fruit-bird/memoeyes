use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use std::num::NonZeroUsize;
use syn::{Error, FnArg, ItemFn, Pat, PatType, Result, ReturnType};

use crate::{add_fn_arg::AddFnArg, lru_args::LruArgs};

pub fn lru_cache_impl(parsed_args: LruArgs, mut parsed_input: ItemFn) -> Result<TokenStream2> {
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

    if input_names.is_empty() {
        return Err(Error::new(
            Span::call_site(),
            "There is no use in memoizing functions that don't have any inputs",
        ));
    }

    let input_tys = parsed_input.sig.inputs.iter().filter_map(|arg| match arg {
        FnArg::Typed(PatType { ty, .. }) => Some(ty),
        FnArg::Receiver(_) => None,
    });

    let return_ty = match parsed_input.sig.output {
        ReturnType::Type(_, ref ty) => ty,
        ReturnType::Default => {
            return Err(Error::new(
                Span::call_site(),
                "There is no use in memoizing functions that don't return",
            ));
        }
    };

    let lru_cap = parsed_args.cap.base10_parse::<NonZeroUsize>()?.get();
    let fn_name = parsed_input.sig.ident.to_string().to_uppercase();
    let cache_ident = format_ident!("{}_CACHE", fn_name);
    let cache_tokens = quote! {
        use lru::LruCache;
        use once_cell::sync::Lazy;
        use std::num::NonZeroUsize;

        static mut #cache_ident: Lazy<LruCache<(#(#input_tys),*), #return_ty>> =
            Lazy::new(|| LruCache::new(unsafe { NonZeroUsize::new_unchecked(#lru_cap) }));
    };

    let fn_body_block = parsed_input.block;
    let fn_block_tokens = quote! {
        {
            unsafe {
                if let Some(result) = #cache_ident.get(&(#(#input_names),*)) {
                    return *result;
                }

                let result = { #fn_body_block };
                #cache_ident.put((#(#input_names),*), result);
                result
            }
        }
    };

    parsed_input.block = syn::parse2(fn_block_tokens)?;

    Ok(quote! {
        #cache_tokens
        #parsed_input
    })
}

pub fn memo_impl(mut parsed_input: ItemFn) -> Result<TokenStream2> {
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

    if input_names.is_empty() {
        return Err(Error::new(
            Span::call_site(),
            "There is no use in memoizing functions that don't have any inputs",
        ));
    }

    let input_tys = parsed_input.sig.inputs.iter().filter_map(|arg| match arg {
        FnArg::Typed(PatType { ty, .. }) => Some(ty),
        FnArg::Receiver(_) => None,
    });

    let return_ty = match parsed_input.sig.output {
        ReturnType::Type(_, ref ty) => ty,
        ReturnType::Default => {
            return Err(Error::new(
                Span::call_site(),
                "There is no use in memoizing functions that don't return",
            ));
        }
    };

    let memo_tokens =
        quote! { memo: &mut std::collections::HashMap<(#(#input_tys),*), #return_ty> };
    let memo = syn::parse2::<FnArg>(memo_tokens)?;

    let memo_check_tokens = quote! {
        if let Some(result) = memo.get(&(#(#input_names),*)) {
            return *result;
        }
    };

    let fn_ident = &parsed_input.sig.ident;
    let arg_tokens = &quote! { memo };
    let updated_fn_block_tokens = parsed_input
        .block
        .stmts
        .iter()
        .map(|stmt| stmt.to_token_stream().add_fn_arg(fn_ident, arg_tokens));

    let fn_body_and_memo_insert = quote! {
        let result = {
            #(#updated_fn_block_tokens)*
        };
        memo.insert((#(#input_names),*), result);
        result
    };

    let new_block_tokens = quote! {
        {
            #memo_check_tokens
            #fn_body_and_memo_insert
        }
    };

    // Modifying the function
    parsed_input.sig.inputs.push(memo);
    parsed_input.block = syn::parse2(new_block_tokens)?;

    Ok(quote! { #parsed_input })
}
