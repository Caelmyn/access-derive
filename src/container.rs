use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, Fields};

use super::field::Field;
use super::checker::Checker;

pub struct Container<'a> {
    struct_name: &'a syn::Ident,
    struct_generics: &'a syn::Generics,
    fields: Vec<Field<'a>>
}

impl<'a> Container<'a> {
    pub fn from_ast(chk: &Checker, input: &'a syn::DeriveInput) -> Self {
        let struct_name = &input.ident;
        let struct_generics = &input.generics;

        let fields = match &input.data {
            Data::Struct(struc) => {
                get_fields(chk, &struc.fields)
            },
            _ => {
                chk.append_error_from(input, "Access derive only support struct");
                Vec::new()
            }
        };

        Self {
            struct_name,
            struct_generics,
            fields
        }
    }

    pub fn expand(&self) -> TokenStream {
        let struct_name = self.struct_name;
        let (impl_generics, ty_generics, where_clauses) = self.struct_generics.split_for_impl();
        let fields_expanded = self.fields.iter().map(|field| { field.expand() });

        quote! {
            impl #impl_generics #struct_name #ty_generics #where_clauses {
                #(#fields_expanded)*
            }
        }
    }
}

fn get_fields<'a>(chk: &Checker, fields: &'a Fields) -> Vec<Field<'a>> {
    match &fields {
        Fields::Named(fields) => {
            fields.named.iter()
                .filter(|ast_field| { !ast_field.attrs.is_empty() })
                .map(|ast_field| { Field::from_ast(chk, ast_field) })
                .collect()
        }
        bad => {
            chk.append_error_from(bad, "Access derive doesn't support tuple struct");
            Vec::new()
        }
    }
}
