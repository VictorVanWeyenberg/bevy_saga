use crate::SagaEvent;
use crate::handler::EventHandler;
use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::App;

pub trait Saga<M> {
    type In: SagaEvent;

    fn register(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem>;
}

impl<S, M, In> Saga<(M,)> for S
where
    In: SagaEvent,
    S: EventHandler<M, In = In>,
{
    type In = In;

    fn register(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem> {
        self.register_handler(app)
    }
}

#[crabtime::function]
fn impl_saga() -> proc_macro2::TokenStream {
    #![dependency(proc-macro2 = "1")]
    #![dependency(quote = "1")]

    use quote::{format_ident, quote};

    let tokens = (1u32..16).map(|number_of_processors| {
        let mut processor_generics = vec![];
        let mut marker_generics = vec![];
        let mut input_generics = vec![quote!(In)];
        let mut unpack_variables = vec![];
        for index in 1..(number_of_processors + 1) {
            processor_generics.push(format_ident!("S{}", index));
            marker_generics.push(format_ident!("M{}", index));
            let previous_processor = format_ident!("S{}", index);
            input_generics.push(quote!(#previous_processor::Out));
            unpack_variables.push(format_ident!("s{}", index));
        }
        let handler_input = input_generics.remove(input_generics.len() - 1);

        quote! {
            impl<#(#processor_generics,)* H, #(#marker_generics,)* MH, In> crate::saga::Saga<(#(#marker_generics,)* MH)> for (#(#processor_generics,)* H)
            where
                #(#processor_generics: crate::processor::EventProcessor<#marker_generics, In = #input_generics>,)*
                H: crate::handler::EventHandler<MH, In = #handler_input>,
                In: crate::SagaEvent,
                #(#processor_generics::In: crate::SagaEvent,)*
            {
                type In = In;

                fn register(self, app: &mut bevy::prelude::App) -> bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::ScheduleSystem> {
                    let (#(#unpack_variables,)* h) = self;
                    (
                        #(#unpack_variables.register_processor(app),)*
                        h.register_handler(app),
                    )
                        .chain()
                }
            }
        }
    }).collect::<Vec<_>>();

    quote! {
        use bevy::prelude::IntoScheduleConfigs;
        #(#tokens)*
    }
}


impl_saga!();