//! Provides the `ToStatic` derive macro.
//!
//! The [`ToStatic`] derive macro implements the [`ToBoundedStatic`] and [`IntoBoundedStatic`] traits for any `struct`
//! and `enum` that can be converted to a form that is bounded by `'static`.
//!
//! It support all `struct` flavors (unit, named & unnamed), all `enum` variant flavors (unit, named & unnamed).  It
//! does not currently support `union`.
//!
//! # Examples
//!
//! ```rust
//! # use std::borrow::Cow;
//! # use std::collections::HashMap;
//! # use bounded_static::ToStatic;
//! /// Named field struct
//! #[derive(ToStatic)]
//! struct Foo<'a> {
//!     aaa: Cow<'a, str>,
//!     bbb: &'static str,
//!     ccc: Baz<'a>,
//! }
//!
//! /// Unnamed field struct
//! #[derive(ToStatic)]
//! struct Bar<'a, 'b>(u128, HashMap<Cow<'a, str>, Cow<'b, str>>);
//!
//! /// Unit struct
//! #[derive(ToStatic)]
//! struct Qux;
//!
//! #[derive(ToStatic)]
//! enum Baz<'a> {
//!     First(String, usize, Vec<Cow<'a, str>>),
//!     Second { fst: u32, snd: &'static str },
//!     Third,
//! }
//! ```
//!
//! See the documentation for [`bounded_static`](https://) for details of the [`ToBoundedStatic`] and
//! [`IntoBoundedStatic`] traits.
#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![forbid(unsafe_code)]

mod common;
mod data_enum;
mod data_struct_named;
mod data_struct_unit;
mod data_struct_unnamed;
mod generate;
mod generics;

/// The `ToStatic` derive macro.
///
/// See the root module for documentation and examples.
#[proc_macro_derive(ToStatic)]
pub fn to_static(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    proc_macro::TokenStream::from(generate::generate(&input))
}
