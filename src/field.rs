use std::iter::FromIterator;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, format_ident};
use syn::Meta::{List, Path, NameValue};
use syn::NestedMeta::{self, Meta};

use super::checker::Checker;

enum AccessType {
    GetRef((syn::Path, Option<syn::Type>)),
    GetRefMut(syn::Path),
    Set(syn::Path)
}

pub struct Field<'a> {
    ident: &'a syn::Ident,
    typ: &'a syn::Type,
    accessors: Vec<AccessType>
}

impl<'a> Field<'a> {
    pub fn from_ast(chk: &Checker, ast_field: &'a syn::Field) -> Self {
        let ident = ast_field.ident.as_ref().unwrap();
        let typ = &ast_field.ty;
        let accessors: Vec<AccessType> = ast_field.attrs.iter()
            .flat_map(|attr| { get_access_attributes(chk, attr) })
            .flatten()
            .map(|attr| { parse_meta(chk, attr) })
            .flatten()
            .collect();

        Self {
            ident,
            typ,
            accessors
        }
    }

    pub fn expand(&self) -> TokenStream {
        let ident = self.ident;
        let typ = self.typ;

        let field_expanded = self.accessors.iter().map(|access| {
            match access {
                AccessType::GetRef((_, Some(typ_override))) => {
                    let fn_name = format_ident!("{}", ident);
                    quote_spanned! {ident.span()=>
                        pub fn #fn_name(&self) -> #typ_override {
                            &self.#ident
                        }
                    }
                },
                AccessType::GetRef(_) => {
                    let fn_name = format_ident!("{}", ident);
                    quote_spanned! {ident.span()=>
                        pub fn #fn_name(&self) -> &#typ {
                            &self.#ident
                        }
                    }
                },
                AccessType::GetRefMut(_) => {
                    let fn_name = format_ident!("{}_mut", ident);
                    quote_spanned! {ident.span()=>
                        pub fn #fn_name(&mut self) -> &mut #typ {
                            &mut self.#ident
                        }
                    }
                }
                AccessType::Set(_) => {
                    let fn_name = format_ident!("set_{}", ident);
                    quote_spanned! {ident.span()=>
                        pub fn #fn_name(&mut self, new_val: #typ) {
                            self.#ident = new_val
                        }
                    }
                }
            }
        });

        quote!(#(#field_expanded)*)
    }
}

fn get_access_attributes(chk: &Checker, attr: &syn::Attribute) -> Result<Vec<NestedMeta>, ()> {
    if !attr.path.is_ident("access") {
        return Ok(Vec::new())
    }

    match attr.parse_meta() {
        Ok(List(meta)) => return Ok(Vec::from_iter(meta.nested)),
        Ok(other) => chk.append_error_from(other, "expected #[access(...)]"),
        Err(bad) => chk.append_syn_error(bad)
    };

    Err(())
}

fn parse_meta(chk: &Checker, meta: NestedMeta) -> Vec<AccessType> {
    match &meta {
        Meta(Path(path)) if path.is_ident("get") => {
            vec![AccessType::GetRef((path.clone(), None))]
        },
        Meta(Path(path)) if path.is_ident("set") => {
            vec![AccessType::Set(path.clone())]
        },
        Meta(List(list)) if list.path.is_ident("get") => {
            list.nested.iter().flat_map(|meta| { parse_get_attributes(chk, meta) }).collect()
        },
        bad => {
            chk.append_error_from(bad, "unexpected literal in #[access(...)]");
            Vec::new()
        }
    }
}

fn parse_get_attributes(chk: &Checker, meta: &NestedMeta) -> Result<AccessType, ()> {
    match meta {
        Meta(Path(path)) if path.is_ident("ref") => {
            Ok(AccessType::GetRef((path.clone(), None)))
        }
        Meta(Path(path)) if path.is_ident("ref_mut") => {
            Ok(AccessType::GetRefMut(path.clone()))
        }
        Meta(NameValue(name_value)) if name_value.path.is_ident("ref") => {
            get_type_from_lit(chk, &name_value.lit).map(|typ| { AccessType::GetRef((name_value.path.clone(), Some(typ))) })
        }
        bad => {
            chk.append_error_from(bad, "#[access(get(...))] only takes `ref` or `ref_mut` values");
            Err(())
        }
    }
}

fn get_type_from_lit(chk: &Checker, lit: &syn::Lit) -> Result<syn::Type, ()> {
    match lit {
        syn::Lit::Str(token) => {
            token.parse().map_err(|err| { chk.append_syn_error(err) })
        },
        bad => {
            chk.append_error_from(bad, "`#[access(get(ref = ...))]` only takes literal string");
            Err(())
        }
    }
}
