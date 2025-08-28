use quote::{quote, ToTokens};
use syn::{ItemEnum, ItemStruct};

pub fn saga_event_from_struct(struct_item: ItemStruct) -> proc_macro2::TokenStream {
    let ident = struct_item.ident.clone();
    saga_event_from_tokens(struct_item.into_token_stream(), ident)
}

pub fn saga_event_from_enum(enum_item: ItemEnum) -> proc_macro2::TokenStream {
    let ident = enum_item.ident.clone();
    saga_event_from_tokens(enum_item.into_token_stream(), ident)
}

fn saga_event_from_tokens(tokens: proc_macro2::TokenStream, ident: proc_macro2::Ident) -> proc_macro2::TokenStream {
    quote! {
        #[derive(Clone, bevy::prelude::Event)]
        #tokens

        impl bevy_saga_impl::SagaEvent for #ident {
        }

        impl bevy::prelude::SystemInput for #ident {
            type Param<'i> = #ident;
            type Inner<'i> = #ident;

            fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> {
                this
            }
        }
    }
}