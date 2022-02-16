use proc_macro2::{Ident, TokenStream};
use quote::quote;

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
