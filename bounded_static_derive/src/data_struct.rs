use crate::common;
use crate::common::make_bounded_generics;
use crate::common::TargetTrait;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Field, FieldsNamed, FieldsUnnamed, Generics};

/// Generate `ToBoundedStatic` and `IntoBoundedStatic` impls for a `struct` with named fields deriving `ToStatic`.
pub(super) fn generate_struct_named(
    name: &Ident,
    generics: &Generics,
    fields_named: &FieldsNamed,
) -> TokenStream {
    fields_named.named.iter().for_each(common::check_field);
    let to_static_fields =
        make_named_fields_init_methods(fields_named, TargetTrait::ToBoundedStatic);
    let into_static_fields =
        make_named_fields_init_methods(fields_named, TargetTrait::IntoBoundedStatic);
    let to_static_bounded_generics =
        common::make_bounded_generics(generics, TargetTrait::ToBoundedStatic);
    let into_static_bounded_generics =
        common::make_bounded_generics(generics, TargetTrait::IntoBoundedStatic);
    let unbounded_generics = common::make_unbounded_generics(generics);
    let target_generics = common::make_target_generics(generics);
    quote!(
        impl <#(#to_static_bounded_generics),*> ::bounded_static::ToBoundedStatic for #name <#(#unbounded_generics),*> {
            type Static = #name<#(#target_generics),*>;

            fn to_static(&self) -> Self::Static {
                #name {
                    #(#to_static_fields),*
                }
            }
        }
        impl <#(#into_static_bounded_generics),*> ::bounded_static::IntoBoundedStatic for #name <#(#unbounded_generics),*> {
            type Static = #name<#(#target_generics),*>;

            fn into_static(self) -> Self::Static {
                #name {
                    #(#into_static_fields),*
                }
            }
        }
    )
}

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
    let unbounded_generics = common::make_unbounded_generics(generics);
    let target_generics = common::make_target_generics(generics);
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

/// Generate `ToBoundedStatic` and `IntoBoundedStatic` impls for a unit `struct` deriving `ToStatic`.
pub(super) fn generate_struct_unit(name: &Ident) -> TokenStream {
    quote!(
        impl ::bounded_static::ToBoundedStatic for #name {
            type Static = #name;

            fn to_static(&self) -> Self::Static {
                #name
            }
        }
        impl ::bounded_static::IntoBoundedStatic for #name {
            type Static = #name;

            fn into_static(self) -> Self::Static {
                #name
            }
        }
    )
}

fn make_named_fields_init_methods(
    fields_named: &FieldsNamed,
    target: TargetTrait,
) -> Vec<TokenStream> {
    fields_named
        .named
        .iter()
        .map(|field| make_named_field_init_method(field, target))
        .collect()
}

/// i.e. `foo: self.foo.to_static()`
fn make_named_field_init_method(field: &Field, target: TargetTrait) -> TokenStream {
    let field_name = field
        .ident
        .as_ref()
        .expect("FieldsNamed field must have an ident");
    let method = target.method();
    quote!(#field_name: self.#field_name.#method())
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
