use proc_macro2::Ident;
use std::num::NonZeroUsize;
use syn::{
    parse::{Parse, ParseStream},
    Error, LitInt, Result, Token,
};

const LRU_ARGS_IDENT: &str = "max";

pub struct LruArgs {
    pub max_ident: Ident,
    pub eq_token: Token![=],
    pub cap: NonZeroUsize,
}

impl Parse for LruArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let max_ident = input.parse::<Ident>()?;
        let eq_token = input.parse()?;
        let cap = input.parse::<LitInt>()?.base10_parse::<NonZeroUsize>()?;

        if max_ident != LRU_ARGS_IDENT {
            return Err(Error::new(
                max_ident.span(),
                format!("#[memo(max = {})]", cap),
            ));
        }

        Ok(Self {
            max_ident,
            eq_token,
            cap,
        })
    }
}
