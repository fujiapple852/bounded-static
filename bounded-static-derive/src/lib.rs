#![doc(html_root_url = "https://docs.rs/bounded-static-derive/0.4.0")]
//! Provides the `ToStatic` derive macro.
//!
//! The [`ToStatic`] derive macro implements the [`ToBoundedStatic`](https://docs.rs/bounded-static/0.4.0/bounded_static/trait.ToBoundedStatic.html)
//! and [`IntoBoundedStatic`](https://docs.rs/bounded-static/0.4.0/bounded_static/trait.IntoBoundedStatic.html) traits for any `struct`
//! and `enum` that can be converted to a form that is bounded by `'static`.
//!
//! The [`ToStatic`] macro should be used via the [`bounded-static`](https://docs.rs/bounded-static/0.4.0) crate
//! rather than using this crate directly.
#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![allow(clippy::redundant_pub_crate)]
#![forbid(unsafe_code)]

use proc_macro2::TokenStream;
use syn::{Data, DataStruct, DeriveInput, Fields};

mod common;
mod data_enum;
mod data_struct;

/// The `ToStatic` derive macro.
///
/// Generate [`ToBoundedStatic`](https://docs.rs/bounded-static/0.4.0/bounded_static/trait.ToBoundedStatic.html) and
/// [`IntoBoundedStatic`](https://docs.rs/bounded-static/0.4.0/bounded_static/trait.IntoBoundedStatic.html) impls for the data item deriving
/// `ToStatic`.
#[proc_macro_derive(ToStatic)]
pub fn to_static(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    proc_macro::TokenStream::from(generate_traits(&input))
}

fn generate_traits(input: &DeriveInput) -> TokenStream {
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields_named),
            ..
        }) => data_struct::generate_struct_named(&input.ident, &input.generics, fields_named),
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(fields_unnamed),
            ..
        }) => data_struct::generate_struct_unnamed(&input.ident, &input.generics, fields_unnamed),
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => data_struct::generate_struct_unit(&input.ident),
        Data::Enum(data_enum) => data_enum::generate_enum(
            &input.ident,
            &input.generics,
            data_enum.variants.iter().collect::<Vec<_>>().as_slice(),
        ),
        Data::Union(_) => unimplemented!("union is not yet supported"),
    }
}
