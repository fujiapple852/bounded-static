use crate::common::TargetTrait;
use crate::{common, generics};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Field, FieldsNamed, Generics};

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
        generics::make_bounded_generics(generics, TargetTrait::ToBoundedStatic);
    let into_static_bounded_generics =
        generics::make_bounded_generics(generics, TargetTrait::IntoBoundedStatic);
    let unbounded_generics = generics::make_unbounded_generics(generics);
    let target_generics = generics::make_target_generics(generics);
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
