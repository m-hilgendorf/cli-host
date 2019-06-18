#![recursion_limit="128"]
//! modified from the original com-impl crate
//!
//todo: document vst3-derive

#[macro_use] extern crate quote;
#[macro_use] extern crate syn;

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use syn::DeriveInput;
use syn::{AttributeArgs, Item};

mod derive;
mod vst3impl;

#[proc_macro_derive(Vst3Impl, attributes(interfaces))]
pub fn derive_vst3_impl (input : TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive::expand_derive_vst3_impl(&input)
        .unwrap_or_else(compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn vst3_impl(attr: TokenStream, item : TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let item = parse_macro_input!(item as Item);

    vst3impl::expand_vst3_impl (&args, &item)
        .unwrap_or_else(compile_error)
        .into()
}

fn compile_error(message : String)-> proc_macro2::TokenStream {
    quote!{
        compile_error!(#message);
    }
}