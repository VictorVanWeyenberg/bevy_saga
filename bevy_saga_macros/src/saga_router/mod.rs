use crate::saga_router::event_handler::generate_event_handler;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Fields, ItemEnum, Type};

mod event_handler;
mod util;

struct InputEnumMetaData {
    enum_ident: Ident,
    variants: Vec<InputVariantMetaData>,
}

struct InputVariantMetaData {
    ident: Ident,
    ty: Type,
}

pub fn saga_router_from_enum(item_enum: ItemEnum) -> TokenStream {
    let meta_data = match validate_input(&item_enum) {
        Ok(meta_data) => meta_data,
        Err(err) => return err,
    };
    let generated = generate_routing_context(meta_data);
    quote! {
        #[bevy_saga_macros::saga_event]
        #item_enum
        #generated
    }
}

fn generate_routing_context(meta_data: InputEnumMetaData) -> TokenStream {
    let event_handler_context = generate_event_handler(&meta_data);
    quote! {
        #event_handler_context
    }
}

fn validate_input(item_enum: &ItemEnum) -> Result<InputEnumMetaData, TokenStream> {
    let mut variants = vec![];
    for variant in item_enum.variants.iter() {
        let variant_ident = variant.ident.clone();
        match &variant.fields {
            Fields::Named(_) => return Err(compile_error("Variant has named fields. Only variants with one unnamed field are allowed for routing.")),
            Fields::Unnamed(unnamed_fields) => {
                let count = unnamed_fields.unnamed.iter().count();
                if count == 0 {
                    return Err(compile_error("Variant with 0 unnamed fields. Only variants with one unnamed field are allowed for routing."))
                }
                if count > 1 {
                    return Err(compile_error("Variant with more than 1 unnamed fields. Only variants with one unnamed field are allowed for routing."))
                }
                variants.push(InputVariantMetaData{ ident: variant_ident, ty: unnamed_fields.unnamed.first().unwrap().ty.clone() });
            }
            Fields::Unit => return Err(compile_error("Unit variant. Only variants with one unnamed field are allowed for routing.")),
        }
    }
    Ok(InputEnumMetaData {
        enum_ident: item_enum.ident.clone(),
        variants,
    })
}

fn compile_error(message: &str) -> TokenStream {
    quote! {
        compile_error!(
            #message
        )
    }
}
