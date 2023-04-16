#![feature(let_chains)]
#![feature(decl_macro)]
use quote::*;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Type, parse_quote};

mod with_builder;

#[proc_macro_derive(WithBuilder)]
pub fn struct_with_builder(item: TokenStream) -> TokenStream {
    with_builder::with_builder(item)
}

