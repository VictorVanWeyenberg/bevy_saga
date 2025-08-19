use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use stringcase::snake_case;
use crate::saga_router::{InputEnumMetaData, InputVariantMetaData};

pub fn generic_types(input_enum: &InputEnumMetaData) -> Vec<Ident> {
    input_enum
        .variants
        .iter()
        .map(to_generic_type)
        .collect()
}

pub fn field_notations(input_enum: &InputEnumMetaData) -> Vec<TokenStream> {
    input_enum
        .variants
        .iter()
        .map(to_field_notation)
        .collect()
}

pub fn field_names(input_enum: &InputEnumMetaData) -> Vec<Ident> {
    input_enum
        .variants
        .iter()
        .map(to_field_name)
        .collect()
}

pub fn marker_generics(input_enum: &InputEnumMetaData) -> Vec<Ident> {
    input_enum
        .variants
        .iter()
        .map(to_marker_generic_types)
        .collect()
}

pub fn generics_constraints(input_enum: &InputEnumMetaData) -> Vec<TokenStream> {
    input_enum
        .variants
        .iter()
        .map(to_generic_constraint)
        .collect()
}

pub fn saga_register_calls(input_enum: &InputEnumMetaData) -> Vec<TokenStream> {
    input_enum
        .variants
        .iter()
        .map(to_saga_register_call)
        .collect()
}

pub fn plugin_add_handler_method_name(input_enum: &InputEnumMetaData) -> Ident {
    format_ident!(
        "add_{}_handler",
        snake_case(input_enum.enum_ident.to_string().as_str())
    )
}

pub fn router_struct_name(input_enum: &InputEnumMetaData) -> Ident {
    format_ident!("{}Router", input_enum.enum_ident)
}

pub fn to_generic_type(variant: &InputVariantMetaData) -> Ident {
    handler_field_type(variant)
}

pub fn to_field_notation(variant: &InputVariantMetaData) -> TokenStream {
    let ident = to_field_name(variant);
    let field_type = handler_field_type(variant);
    quote! {
        #ident: #field_type
    }
}

pub fn to_field_name(variant: &InputVariantMetaData) -> Ident {
    handler_field_name(variant)
}

pub fn to_marker_generic_types(variant: &InputVariantMetaData) -> Ident {
    format_ident!("Marker{}", handler_field_type(variant))
}

pub fn to_generic_constraint(variant: &InputVariantMetaData) -> TokenStream {
    let generic_name = handler_field_type(variant);
    let marker_generic_type = to_marker_generic_types(variant);
    let ty = &variant.ty;
    quote! {
        #generic_name: bevy_saga::Saga<#marker_generic_type, In = #ty>
    }
}

pub fn to_saga_register_call(variant: &InputVariantMetaData) -> TokenStream {
    let field_name = handler_field_name(variant);
    quote! {
        #field_name.register(app)
    }
}

pub fn handler_field_name(variant: &InputVariantMetaData) -> Ident {
    format_ident!("{}", variant.ident.clone().to_string().to_lowercase())
}

pub fn handler_field_type(variant: &InputVariantMetaData) -> Ident {
    format_ident!("{}Saga", variant.ident)
}