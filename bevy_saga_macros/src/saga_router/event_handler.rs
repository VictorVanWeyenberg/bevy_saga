use crate::saga_router::InputEnumMetaData;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use crate::saga_router::util::*;

fn router_struct(input_enum: &InputEnumMetaData) -> TokenStream {
    let struct_ident = router_struct_name(input_enum);
    let generic_types: Vec<Ident> = generic_types(input_enum);
    let fields: Vec<TokenStream> = field_notations(input_enum);
    quote! {
        struct #struct_ident<Source, #(#generic_types, )*> {
            source: Source,
            #(#fields, )*
        }
    }
}

fn router_impl(input_enum: &InputEnumMetaData) -> TokenStream {
    let generics = generic_types(input_enum);
    let ident = router_struct_name(input_enum);
    let field_notations = field_notations(input_enum);
    let field_names = field_names(input_enum);
    quote! {
        impl<Source, #(#generics, )*> #ident<Source, #(#generics, )*>
        {
            pub fn new(source: Source, #(#field_notations, )*) -> Self {
                Self {
                    source,
                    #(#field_names, )*
                }
            }
        }
    }
}

fn event_handler_impl(input_enum: &InputEnumMetaData) -> TokenStream {
    let struct_ident = router_struct_name(input_enum);
    let generics = generic_types(input_enum);
    let processor_trait = processor_trait_name(input_enum);
    let processor_trait_method = processor_trait_method_name(input_enum);
    let marker_generics = marker_generics(input_enum);
    let field_names = field_names(input_enum);
    let constraints = generics_constraints(input_enum);
    let saga_register_calls = saga_register_calls(input_enum);
    quote! {
        impl<
            Source,
            #(#generics, )*
            MarkerSource,
            #(#marker_generics, )*
        > bevy_saga_impl::prelude::EventHandler<(MarkerSource, #(#marker_generics, )*)>
        for #struct_ident<Source, #(#generics, )*>
        where
            Source: #processor_trait<MarkerSource>,
            Source::In: bevy_saga_impl::SagaEvent,
            #(#constraints, )*
            MarkerSource: 'static,
        {
            type In = Source::In;

            fn register_handler(
                self,
                app: &mut bevy::prelude::App,
            ) -> bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::ScheduleSystem> {
                let Self {
                    source,
                    #(#field_names, )*
                } = self;
                bevy::prelude::IntoScheduleConfigs::chain((
                    source.#processor_trait_method(app),
                    (#(#saga_register_calls, )*),
                ))
            }
        }
    }
}

pub fn generate_event_handler(input_enum_meta_data: &InputEnumMetaData) -> TokenStream {
    let router_struct = router_struct(input_enum_meta_data);
    let router_impl = router_impl(input_enum_meta_data);
    let event_handler_impl = event_handler_impl(input_enum_meta_data);
    quote! {
        #router_struct
        #router_impl
        #event_handler_impl
    }
}
