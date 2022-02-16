use crate::common::TargetTrait;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericParam, Generics, TypeParam};

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
            GenericParam::Const(_) => unimplemented!(),
        })
        .collect()
}

/// Use the generic arguments.
///
/// i.e. `impl ... for Foo<'_, '_, T>`
pub(super) fn make_unbounded_generics(generics: &Generics) -> Vec<TokenStream> {
    generics
        .params
        .iter()
        .map(|param| match param {
            GenericParam::Type(TypeParam { ident, .. }) => quote!(#ident),
            GenericParam::Lifetime(_) => quote!('_),
            GenericParam::Const(_) => unimplemented!(),
        })
        .collect()
}

/// Declare the generics arguments and constraints.
///
/// i.e. `impl <T: ToBoundedStatic> for ...`
pub(super) fn make_bounded_generics(generics: &Generics, target: TargetTrait) -> Vec<TokenStream> {
    let bound = target.bound();
    generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Type(TypeParam { ident, .. }) => Some(quote!(#ident: #bound)),
            _ => None,
        })
        .collect()
}
