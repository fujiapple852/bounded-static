use crate::common::TargetTrait;
use crate::generics::make_bounded_generics;
use crate::{common, generics};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{FieldsUnnamed, Generics};

/// Generate `ToBoundedStatic` and `IntoBoundedStatic` impls for a `struct` with unnamed fields deriving `ToStatic`.
pub(super) fn generate_struct_unnamed(
    name: &Ident,
    generics: &Generics,
    fields_unnamed: &FieldsUnnamed,
) -> TokenStream {
    fields_unnamed.unnamed.iter().for_each(common::check_field);
    let to_static_fields = make_unnamed_fields(fields_unnamed, TargetTrait::ToBoundedStatic);
    let into_static_fields = make_unnamed_fields(fields_unnamed, TargetTrait::IntoBoundedStatic);
    let to_static_bounded_generics = make_bounded_generics(generics, TargetTrait::ToBoundedStatic);
    let into_static_bounded_generics =
        make_bounded_generics(generics, TargetTrait::IntoBoundedStatic);
    let unbounded_generics = generics::make_unbounded_generics(generics);
    let target_generics = generics::make_target_generics(generics);
    quote!(
        impl <#(#to_static_bounded_generics),*> ::bounded_static::ToBoundedStatic for #name <#(#unbounded_generics),*> {
            type Static = #name<#(#target_generics),*>;

            fn to_static(&self) -> Self::Static {
                #name (
                    #(#to_static_fields),*
                )
            }
        }
        impl <#(#into_static_bounded_generics),*> ::bounded_static::IntoBoundedStatic for #name <#(#unbounded_generics),*> {
            type Static = #name<#(#target_generics),*>;

            fn into_static(self) -> Self::Static {
                #name (
                    #(#into_static_fields),*
                )
            }
        }
    )
}

fn make_unnamed_fields(fields_unnamed: &FieldsUnnamed, target: TargetTrait) -> Vec<TokenStream> {
    let fields_to_static: Vec<_> = fields_unnamed
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, _)| make_unnamed_field(i, target))
        .collect();
    fields_to_static
}

/// i.e. `self.0.to_static()`
fn make_unnamed_field(i: usize, target: TargetTrait) -> TokenStream {
    let i = syn::Index::from(i);
    let method = target.method();
    quote!(self.#i.#method())
}
