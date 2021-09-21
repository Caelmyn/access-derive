#![allow(dead_code)]

use access_derive::Access;

#[test]
fn gen_basic() {
    #[derive(Access)]
    struct Test {
        #[access(get(ref, ref_mut), set)]
        var1: String,
        var2: u32
    }
}

#[test]
fn gen_with_generics() {
    #[derive(Access)]
    struct Test<T> {
        #[access(get(ref, ref_mut), set)]
        var1: T,
        var2: u32
    }
}

#[test]
fn gen_with_generics_and_where_clauses() {
    #[derive(Access)]
    struct Test<T> where T: Copy {
        #[access(get(ref, ref_mut), set)]
        var1: T,
        var2: u32
    }
}

#[test]
fn gen_with_lifetimes() {
    #[derive(Access)]
    struct Test<'a> {
        #[access(get(ref, ref_mut), set)]
        var1: &'a str,
        var2: u32
    }
}
