use crate::{data_enum, data_struct_named, data_struct_unit, data_struct_unnamed};
use proc_macro2::TokenStream;
use syn::{Data, DataStruct, DeriveInput, Fields};

/// Generate `ToBoundedStatic` and `IntoBoundedStatic` impls for the data item deriving `ToStatic`.
pub(super) fn generate(input: &DeriveInput) -> TokenStream {
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields_named),
            ..
        }) => data_struct_named::generate_struct_named(&input.ident, &input.generics, fields_named),
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(fields_unnamed),
            ..
        }) => data_struct_unnamed::generate_struct_unnamed(
            &input.ident,
            &input.generics,
            fields_unnamed,
        ),
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => data_struct_unit::generate_struct_unit(&input.ident),
        Data::Enum(data_enum) => data_enum::generate_enum(
            &input.ident,
            &input.generics,
            data_enum.variants.iter().collect::<Vec<_>>().as_slice(),
        ),
        Data::Union(_) => unimplemented!("union is not yet supported"),
    }
}
