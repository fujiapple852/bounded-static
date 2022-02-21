use crate::common;
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
    let to = generate_struct_named_to(name, generics, fields_named);
    let into = generate_struct_named_into(name, generics, fields_named);
    quote!(#to #into)
}

/// Generate `ToBoundedStatic` and `IntoBoundedStatic` impls for a `struct` with unnamed fields deriving `ToStatic`.
pub(super) fn generate_struct_unnamed(
    name: &Ident,
    generics: &Generics,
    fields_unnamed: &FieldsUnnamed,
) -> TokenStream {
    fields_unnamed.unnamed.iter().for_each(common::check_field);
    let to = generate_struct_unnamed_to(name, generics, fields_unnamed);
    let into = generate_struct_unnamed_into(name, generics, fields_unnamed);
    quote!(#to #into)
}

/// Generate `ToBoundedStatic` and `IntoBoundedStatic` impls for a unit `struct` deriving `ToStatic`.
pub(super) fn generate_struct_unit(name: &Ident) -> TokenStream {
    let to = generate_struct_unit_to(name);
    let into = generate_struct_unit_into(name);
    quote!(#to #into)
}

/// Generate `ToBoundedStatic` for a `struct` with with named fields.
fn generate_struct_named_to(
    name: &Ident,
    generics: &Generics,
    fields_named: &FieldsNamed,
) -> TokenStream {
    let fields = make_named_fields_init_methods(fields_named, TargetTrait::ToBoundedStatic);
    let gens = common::make_bounded_generics(generics, TargetTrait::ToBoundedStatic);
    let (impl_gens, ty_gens, where_clause) = gens.split_for_impl();
    let static_gens = common::make_target_generics(generics);
    quote!(
        impl #impl_gens ::bounded_static::ToBoundedStatic for #name #ty_gens #where_clause {
            type Static = #name<#(#static_gens),*>;
            fn to_static(&self) -> Self::Static {
                #name {
                    #(#fields),*
                }
            }
        }
    )
}

/// Generate `IntoBoundedStatic` for a `struct` with with named fields.
fn generate_struct_named_into(
    name: &Ident,
    generics: &Generics,
    fields_named: &FieldsNamed,
) -> TokenStream {
    let fields = make_named_fields_init_methods(fields_named, TargetTrait::IntoBoundedStatic);
    let gens = common::make_bounded_generics(generics, TargetTrait::IntoBoundedStatic);
    let (impl_gens, ty_gens, where_clause) = gens.split_for_impl();
    let static_gens = common::make_target_generics(generics);
    quote!(
        impl #impl_gens ::bounded_static::IntoBoundedStatic for #name #ty_gens #where_clause {
            type Static = #name<#(#static_gens),*>;
            fn into_static(self) -> Self::Static {
                #name {
                    #(#fields),*
                }
            }
        }
    )
}

/// Generate `ToBoundedStatic` for a `struct` with unnamed fields.
fn generate_struct_unnamed_to(
    name: &Ident,
    generics: &Generics,
    fields_unnamed: &FieldsUnnamed,
) -> TokenStream {
    let fields = make_unnamed_fields(fields_unnamed, TargetTrait::ToBoundedStatic);
    let gens = common::make_bounded_generics(generics, TargetTrait::ToBoundedStatic);
    let (impl_gens, ty_gens, where_clause) = gens.split_for_impl();
    let static_gens = common::make_target_generics(generics);
    quote!(
        impl #impl_gens ::bounded_static::ToBoundedStatic for #name #ty_gens #where_clause {
            type Static = #name<#(#static_gens),*>;
            fn to_static(&self) -> Self::Static {
                #name (
                    #(#fields),*
                )
            }
        }
    )
}

/// Generate `IntoBoundedStatic` for a `struct` with unnamed fields.
fn generate_struct_unnamed_into(
    name: &Ident,
    generics: &Generics,
    fields_unnamed: &FieldsUnnamed,
) -> TokenStream {
    let fields = make_unnamed_fields(fields_unnamed, TargetTrait::IntoBoundedStatic);
    let gens = common::make_bounded_generics(generics, TargetTrait::IntoBoundedStatic);
    let (impl_gens, ty_gens, where_clause) = gens.split_for_impl();
    let static_gens = common::make_target_generics(generics);
    quote!(
        impl #impl_gens ::bounded_static::IntoBoundedStatic for #name #ty_gens #where_clause {
            type Static = #name<#(#static_gens),*>;
            fn into_static(self) -> Self::Static {
                #name (
                    #(#fields),*
                )
            }
        }
    )
}

/// Generate `ToBoundedStatic` for unit struct.
fn generate_struct_unit_to(name: &Ident) -> TokenStream {
    quote!(
        impl ::bounded_static::ToBoundedStatic for #name {
            type Static = #name;
            fn to_static(&self) -> Self::Static {
                #name
            }
        }
    )
}

/// Generate `IntoBoundedStatic` for unit struct.
fn generate_struct_unit_into(name: &Ident) -> TokenStream {
    quote!(
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
