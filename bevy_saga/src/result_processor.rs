use crate::SagaEvent;
use crate::plugin::BevySagaUtil;
use bevy::app::App;
use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{Event, IntoScheduleConfigs, SystemParamFunction};
use variadics_please::all_tuples;

pub trait ResultProcessor<M> {
    type In: SagaEvent;
    type Ok: Event;
    type Err: Event;

    fn register_result_processor(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem>;
}

impl<RS, MRS, In, Ok, Err> ResultProcessor<(MRS,)> for RS
where
    RS: SystemParamFunction<MRS, In = In, Out = Result<Ok, Err>>,
    In: SagaEvent,
    Ok: Event,
    Err: Event,
    MRS: 'static,
{
    type In = In;
    type Ok = Ok;
    type Err = Err;

    fn register_result_processor(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem> {
        app.add_result_handler(self)
    }
}

macro_rules! impl_result_processor {
    ($(#[$meta:meta])* $(($RH:ident, $rh:ident, $MRH:ident)),*) => {
        impl<RS, MRS, $($RH,)* $($MRH,)* In, Ok, Err> ResultProcessor<(MRS, $($MRH,)*)> for (RS, $($RH,)*)
        where
            RS: SystemParamFunction<MRS, In = In, Out = Result<Ok, Err>>,
            $($RH: SystemParamFunction<$MRH, In = In, Out = ()>,)*
            In: SagaEvent,
            Ok: Event,
            Err: Event,
            MRS: 'static,
            $($MRH: 'static,)*
        {
            type In = In;
            type Ok = Ok;
            type Err = Err;

            fn register_result_processor(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem> {
                let (rs, $($rh,)*) = self;
                (
                    app.add_result_handler(rs),
                    $(app.add_event_handler($rh),)*
                )
                    .into_configs()
            }
        }
    }
}

all_tuples!(impl_result_processor, 1, 15, RH, rh, MRH);
