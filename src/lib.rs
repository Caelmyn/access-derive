use syn::{self, parse_macro_input, DeriveInput};

mod container;
mod field;
mod checker;

use quote::quote;

#[proc_macro_derive(Access, attributes(access))]
pub fn access_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let checker = checker::Checker::new();

    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    let container = container::Container::from_ast(&checker, &ast);

    let errors = checker.validate().err();
    let cont_expanded = container.expand();

    let exp = quote!{
        #errors
        #cont_expanded
    };

    proc_macro::TokenStream::from(exp)
}