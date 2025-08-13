use crate::saga_event::{saga_event_from_enum, saga_event_from_struct};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

mod saga_event;

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