use crate::saga_router::{InputEnumMetaData, InputVariantMetaData};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use stringcase::snake_case;
use syn::Type;

pub fn generic_types(input_enum: &InputEnumMetaData) -> Vec<Ident> {
    input_enum.variants.iter().map(to_generic_type).collect()
}

pub fn field_notations(input_enum: &InputEnumMetaData) -> Vec<TokenStream> {
    input_enum.variants.iter().map(to_field_notation).collect()
}

pub fn field_names(input_enum: &InputEnumMetaData) -> Vec<Ident> {
    input_enum.variants.iter().map(to_field_name).collect()
}

pub fn marker_generics(input_enum: &InputEnumMetaData) -> Vec<Ident> {
    input_enum
        .variants
        .iter()
        .map(to_marker_generic_type)
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
    format_ident!("add_{}_handler", enum_ident_snake_case(input_enum))
}

fn enum_ident_snake_case(input_enum: &InputEnumMetaData) -> String {
    snake_case(input_enum.enum_ident.to_string().as_str())
}

pub fn plugin_trait_name(input_enum: &InputEnumMetaData) -> Ident {
    format_ident!("{}Plugin", input_enum.enum_ident)
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

pub fn to_variant_idents(input_enum: &InputEnumMetaData) -> Vec<Ident> {
    input_enum
        .variants
        .iter()
        .map(|variant| variant.ident.clone())
        .collect()
}

pub fn to_variant_types(input_enum: &InputEnumMetaData) -> Vec<&Type> {
    input_enum
        .variants
        .iter()
        .map(|variant| &variant.ty)
        .collect()
}

pub fn to_writer_parameters(input_enum: &InputEnumMetaData) -> Vec<Ident> {
    input_enum
        .variants
        .iter()
        .map(|variant| format_ident!("{}_writer", variant.ident.to_string().to_lowercase()))
        .collect()
}

pub fn to_field_name(variant: &InputVariantMetaData) -> Ident {
    handler_field_name(variant)
}

pub fn to_marker_generic_type(variant: &InputVariantMetaData) -> Ident {
    format_ident!("Marker{}", handler_field_type(variant))
}

pub fn to_generic_constraint(variant: &InputVariantMetaData) -> TokenStream {
    let generic_name = handler_field_type(variant);
    let marker_generic_type = to_marker_generic_type(variant);
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

pub fn pipe_system_name(input_enum: &InputEnumMetaData) -> Ident {
    format_ident!("send_{}_response", enum_ident_snake_case(input_enum))
}

pub fn to_field_types(variants: &[InputVariantMetaData]) -> Vec<Ident> {
    variants.iter().map(handler_field_type).collect()
}

pub fn handler_field_type(variant: &InputVariantMetaData) -> Ident {
    format_ident!("{}Saga", variant.ident)
}

pub fn trait_name(variant: &InputVariantMetaData) -> Ident {
    format_ident!("{}Stage", variant.ident)
}

pub fn builder_struct_name(variant: &InputVariantMetaData) -> Ident {
    format_ident!("{}Builder", trait_name(variant))
}

pub fn trait_method_name(variant: &InputVariantMetaData) -> Ident {
    format_ident!("{}", variant.ident.clone().to_string().to_lowercase())
}

pub fn to_parameter_names(variants: &[InputVariantMetaData]) -> Vec<Ident> {
    variants.into_iter().map(trait_parameter_name).collect()
}

pub fn trait_parameter_name(variant: &InputVariantMetaData) -> Ident {
    format_ident!("{}_saga", variant.ident.clone().to_string().to_lowercase())
}
