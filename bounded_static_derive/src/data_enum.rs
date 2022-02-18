use crate::common;
use crate::common::TargetTrait;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Fields, FieldsNamed, FieldsUnnamed, Generics, Variant};

/// Generate `ToBoundedStatic` and `IntoBoundedStatic` impls for an `enum` deriving `ToStatic`.
pub(super) fn generate_enum(
    name: &Ident,
    generics: &Generics,
    variants: &[&Variant],
) -> TokenStream {
    variants
        .iter()
        .for_each(|v| v.fields.iter().for_each(common::check_field));
    let to_variants = generate_variants(name, variants, TargetTrait::ToBoundedStatic);
    let into_variants = generate_variants(name, variants, TargetTrait::IntoBoundedStatic);
    let to_bounded_generics = common::make_bounded_generics(generics, TargetTrait::ToBoundedStatic);
    let into_bounded_generics =
        common::make_bounded_generics(generics, TargetTrait::IntoBoundedStatic);
    let unbounded_generics = common::make_unbounded_generics(generics);
    let target_generics = common::make_target_generics(generics);
    quote!(
        impl <#(#to_bounded_generics),*> ::bounded_static::ToBoundedStatic for #name <#(#unbounded_generics),*> {
            type Static = #name<#(#target_generics),*>;

            fn to_static(&self) -> Self::Static {
                match self {
                    #(#to_variants),*
                }
            }
        }
        impl <#(#into_bounded_generics),*> ::bounded_static::IntoBoundedStatic for #name <#(#unbounded_generics),*> {
            type Static = #name<#(#target_generics),*>;

            fn into_static(self) -> Self::Static {
                match self {
                    #(#into_variants),*
                }
            }
        }
    )
}

/// Generate a collection of match arms for unit, named and unnamed variants.
///
/// i.e.:
///
/// *Unit*: `Foo::Bar => Foo::bar`
///
/// *Named*: `Foo::Bar { a, b } => Foo::Bar { a: a.to_static(), b: b.to_static() }`
///
/// *Unnamed*: `Foo::Bar(a, b) => Foo::Bar(a.to_static(), b.to_static())`
fn generate_variants(name: &Ident, variants: &[&Variant], target: TargetTrait) -> Vec<TokenStream> {
    variants
        .iter()
        .map(|variant| match &variant.fields {
            Fields::Unit => generate_variant_unit(name, &variant.ident),
            Fields::Named(fields_named) => {
                generate_variant_named(name, &variant.ident, fields_named, target)
            }
            Fields::Unnamed(fields_unnamed) => {
                generate_variant_unnamed(name, &variant.ident, fields_unnamed, target)
            }
        })
        .collect()
}

/// Generate match arm for an unit variant.
///
/// i.e. `Foo::Bar => Foo::bar`
fn generate_variant_unit(name: &Ident, variant: &Ident) -> TokenStream {
    quote!(#name::#variant => #name::#variant)
}

/// Generate match arm for a nnamed variant.
///
/// i.e. `Foo::Bar { a, b } => Foo::Bar { a: a.to_static(), b: b.to_static() }`
fn generate_variant_named(
    name: &Ident,
    variant: &Ident,
    fields_named: &FieldsNamed,
    target: TargetTrait,
) -> TokenStream {
    let fields = extract_named_fields(fields_named);
    let fields_to_method = generate_named_field_init_method(fields_named, target);
    quote!(#name::#variant{ #(#fields),* } => #name::#variant{ #(#fields_to_method),* })
}

/// Generate match arm for an unnamed variant.
///
/// i.e. `Foo::Bar(a, b) => Foo::Bar(a.to_static(), b.to_static())`
fn generate_variant_unnamed(
    name: &Ident,
    variant: &Ident,
    fields_unnamed: &FieldsUnnamed,
    target: TargetTrait,
) -> TokenStream {
    let fields = extract_unnamed_fields(fields_unnamed);
    let fields_to_method = generate_unnamed_field_init_method(fields_unnamed, target);
    quote!(#name::#variant( #(#fields),* ) => #name::#variant( #(#fields_to_method),* ))
}

/// i.e. `foo: foo.to_static()`
fn generate_named_field_init_method(
    fields_named: &FieldsNamed,
    target: TargetTrait,
) -> Vec<TokenStream> {
    let method = target.method();
    fields_named
        .named
        .iter()
        .map(|f| {
            let field_name = f.ident.as_ref().expect("FieldsNamed must have an ident");
            quote!(#field_name: #field_name.#method())
        })
        .collect()
}

/// i.e. `foo.to_static()`
fn generate_unnamed_field_init_method(
    fields_unnamed: &FieldsUnnamed,
    target: TargetTrait,
) -> Vec<TokenStream> {
    let method = target.method();
    fields_unnamed
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let field_name = format_ident!("field_{}", i);
            quote!(#field_name.#method())
        })
        .collect()
}

fn extract_named_fields(fields_named: &FieldsNamed) -> Vec<&Ident> {
    fields_named
        .named
        .iter()
        .map(|f| f.ident.as_ref().expect("FieldsNamed must have an ident"))
        .collect()
}

fn extract_unnamed_fields(fields_unnamed: &FieldsUnnamed) -> Vec<Ident> {
    fields_unnamed
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, _)| format_ident!("field_{}", i))
        .collect()
}
