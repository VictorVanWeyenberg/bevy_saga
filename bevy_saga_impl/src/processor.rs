use crate::{SagaEvent, extension::BevySagaUtil};
use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{App, Event, IntoScheduleConfigs, SystemParamFunction};
use variadics_please::all_tuples;

/// The definition of an event processor.
///
/// An event processor is a collection of functions that all have the same type as input
/// parameter. Only the first system in the collection has a return type. The following systems in
/// the collection may not return anything. The return value of that first system will be passed on
/// to the following processor or handler in the saga.
///
/// Both the input and output types have to be attributed with `saga_event`.
/// 
/// All systems in the collection are executed concurrently.
/// 
/// # Option Processor
/// 
/// There is one special type of event processor: the Option processor.
/// 
/// This processor accepts a saga event and returns an [Option](Option) of a saga event.
/// If the option is Some, the containing value will be passed on to the following processors or 
/// handler in the saga.
/// If the option is empty, the following processors or handler in the saga won't be executed.
/// 
/// # Example
///
/// ```
/// # use bevy::app::{App, Update};
/// # use bevy::prelude::{Component, Query};
/// # use bevy_saga_impl::SagaRegistry;
/// # use bevy_saga_macros::saga_event;
/// #[saga_event]
/// struct A;
///
/// #[saga_event]
/// struct B;
///
/// fn process_event(_: A, /* other queries or resources */) -> B { B }
/// fn sibling1(_: A, /* other queries or resources */) { }
/// fn sibling2(_: A, /* other queries or resources */) { }
/// fn sibling3(_: A, /* other queries or resources */) { }
///
/// fn handler(_: B, /* other queries or resources */) { }
///
/// # let mut app = App::new();
///
/// // One event processor function is a valid event processor.
/// let processor = process_event;
/// app.add_saga(Update, (processor, handler));
///
/// // A processor can also be accompanied by other functions the handle the event.
/// let processor = (process_event, sibling1, sibling2, sibling3);
/// app.add_saga(Update, (processor, handler));
/// 
/// // Let's try an option processor:
/// fn maybe_process_event(_: A, /* other queries or resources */) -> Option<B> {
///     None
/// }
/// 
/// let processor = (maybe_process_event, sibling1, sibling2);
/// app.add_saga(Update, (processor, handler));
/// ```
pub trait EventProcessor<M> {
    type In: SagaEvent;
    type Out: Event;

    fn register_processor(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem>;
}

impl<SPF, M, In, Out> EventProcessor<(M,)> for SPF
where
    In: SagaEvent,
    Out: Event,
    SPF: SystemParamFunction<M, In = In, Out = Out>,
    M: 'static,
{
    type In = In;
    type Out = Out;

    fn register_processor(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem> {
        app.add_event_processor::<In, Out, _>(self)
    }
}

macro_rules! impl_event_processor {
    ($(#[$meta:meta])* $(($SPF:ident, $h:ident, $M:ident)),*) => {
        impl<PROC, MPROC, $($SPF,)* $($M,)* In, Out> EventProcessor<(MPROC, $($M,)*)> for (PROC, $($SPF,)*)
        where
            In: SagaEvent,
            Out: Event,
            PROC: SystemParamFunction<MPROC, In = In, Out = Out>,
            $($SPF: SystemParamFunction<$M, In = In, Out = ()>,)*
            MPROC: 'static,
            $($M: 'static,)*
        {
            type In = In;
            type Out = Out;

            fn register_processor(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem> {
                let (proc, $($h,)*) = self;
                (
                    app.add_event_processor::<In, Out, _>(proc),
                    $(app.add_event_handler::<In, _>($h),)*
                )
                    .into_configs()
            }
        }
    }
}

all_tuples!(impl_event_processor, 1, 15, SPF, h, M);