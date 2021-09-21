use std::cell::RefCell;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub struct Checker {
    errors: RefCell<Vec<syn::Error>>
}

impl Checker {
    pub fn new() -> Self {
        Self { errors: RefCell::new(Vec::new()) }
    }

    pub fn append_error_from<O: ToTokens, M: std::fmt::Display>(&self, obj: O, msg: M) {
        self.errors.borrow_mut().push(syn::Error::new_spanned(obj.into_token_stream(), msg))
    }

    pub fn append_syn_error(&self, err: syn::Error) {
        self.errors.borrow_mut().push(err)
    }

    pub fn validate(&self) -> Result<(), TokenStream> {
        let errors = self.errors.borrow();

        if errors.len() == 0 {
            return Ok(());
        }

        let compile_error = errors.iter().map(|error| { error.to_compile_error() });
        Err(quote!(#(#compile_error)*))
    }
}
