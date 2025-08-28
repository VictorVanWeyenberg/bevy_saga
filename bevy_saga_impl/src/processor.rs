use crate::{SagaEvent, extension::BevySagaUtil};
use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{App, Event, IntoScheduleConfigs, SystemParamFunction};
use variadics_please::all_tuples;

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