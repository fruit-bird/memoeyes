use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::{Error, FnArg, ItemFn, Pat, PatType, Result, ReturnType};

use crate::{add_fn_arg::AddFnArg, lru_args::LruArgs};

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
                "There is no use in memoizing functions that don't return",
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

    let fn_block_tokens = quote! {
        {
            #(#other_stmts)*
            let result = unsafe {
                if let Some(result) = #cache_ident.get(&(#(#input_names),*)) {
                    return *result;
                }

                let result = #return_stmt;
                #cache_ident.put((#(#input_names),*), result);

                // SAFETY: We just inserted the value
                FIB_CACHE.get(&n).unwrap()
            };

            *result
        }
    };

    let attrs = parsed_input.attrs;
    let vis = parsed_input.vis;
    let sig = parsed_input.sig;
    let tokens = quote! {
        #cache_tokens
        #(#attrs)* #vis #sig #fn_block_tokens
    };

    // return Err(Error::new(Span::call_site(), tokens));
    Ok(tokens)
}

pub fn memo_impl(parsed_input: ItemFn) -> Result<TokenStream2> {
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

    // Rebuilding the function
    let attrs = parsed_input.attrs;
    let vis = parsed_input.vis;
    let mut sig = parsed_input.sig;
    sig.inputs.push(memo);
    let block = new_block_tokens;

    let memoed_fn_tokens = quote! {
        #(#attrs)* #vis #sig #block
    };

    Ok(memoed_fn_tokens)
}
