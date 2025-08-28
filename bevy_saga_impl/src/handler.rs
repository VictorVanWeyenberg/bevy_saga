use crate::{extension::BevySagaUtil, SagaEvent};
use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{App, IntoScheduleConfigs, SystemParamFunction};
use variadics_please::all_tuples;

pub trait EventHandler<M> {
    type In: SagaEvent;

    fn register_handler(
        self,
        app: &mut App,
    ) -> ScheduleConfigs<ScheduleSystem>;
}

impl<SPF, M, In> EventHandler<(M,)> for SPF
where
    In: SagaEvent,
    SPF: SystemParamFunction<M, In = In, Out = ()>,
    M: 'static,
{
    type In = In;

    fn register_handler(
        self,
        app: &mut App,
    ) -> ScheduleConfigs<ScheduleSystem>
    {
        app.add_event_handler::<In, _>(self)
    }
}

macro_rules! impl_event_handler {
    ($(#[$meta:meta])* $(($SPF:ident, $spf:ident, $M:ident)),*) => {
        impl<$($SPF,)* $($M,)* In> EventHandler<($($M,)*)> for ($($SPF,)*)
        where
            In: SagaEvent,
            $($SPF: SystemParamFunction<$M, In = In, Out = ()>,)*
            $($M: 'static,)*
        {
            type In = In;

            fn register_handler(
                self,
                app: &mut App,
            ) -> ScheduleConfigs<ScheduleSystem>
            {
                let ($($spf,)*) = self;
                (
                    $(app.add_event_handler::<In, _>($spf),)*
                )
                    .into_configs()
            }
        }
    }
}

all_tuples!(impl_event_handler, 2, 16, SPF, spf, M);