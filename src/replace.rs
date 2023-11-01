use proc_macro2::{Group, Ident, TokenStream as TokenStream2, TokenTree};

pub trait Replace<T, U> {
    fn replace(self, from: T, to: U) -> Self;
}

impl Replace<&Ident, &Ident> for TokenStream2 {
    // We will use this to replace `fnident` with `fnident_internal`
    // TODO: add the memo argument to the function call at the end/start
    fn replace(self, from: &Ident, to: &Ident) -> Self {
        self.into_iter()
            .map(|tt| {
                match tt {
                    TokenTree::Group(group) => {
                        let delimiter = group.delimiter();
                        let stream = group.stream();
                        TokenTree::Group(Group::new(delimiter, stream))
                    }
                    TokenTree::Ident(ident) if ident == *from => {
                        // Converts the old ident into the new ident
                        // TODO: still has to modify the function args by adding the hashmap
                        TokenTree::Ident(to.clone())
                    }
                    other => other,
                }
            })
            .collect()
    }
}
