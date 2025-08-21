use crate::saga_router::util::*;
use crate::saga_router::{InputEnumMetaData, InputVariantMetaData};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn generate_builder_impls(input_enum: &InputEnumMetaData) -> Vec<TokenStream> {
    input_enum
        .variants
        .iter()
        .enumerate()
        .map(|(index, current)| generate_builder_impl_with_index(index, current, input_enum))
        .collect()
}

fn generate_builder_impl_with_index(
    index: usize,
    current: &InputVariantMetaData,
    input_enum: &InputEnumMetaData,
) -> TokenStream {
    let (previous, following) = input_enum.variants.split_at(index);
    let last = match previous.len() {
        0 => None,
        n => Some(&previous[n - 1]),
    };
    let next = following.get(1);
    generate_builder_impl(input_enum, previous, last, current, next)
}

fn generate_builder_impl(
    input_enum: &InputEnumMetaData,
    previous: &[InputVariantMetaData],
    last: Option<&InputVariantMetaData>,
    current: &InputVariantMetaData,
    next: Option<&InputVariantMetaData>,
) -> TokenStream {
    let enum_ident = &input_enum.enum_ident;
    let previous_field_types = to_field_types(previous);
    let trait_name = trait_name(current);
    let implementor = derive_implementor(last, &previous_field_types);
    let method_definition = generate_method_definition(current);
    let constraint = to_generic_constraint(current);
    let return_type = derive_return_type(input_enum, previous, current, next);
    let unpack = generate_unpack(previous, last);
    let return_value = derive_return_value(input_enum, previous, current, next);
    quote! {
        impl<Source, MarkerSource, #(#previous_field_types,)*> #trait_name<Source, MarkerSource, #(#previous_field_types,)*>
            for #implementor
        where
            Source: bevy::prelude::SystemParamFunction<MarkerSource, Out = #enum_ident>,
            Source::In: bevy_saga::SagaEvent,
            MarkerSource: 'static,
        {
            #method_definition -> #return_type
            where
                #constraint,
            {
                #unpack
                #return_value
            }
        }
    }
}

fn derive_implementor(last: Option<&InputVariantMetaData>, previous_field_types: &Vec<Ident>) -> TokenStream {
    match last {
        None => quote! { Source },
        Some(last) => {
            let builder_struct_name = builder_struct_name(last);
            quote! { #builder_struct_name<Source, #(#previous_field_types,)*> }
        }
    }
}

fn generate_method_definition(variant: &InputVariantMetaData) -> TokenStream {
    let method_name = trait_method_name(variant);
    let field_type = handler_field_type(variant);
    let marker = to_marker_generic_type(variant);
    let parameter_name = trait_parameter_name(variant);
    quote! {
        fn #method_name<#field_type, #marker>(
            self,
            #parameter_name: #field_type,
        )
    }
}

fn derive_return_type(input_enum: &InputEnumMetaData, previous: &[InputVariantMetaData], current: &InputVariantMetaData, next: Option<&InputVariantMetaData>) -> TokenStream {
    let previous_types = to_field_types(previous);
    let current_type = handler_field_type(current);
    match next {
        None => {
            let struct_name = router_struct_name(input_enum);
            quote! {
                #struct_name<Source, #(#previous_types,)* #current_type>
            }
        }
        Some(next) => {
            let trait_name = trait_name(next);
            quote! {
                impl #trait_name<Source, MarkerSource, #(#previous_types,)* #current_type>
            }
        }
    }
}

fn generate_unpack(previous: &[InputVariantMetaData], last: Option<&InputVariantMetaData>) -> TokenStream {
    match last {
        None => {
            quote! {
                let source = self;
            }
        }
        Some(last) => {
            let builder_struct_name = builder_struct_name(last);
            let fields = to_parameter_names(previous);
            quote! {
                let #builder_struct_name {
                    source,
                    #(#fields,)*
                } = self;
            }
        }
    }
}

fn derive_return_value(input_enum: &InputEnumMetaData, previous: &[InputVariantMetaData], current: &InputVariantMetaData, next: Option<&InputVariantMetaData>) -> TokenStream {
    let previous_fields = to_parameter_names(previous);
    let current_field = trait_parameter_name(current);
    if next.is_some() {
        let builder_name = builder_struct_name(current);
        quote! {
            #builder_name {
                source,
                #(#previous_fields,)*
                #current_field
            }
        }
    } else {
        let router_ident = router_struct_name(input_enum);
        quote! {
            #router_ident::new(source, #(#previous_fields,)* #current_field)
        }
    }
}
