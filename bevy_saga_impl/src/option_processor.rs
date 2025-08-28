use crate::processor::EventProcessor;
use crate::{SagaEvent, plugin::BevySagaUtil};
use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{App, Event, IntoScheduleConfigs, SystemParamFunction};
use variadics_please::all_tuples;

pub struct OptionProcessor<T>(T);

impl<SPF, M, In, Out> EventProcessor<OptionProcessor<(M,)>> for SPF
where
    In: SagaEvent,
    Out: Event,
    SPF: SystemParamFunction<M, In = In, Out = Option<Out>>,
    M: 'static,
{
    type In = In;
    type Out = Out;

    fn register_processor(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem> {
        app.add_option_processor::<In, Out, _>(self)
    }
}

macro_rules! impl_option_processor {
    ($(#[$meta:meta])* $(($SPF:ident, $p:ident, $M:ident)),*) => {
        impl<PROC, MPROC, $($SPF,)* $($M,)* In, Out> EventProcessor<OptionProcessor<(MPROC, $($M,)*)>> for (PROC, $($SPF,)*)
        where
            In: SagaEvent,
            Out: Event,
            PROC: SystemParamFunction<MPROC, In = In, Out = Option<Out>>,
            $($SPF: SystemParamFunction<$M, In = In, Out = ()>,)*
            MPROC: 'static,
            $($M: 'static,)*
        {
            type In = In;
            type Out = Out;

            fn register_processor(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem> {
                let (proc, $($p,)*) = self;
                (
                    app.add_option_processor::<In, Out, _>(proc),
                    $(app.add_event_handler::<In, _>($p),)*
                )
                    .into_configs()
            }
        }
    }
}

all_tuples!(impl_option_processor, 1, 15, SPF, p, M);