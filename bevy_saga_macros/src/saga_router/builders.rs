use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use crate::saga_router::{InputEnumMetaData, InputVariantMetaData};
use crate::saga_router::util::{to_field_types, to_parameter_names};

pub fn generate_builders(input_enum: &InputEnumMetaData) -> Vec<TokenStream> {
    // [1, 2, 3] => [1], [1, 2], [1, 2, 3]
    input_enum.variants
        .split_last()
        .map(|variants| variants.1
            .iter()
            .scan(vec![], |state, variant| {
                state.push(variant);
                Some(state.clone())
            })
            .map(builder_from_variants)
            .collect())
        .unwrap_or(vec![])
}

fn builder_from_variants(variants: Vec<&InputVariantMetaData>) -> TokenStream {
    let builder_struct_name = format_ident!("{}StageBuilder", variants.last().unwrap().ident);
    let field_names = to_parameter_names(&variants);
    let field_types = to_field_types(&variants);
    quote! {
        struct #builder_struct_name<Source, #(#field_types, )*> {
            source: Source,
            #(#field_names: #field_types, )*
        }
    }
}