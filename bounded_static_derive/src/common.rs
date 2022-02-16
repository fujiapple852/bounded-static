use quote::{format_ident, quote};
use syn::{Field, Ident, Lifetime, Type};

/// The method and trait bound for both traits we will generate.
#[derive(Copy, Clone)]
pub(super) enum TargetTrait {
    ToBoundedStatic,
    IntoBoundedStatic,
}

impl TargetTrait {
    pub fn method(self) -> Ident {
        match self {
            TargetTrait::ToBoundedStatic => format_ident!("to_static"),
            TargetTrait::IntoBoundedStatic => format_ident!("into_static"),
        }
    }

    pub fn bound(self) -> Ident {
        match self {
            TargetTrait::ToBoundedStatic => format_ident!("ToBoundedStatic"),
            TargetTrait::IntoBoundedStatic => format_ident!("IntoBoundedStatic"),
        }
    }
}

/// Check for references which aren't `'static` and panic.
///
/// # Examples
///
/// The following `struct` cannot be made static _for all_ lifetimes `'a` (it is only valud for the `'static` lifetime)
/// and so will fail this check:
///
/// ```compile_fail
/// #[derive(ToStatic)]
/// struct Foo<'a> {
///   bar: &'a str
/// }
/// ```
///
/// This `struct` will pass validation as the reference is `'static`:
///
/// ```rust
/// # use bounded_static::ToStatic;
/// #[derive(ToStatic)]
/// struct Foo {
///   bar: &'static str
/// }
/// ```
///
/// This `struct` is will also pass validation as it can be converted to `'static` _for all_ lifetimes `'a`:
///
/// ```rust
/// # use bounded_static::ToStatic;
/// #[derive(ToStatic)]
/// struct Foo<'a> {
///   bar: std::borrow::Cow<'a, str>
/// }
/// ```
///
/// Note that even withot this check the compilation will fail if a non-static reference is used, however by performing
/// this check we can issue a more explicit failure message to the developer.
pub(super) fn check_field(field: &Field) {
    if let Type::Reference(ty) = &field.ty {
        if let Some(Lifetime { ident, .. }) = &ty.lifetime {
            if *ident != "static" {
                panic!(
                    "non-static references cannot be made static: {:?}",
                    quote!(#field).to_string()
                )
            }
        }
    };
}
