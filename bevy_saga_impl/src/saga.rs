use crate::SagaEvent;
use bevy::prelude::App;

/// The definition of a saga.
///
/// A Saga is a chain of optionally multiple _event processors_ and one _event handler_.
///
/// An event processor has a [SagaEvent](SagaEvent) as input parameter and a SagaEvent as output
/// parameter. An event handler only has a SagaEvent as input parameter but doesn't return
/// anything.
///
/// When you chain the event processors, you have to order then that the output parameter of one
/// processor is the same type as the input parameter of the following processor (or handler).
///
/// ```
/// # use bevy::prelude::{App, Update};
/// use bevy_saga_impl::SagaRegistry;
/// # use bevy_saga_macros::saga_event;
/// # let mut app = App::new();
/// #[saga_event]
/// struct A;
///
/// // Other events attributed with `saga_event`.
/// # #[saga_event]
/// # struct B;
/// # #[saga_event]
/// # struct C;
/// # #[saga_event]
/// # struct D;
///
/// // Functions are the simplest form of processors and handlers.
/// fn processor1(_: A, /* other queries or resources */) -> B { B }
/// fn processor2(_: B, /* other queries or resources */) -> C { C }
/// fn processor3(_: C, /* other queries or resources */) -> D { D }
/// fn handler   (_: D, /* other queries or resources */)      {   }
///
/// app.add_saga(Update, (processor1, processor2, processor3, handler));
/// app.world_mut().send_event(A);
/// ```
///
/// In this case the output parameter of `processor1` is the same type as the input parameter of
/// `processor2`.
/// The output parameter of `processor2` is the same type as the input parameter of `processor3`.
/// And finally, the output parameter of `processor3` is the same type as the input parameter of
/// the `handler`.
///
/// If there is any mismatch in parameter types, bevy_saga will not compile.
///
/// In order to trigger the saga, send an event. All processors and the handler will be executed in
/// one update cycle. If you send an event that's further down the chain, that event will still be 
/// propagated through the saga. In that case the earlier processors are not executed.
///
/// - Learn how to write event processors [here](crate::processor::EventProcessor).
/// - Learn how to write an event handler [here](crate::handler::EventHandler).
///
/// You can add up to 15 processors in a saga tuple. **Every chain must end with one handler.**
/// The minimal saga you can write is one single event handler; in that case it doesn't need to be
/// a tuple.
///
/// ```
/// # use bevy::prelude::{App, Update};
/// use bevy_saga_impl::SagaRegistry;
/// # use bevy_saga_macros::saga_event;
/// # let mut app = App::new();
/// # #[saga_event]
/// # struct A;
/// fn handler(_: A) { }
///
/// app.add_saga(Update, handler);
/// app.world_mut().send_event(A);
/// ```
pub trait Saga<M> {
    type In: SagaEvent;

    fn register(self, app: &mut App) -> bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::ScheduleSystem>;
}

impl<S, M, In> Saga<(M,)> for S
where
    In: SagaEvent,
    S: crate::handler::EventHandler<M, In = In>,
{
    type In = In;

    fn register(self, app: &mut App) -> bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::ScheduleSystem> {
        crate::handler::EventHandler::register_handler(self, app)
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
                    bevy::prelude::IntoScheduleConfigs::chain((
                        #(#unpack_variables.register_processor(app),)*
                        crate::handler::EventHandler::register_handler(h, app),
                    ))
                }
            }
        }
    }).collect::<Vec<_>>();

    quote! {
        #(#tokens)*
    }
}


impl_saga!();