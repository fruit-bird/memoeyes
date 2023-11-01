use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use std::collections::VecDeque;
use syn::{Error, FnArg, ItemFn, Pat, PatType, Receiver, Result, ReturnType};

use crate::replace::Replace;

/// NEW TODO: push a `&mut HashMap` as an fn arg
///
/// ```ignore
/// fn memoized_sum(
///     a: usize,
///     b: usize
///     memo: &mut HashMap<(usize, usize), usize>,
/// ) -> usize {
///     if let Some(&result) = memo.get(&(a, b)) {
///         return result;
///     }
///
///     let result = a + b + memoized_sum(memo, a - 1, b - 1);
///     memo.insert((a, b), result);
///     result
/// }
/// ```
pub fn memo_impl(mut parsed_input: ItemFn) -> Result<TokenStream2> {
    let mut new_fn = parsed_input.clone();

    let fnarg_types = parsed_input.sig.inputs.iter().filter_map(|arg| match arg {
        FnArg::Typed(PatType { ty, .. }) => Some(ty),
        FnArg::Receiver(Receiver { ty: _, .. }) => None,
    });

    let fnarg_names = parsed_input
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

    let return_type = match parsed_input.sig.output {
        ReturnType::Type(_arrow, ty) => ty,
        ReturnType::Default => {
            return Err(Error::new(
                Span::call_site(),
                // double triple check this is correct, you never know
                "There is no use in memoizing functions that return ()",
            ));
        }
    };

    let memo_fnarg = quote! {
        memo: &mut std::collections::HashMap<(#(#fnarg_types),*), #return_type>
    };

    let memo_early_return_stmt = quote! {
        if let Some(result) = memo.get(&(#(#fnarg_names),*)) {
            return result.clone();
        }
    };

    let fn_name = &parsed_input.sig.ident;
    let new_fn_name = format_ident!("{}_internal", fn_name);

    let mut internal_fn_stmts = parsed_input.block.stmts;
    // HERE CALL .replace(from, to) and call the var `internal_fn_stmts`
    let return_stmt = internal_fn_stmts.pop().unwrap();
    let internal_fn_stmts_wo_last = internal_fn_stmts;

    let updated_internal_fn_stmts = quote! {
        #memo_early_return_stmt
        #(#internal_fn_stmts_wo_last)*

        let result = #return_stmt;
        memo.insert(&(#(#fnarg_names),*), result);
        result
    };

    // let replaced_recursive_fn_name = internal_fn_stmts
    //     .iter()
    //     .map(|stmt| stmt.to_token_stream().replace(&fn_name, &new_fn_name));

    // return Err(Error::new(Span::call_site(), updated_internal_fn_stmts));

    // finished with internal function, now for the new fn
    let mut new_fn_stmts = VecDeque::from(new_fn.block.stmts);

    // modifying the original function
    parsed_input.sig.ident = format_ident!("{}_internal", parsed_input.sig.ident);
    // parsed_input.sig.inputs.push(memo_fnarg);
    parsed_input.block.stmts = Vec::from(new_fn_stmts);

    todo!()
}
