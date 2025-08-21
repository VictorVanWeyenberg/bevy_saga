use crate::saga_router::util::{builder_struct_name, to_field_types, to_parameter_names};
use crate::saga_router::{InputEnumMetaData, InputVariantMetaData};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_builders(input_enum: &InputEnumMetaData) -> Vec<TokenStream> {
    // [1, 2, 3] => [1], [1, 2], [1, 2, 3]
    input_enum.variants
        .split_last()
        .map(|variants| variants.1
            .iter()
            .scan(vec![], |state, variant| {
                state.push(variant.clone());
                Some(state.clone())
            })
            .map(builder_from_variants)
            .collect())
        .unwrap_or(vec![])
}

fn builder_from_variants(variants: Vec<InputVariantMetaData>) -> TokenStream {
    let builder_struct_name = builder_struct_name(variants.last().unwrap());
    let field_names = to_parameter_names(&variants);
    let field_types = to_field_types(&variants[..]);
    quote! {
        struct #builder_struct_name<Source, #(#field_types, )*> {
            source: Source,
            #(#field_names: #field_types, )*
        }
    }
}