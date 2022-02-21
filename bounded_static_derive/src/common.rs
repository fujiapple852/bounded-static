use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_quote, ConstParam, Field, GenericParam, Generics, Ident, Lifetime, PredicateType, Type,
    TypeParam, WhereClause, WherePredicate,
};

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
/// Note that even without this check the compilation will fail if a non-static reference is used, however by
/// performing this check we can issue a more explicit failure message to the developer.
pub(super) fn check_field(field: &Field) {
    if let Type::Reference(ty) = &field.ty {
        if let Some(Lifetime { ident, .. }) = &ty.lifetime {
            #[allow(clippy::manual_assert)]
            if *ident != "static" {
                panic!(
                    "non-static references cannot be made static: {:?}",
                    quote!(#field).to_string()
                )
            }
        }
    };
}

/// The generic parameters of the `Static` associated type.
///
/// i.e. `Static = Foo<'static, 'static, T::Static, R::Static>`
pub(super) fn make_target_generics(generics: &Generics) -> Vec<TokenStream> {
    generics
        .params
        .iter()
        .map(|param| match param {
            GenericParam::Type(TypeParam { ident, .. }) => quote!(#ident::Static),
            GenericParam::Lifetime(_) => quote!('static),
            GenericParam::Const(ConstParam { ident, .. }) => quote!(#ident),
        })
        .collect()
}

/// Make a `Generics` with generic bounds for `TargetTrait`.
///
/// # Examples
///
/// Given the following struct:
///
/// ```rust
/// # use std::borrow::Cow;
/// struct Baz<'a, T: Into<String> + 'a> {
///     t: T,
///     r: Cow<'a, str>,
///  }
/// ```
///
/// We wish to produce (for example for `ToBoundedStatic`, similar for `ItoBoundedStatic`):
///
/// ```rust
/// # use std::borrow::Cow;
/// # struct Baz<'a, T: Into<String> + 'a> {
/// #    t: T,
/// #    r: Cow<'a, str>,
/// # }
/// impl<'a, T: Into<String> + 'a + ::bounded_static::ToBoundedStatic> ::bounded_static::ToBoundedStatic for Baz<'a, T>
/// where
///     T::Static: Into<String> + 'a {
///
///     type Static = Baz<'static, T::Static>;
///
///     fn to_static(&self) -> Self::Static {
///         Baz { t: self.t.to_static(), r: self.r.to_static() }
///     }
/// }
/// ```
///
/// In the above example we can see that `T` has the bound `Into<String> + 'a` and therefore:
///
/// - Generic parameter `T` has the additional bound `::bounded_static::ToBoundedStatic`
/// - Associated type `T::Static` has the bound of `T`, i.e. `Into<String> + 'a`
///
pub(super) fn make_bounded_generics(generics: &Generics, target: TargetTrait) -> Generics {
    let params = make_bounded_generic_params(generics, target);
    let predicates = make_bounded_generic_predicates(generics, target);
    let static_predicates = make_static_generic_predicates(generics);
    let where_items: Vec<_> = predicates.into_iter().chain(static_predicates).collect();
    Generics {
        params: parse_quote!(#(#params),*),
        where_clause: Some(parse_quote!(where #(#where_items),* )),
        ..*generics
    }
}

/// Make generic parameters bound by `TargetTrait`.
///
/// i.e. given parameter `T: Into<String>` create `T: Into<String> + ::bounded_static::TargetTrait`
fn make_bounded_generic_params(generics: &Generics, target: TargetTrait) -> Vec<GenericParam> {
    generics
        .params
        .iter()
        .map(|param| match param {
            GenericParam::Type(ty) => GenericParam::Type(ty.clone_with_bound(&target.bound())),
            other => other.clone(),
        })
        .collect()
}

/// Make generic predicates bound by `TargetTrait`.
///
/// i.e. given predicate `T: Into<String>` create `T: Into<String> + ::bounded_static::TargetTrait`
fn make_bounded_generic_predicates(
    generics: &Generics,
    target: TargetTrait,
) -> Vec<WherePredicate> {
    match generics.where_clause.as_ref() {
        Some(WhereClause { predicates, .. }) => predicates
            .iter()
            .map(|predicate| match predicate {
                WherePredicate::Type(ty) => {
                    WherePredicate::Type(ty.clone_with_bound(&target.bound()))
                }
                other => other.clone(),
            })
            .collect(),
        None => vec![],
    }
}

/// Make generic predicates for associated item `T::Static` bound as per `T`.
///
/// i.e. given:
///
/// ```rust
/// struct Baz<T: Into<String>> {
///     t: T,
/// }
/// ```
///
/// The generated trait impl must reflect the original generic bounds for the associated type `Static` such that:
/// `T::Static: Into<String>`.
fn make_static_generic_predicates(generics: &Generics) -> Vec<WherePredicate> {
    generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Type(ty) => {
                let var = &ty.ident;
                let bounds = &ty.bounds;
                Some(parse_quote!(#var::Static: #bounds))
            }
            _ => None,
        })
        .collect()
}

/// Clone and add a bound to a type.
trait CloneWithBound {
    fn clone_with_bound(&self, bound: &Ident) -> Self;
}

/// Clone and add a bound to a `PredicateType` (in a `where` clause).
impl CloneWithBound for PredicateType {
    fn clone_with_bound(&self, bound: &Ident) -> Self {
        let mut bounded = self.clone();
        bounded.bounds.push(parse_quote!(::bounded_static::#bound));
        bounded
    }
}

/// Clone and add a bound to a `TypeParam`.
impl CloneWithBound for TypeParam {
    fn clone_with_bound(&self, bound: &Ident) -> Self {
        let mut bounded = self.clone();
        bounded.bounds.push(parse_quote!(::bounded_static::#bound));
        bounded
    }
}
