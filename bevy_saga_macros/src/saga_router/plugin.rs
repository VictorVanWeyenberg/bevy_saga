use proc_macro2::TokenStream;
use quote::quote;
use crate::saga_router::InputEnumMetaData;
use crate::saga_router::util::{pipe_system_name, plugin_add_handler_method_name, plugin_trait_name, to_variant_idents, to_variant_types, to_writer_parameters};

pub fn generate_plugin(input_enum: &InputEnumMetaData) -> TokenStream {
    let plugin_trait = generate_plugin_trait(input_enum);
    let plugin_impl = generate_plugin_impl(input_enum);
    let pipe_system = generate_pipe_system(input_enum);
    quote! {
        #plugin_trait
        #plugin_impl
        #pipe_system
    }
}

fn generate_plugin_trait(input_enum: &InputEnumMetaData) -> TokenStream {
    let plugin_trait_name = plugin_trait_name(input_enum);
    let method_name = plugin_add_handler_method_name(input_enum);
    let enum_ident = &input_enum.enum_ident;
    quote! {
        trait #plugin_trait_name {
            fn #method_name<R, M>(
                &mut self,
                handler: impl bevy::prelude::IntoSystem<R, #enum_ident, M> + 'static,
            ) -> bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::ScheduleSystem>
            where
                R: bevy_saga::SagaEvent;
        }
    }
}

fn generate_plugin_impl(input_enum: &InputEnumMetaData) -> TokenStream {
    let plugin_trait_name = plugin_trait_name(input_enum);
    let method_name = plugin_add_handler_method_name(input_enum);
    let enum_ident = &input_enum.enum_ident;
    let pipe_system_name = pipe_system_name(input_enum);
    quote! {
        impl #plugin_trait_name for bevy::prelude::App {
            fn #method_name<R, M>(
                &mut self,
                handler: impl bevy::prelude::IntoSystem<R, #enum_ident, M> + 'static,
            ) -> bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::ScheduleSystem>
            where
                R: bevy_saga::SagaEvent,
            {
                self.add_event::<R>();
                self.init_resource::<bevy_saga::EventProcessors<R>>();
                let id = self.register_system(handler.pipe(#pipe_system_name));
                self.world_mut()
                    .resource_mut::<bevy_saga::EventProcessors<R>>()
                    .push(id);
                bevy::prelude::IntoScheduleConfigs::into_configs(bevy_saga::process_event::<R>)
            }
        }
    }
}

fn generate_pipe_system(input_enum: &InputEnumMetaData) -> TokenStream {
    let pipe_system_name = pipe_system_name(input_enum);
    let enum_ident = &input_enum.enum_ident;
    let variant_idents = to_variant_idents(input_enum);
    let writer_parameters = to_writer_parameters(input_enum);
    let variant_types = to_variant_types(input_enum);
    quote! {
        fn #pipe_system_name(
            bevy::prelude::In(input_event): bevy::prelude::In<#enum_ident>,
            #(mut #writer_parameters: bevy::prelude::EventWriter<#variant_types>,)*
        ) {
            match input_event {
                #(#enum_ident::#variant_idents(value) => {
                    #writer_parameters.write(value);
                })*
            }
        }
    }
}