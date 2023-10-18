use proc_macro::TokenStream;
use syn::{parse_macro_input, Error, ItemFn};

#[proc_macro_attribute]
pub fn memo(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as ItemFn);

    expand::memo_impl(parsed_input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// TODO: new idea
///
/// ```ignore
/// // this is the new function
/// pub fn fib(args: types**) -> return_type {
///     // this is the old function
///     fn fib_internal(args: types**, map: &mut HashMap<types**, return_type>) -> return_type {
///         memo early return predicate
///         functions stmts
///     }
///     
///     let mut map = HashMap::new();
///     return fib_internal(args**, &mut map)
/// }
/// ```
mod expand {
    use std::collections::VecDeque;

    use proc_macro2::{Span, TokenStream as TokenStream2};
    use quote::{format_ident, quote};
    use syn::{
        token::Else, Error, FnArg, ItemFn, Pat, PatType, Receiver, Result, ReturnType, Stmt,
    };

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

        let fnarg_types = parsed_input
            .sig
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                FnArg::Typed(PatType { ty, .. }) => Some(ty),
                FnArg::Receiver(Receiver { ty: _, .. }) => None,
            })
            .collect::<Vec<_>>();

        let fnarg_names = parsed_input.sig.inputs.iter().filter_map(|arg| match arg {
            FnArg::Typed(PatType { pat, .. }) => match **pat {
                Pat::Ident(ref ident) => Some(ident),
                _ => None,
            },
            FnArg::Receiver(_) => None,
        });

        let return_type = match parsed_input.sig.output {
            ReturnType::Type(_arrow, ty) => *ty,
            ReturnType::Default => {
                return Err(Error::new(
                    Span::call_site(),
                    // double triple check this is correct, you never know
                    "There is no use in memoizing functions that don't return",
                ));
            }
        };

        // to remove the tuple parens if key is only one element
        let memo_fnargs = match fnarg_types.len() > 1 {
            true => syn::parse2::<FnArg>(quote! {
                memo: &mut std::collections::HashMap<(#(#fnarg_types),*), #return_type>
            })?,
            false => syn::parse2::<FnArg>(quote! {
                memo: &mut std::collections::HashMap<#(#fnarg_types)*, #return_type>
            })?,
        };

        let memo_stmts = syn::parse2::<Stmt>(quote! {
            if let Some(ref result) = memo.get(&(#(#fnarg_names),*)) {
                return result;
            }
        })?;

        let memo_insert_stmt = syn::parse2::<Stmt>(quote! {
            memo.insert(, );
        })?;

        let mut new_fn_stmts = VecDeque::from(new_fn.block.stmts.clone());
        new_fn_stmts.push_front(memo_stmts);
        let return_stmt = new_fn_stmts.pop_back().unwrap(); // rememeber to re-insert it
        new_fn_stmts.push_back(memo_insert_stmt);
        new_fn_stmts.push_back(return_stmt);

        new_fn.sig.ident = format_ident!("{}_memo", parsed_input.sig.ident);
        new_fn.sig.inputs.push(memo_fnargs);
        new_fn.block.stmts = new_fn_stmts.into();

        let new_fn_tokens = quote!(#new_fn);
        Ok(new_fn_tokens)

        // return Err(Error::new(Span::call_site(), new_fn_tokens));
        // return Err(Error::new(Span::call_site(), memo_code));
    }
}
