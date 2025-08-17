use crate::saga_event::{saga_event_from_enum, saga_event_from_struct};
use crate::saga_router::saga_router_from_enum;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

mod saga_event;
mod saga_router;

#[proc_macro_attribute]
pub fn saga_event(_attr: TokenStream, item: TokenStream) -> TokenStream {
    match parse_macro_input!(item as Item) {
        Item::Enum(enum_item) => saga_event_from_enum(enum_item),
        Item::Struct(struct_item) => saga_event_from_struct(struct_item),
        _ => quote!{
            compile_error!("Attribute saga_event is only meant for struct or enum items.");
        },
    }.into()
}

#[proc_macro_attribute]
pub fn saga_router(_attr: TokenStream, item: TokenStream) -> TokenStream {
    match parse_macro_input!(item as Item) {
        Item::Enum(item_enum) => saga_router_from_enum(item_enum),
        _ => quote! {
            compile_error!("Attribute saga_router is only meant for enums.")
        }
    }.into()
}