use crate::saga_router::util::{plugin_add_handler_method_name, processor_trait_method_name, processor_trait_name};
use crate::saga_router::InputEnumMetaData;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub fn generate_processor_trait(input_enum: &InputEnumMetaData) -> TokenStream {
    let trait_definition = processor_trait_definition(input_enum);
    let mut implementations = vec![];

    let mut handler_generics = vec![];
    let mut handler_marker_generics = vec![];
    let mut unpack_variables = vec![];
    for n in 0u8..16 {
        implementations.push(processor_trait_implementation(input_enum, &handler_generics, &handler_marker_generics, &unpack_variables));
        handler_generics.push(format_ident!("ER{}", n));
        handler_marker_generics.push(format_ident!("MER{}", n));
        unpack_variables.push(format_ident!("er{}", n));
    }

    quote! {
        #trait_definition
        #(#implementations)*
    }
}

fn processor_trait_definition(input_enum: &InputEnumMetaData) -> TokenStream {
    let trait_name = processor_trait_name(input_enum);
    let method_name = processor_trait_method_name(input_enum);
    quote! {
        pub trait #trait_name<M> {
            type In: bevy_saga::SagaEvent;

            fn #method_name(self, app: &mut bevy::prelude::App) -> bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::ScheduleSystem>;
        }
    }
}

fn processor_trait_implementation(input_enum: &InputEnumMetaData, handler_generics: &Vec<Ident>, handler_marker_generics: &Vec<Ident>, unpack_variables: &Vec<Ident>) -> TokenStream {
    let enum_ident = &input_enum.enum_ident;
    let trait_name = processor_trait_name(input_enum);
    let method_name = processor_trait_method_name(input_enum);
    let plugin_method_name = plugin_add_handler_method_name(input_enum);
    let implementor = derive_implementor(handler_generics);
    let implementation = derive_implementation(unpack_variables, plugin_method_name);
    quote! {
        impl<RS, MRS, #(#handler_generics,)* #(#handler_marker_generics,)* In> #trait_name<(MRS, #(#handler_marker_generics,)*)> for #implementor
        where
            RS: bevy::prelude::SystemParamFunction<MRS, In = In, Out = #enum_ident>,
            #(#handler_generics: bevy::prelude::SystemParamFunction<#handler_marker_generics, In = In, Out = ()>,)*
            In: bevy_saga::SagaEvent,
            MRS: 'static,
            #(#handler_marker_generics: 'static,)*
        {
            type In = In;

            fn #method_name(self, app: &mut bevy::prelude::App) -> bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::ScheduleSystem> {
                #implementation
            }
        }
    }
}

fn derive_implementor(handler_generics: &Vec<Ident>) -> TokenStream {
    if handler_generics.is_empty() {
        quote! { RS }
    } else {
        quote! { (RS, #(#handler_generics,)*) }
    }
}

fn derive_implementation(unpack_variables: &Vec<Ident>, plugin_method_name: Ident) -> TokenStream {
    if unpack_variables.is_empty() {
        quote! { app.#plugin_method_name(self) }
    } else {
        quote! {
            let (rs, #(#unpack_variables,)*) = self;
            bevy::prelude::IntoScheduleConfigs::into_configs((
                app.#plugin_method_name(rs),
                #(bevy_saga::BevySagaUtil::add_event_handler(app, #unpack_variables),)*
            ))
        }
    }
}