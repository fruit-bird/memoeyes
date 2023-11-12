use proc_macro2::{Delimiter, Group, Ident, TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens};

pub trait AddFnArg {
    fn add_fn_arg(self, fn_ident: &Ident, arg_tokens: &TokenStream2) -> Self;
}

impl AddFnArg for TokenStream2 {
    fn add_fn_arg(self, fn_ident: &Ident, arg_tokens: &TokenStream2) -> Self {
        let mut inside_function_call = false;
        self.into_iter()
            .map(|tt| match tt {
                TokenTree::Ident(ref ident) if ident == fn_ident => {
                    inside_function_call = true;
                    ident.into_token_stream()
                }
                TokenTree::Group(group) if inside_function_call => {
                    let delimiter = Delimiter::None;
                    let stream = group.stream().add_fn_arg(fn_ident, arg_tokens);
                    let group_tokens = Group::new(delimiter, stream);

                    inside_function_call = false;
                    quote! { (#group_tokens, #arg_tokens) }
                }
                TokenTree::Group(group) => {
                    let delimiter = group.delimiter();
                    let stream = group.stream().add_fn_arg(fn_ident, arg_tokens);
                    Group::new(delimiter, stream).into_token_stream()
                }
                other => other.into_token_stream(),
            })
            .collect()
    }
}
