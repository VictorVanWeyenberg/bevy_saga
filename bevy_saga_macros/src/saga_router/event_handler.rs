use crate::saga_router::InputEnumMetaData;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use stringcase::snake_case;
use syn::Type;

pub struct EventHandlerContext {
    enum_ident: Ident,
    ident: Ident,
    fields: Vec<EventHandlerField>,
}

impl ToTokens for EventHandlerContext {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let router_struct = self.router_struct();
        let router_impl = self.router_impl();
        let event_handler_impl = self.event_handler_impl();
        quote! {
            #router_struct
            #router_impl
            #event_handler_impl
        }.to_tokens(tokens)
    }
}

impl EventHandlerContext {
    fn generic_types(&self) -> Vec<Ident> {
        self.fields.iter()
            .map(|field| field.to_generic_type())
            .collect()
    }

    fn field_notations(&self) -> Vec<TokenStream> {
        self.fields.iter()
            .map(|field| field.to_field_notation())
            .collect()
    }

    fn field_names(&self) -> Vec<Ident> {
        self.fields.iter()
            .map(|field| field.to_field_name())
            .collect()
    }

    fn marker_generics(&self) -> Vec<Ident> {
        self.fields.iter()
            .map(|field| field.to_marker_generic_types())
            .collect()
    }

    fn generics_constraints(&self) -> Vec<TokenStream> {
        self.fields.iter()
            .filter_map(|field| field.to_generic_constraint())
            .collect()
    }

    fn saga_register_calls(&self) -> Vec<TokenStream> {
        self.fields.iter()
            .filter_map(|field| field.to_saga_register_call())
            .collect()
    }

    fn plugin_add_handler_method_name(&self) -> Ident {
        format_ident!("add_{}_handler", snake_case(self.enum_ident.to_string().as_str()))
    }

    fn router_struct(&self) -> TokenStream {
        let struct_ident = self.ident.clone();
        let generic_types: Vec<Ident> = self.generic_types();
        let fields: Vec<TokenStream> = self.field_notations();
        quote! {
            struct #struct_ident<#(#generic_types, )*> {
                #(#fields, )*
            }
        }
    }

    fn router_impl(&self) -> TokenStream {
        let generics = self.generic_types();
        let ident = self.ident.clone();
        let field_notations = self.field_notations();
        let field_names = self.field_names();
        quote! {
            impl<#(#generics, )*> #ident<#(#generics, )*>
            {
                pub fn new(#(#field_notations, )*) -> Self {
                    Self {
                        #(#field_names, )*
                    }
                }
            }
        }
    }

    fn event_handler_impl(&self) -> TokenStream {
        let struct_ident = &self.ident;
        let generics = self.generic_types();
        let marker_generics = self.marker_generics();
        let field_names = self.field_names();
        let enum_ident = &self.enum_ident;
        let constraints = self.generics_constraints();
        let saga_register_calls = self.saga_register_calls();
        let method_name = self.plugin_add_handler_method_name();
        quote! {
            impl<
                #(#generics, )*
                #(#marker_generics, )*
            > bevy_saga::EventHandler<(#(#marker_generics, )*)>
            for #struct_ident<#(#generics, )*>
            where
                Source: bevy::prelude::SystemParamFunction<MarkerSource, Out = #enum_ident>,
                Source::In: bevy_saga::SagaEvent,
                #(#constraints, )*
                MarkerSource: 'static,
            {
                type In = Source::In;

                fn register_handler(
                    self,
                    app: &mut bevy::prelude::App,
                ) -> bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::ScheduleSystem> {
                    let Self {
                        #(#field_names, )*
                    } = self;
                    bevy::prelude::IntoScheduleConfigs::chain((
                        app.#method_name(source),
                        (#(#saga_register_calls, )*),
                    ))
                }
            }
        }
    }
}

pub struct EventHandlerField {
    variant_name: Option<Ident>,
    variant_type: Option<Type>,
    handler_field_name: Ident,
    handler_field_type: Ident,
}

impl EventHandlerField {
    fn to_generic_type(&self) -> Ident {
        self.handler_field_type.clone()
    }

    fn to_field_notation(&self) -> TokenStream {
        let ident = self.to_field_name();
        let field_type = self.handler_field_type.clone();
        quote! {
            #ident: #field_type
        }
    }

    fn to_field_name(&self) -> Ident {
        self.handler_field_name.clone()
    }

    fn to_marker_generic_types(&self) -> Ident {
        format_ident!("Marker{}", &self.handler_field_type)
    }

    fn to_generic_constraint(&self) -> Option<TokenStream> {
        if let Some(ty) = &self.variant_type {
            let generic_name = &self.handler_field_type;
            let marker_generic_type = self.to_marker_generic_types();
            Some(quote! {
                #generic_name: bevy_saga::Saga<#marker_generic_type, In = #ty>
            })
        } else {
            None
        }
    }

    fn to_saga_register_call(&self) -> Option<TokenStream> {
        if self.variant_type.is_some() {
            let field_name = &self.handler_field_name;
            Some(quote! {
                #field_name.register(app)
            })
        } else {
            None
        }
    }
}

pub fn generate_event_handler(input_enum_meta_data: &InputEnumMetaData) -> EventHandlerContext {
    let enum_ident = &input_enum_meta_data.enum_ident;
    let event_handler_ident = format_ident!("{}Router", enum_ident);
    let mut fields = vec![source_field()];
    input_enum_meta_data.variants_and_types.iter()
        .map(variants_to_event_handler_variant_context)
        .for_each(|field| fields.push(field));
    EventHandlerContext {
        enum_ident: enum_ident.clone(),
        ident: event_handler_ident,
        fields,
    }
}

pub fn source_field() -> EventHandlerField {
    EventHandlerField {
        variant_name: None,
        variant_type: None,
        handler_field_name: format_ident!("source"),
        handler_field_type: format_ident!("Source"),
    }
}

fn variants_to_event_handler_variant_context((ident, ty): &(Ident, Type)) -> EventHandlerField {
    let variant_name = Some(ident.clone());
    let variant_type = Some(ty.clone());
    let handler_field_name = format_ident!("{}", ident.clone().to_string().to_lowercase());
    let handler_field_type = format_ident!("{}Saga", ident);
    EventHandlerField {
        variant_name,
        variant_type,
        handler_field_name,
        handler_field_type,
    }
}
