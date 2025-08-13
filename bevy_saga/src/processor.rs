use crate::{SagaEvent, plugin::BevySagaUtil};
use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{App, Event, IntoScheduleConfigs, SystemParamFunction};

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

impl<SPF1, SPF2, M1, M2, In, Out> EventProcessor<(M1, M2)> for (SPF1, SPF2)
where
    In: SagaEvent,
    Out: Event,
    SPF1: SystemParamFunction<M1, In = In, Out = Out>,
    SPF2: SystemParamFunction<M2, In = In, Out = Out>,
    M1: 'static,
    M2: 'static,
{
    type In = In;
    type Out = Out;

    fn register_processor(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem> {
        let (spf1, spf2) = self;
        (
            app.add_event_processor::<In, Out, _>(spf1),
            app.add_event_processor::<In, Out, _>(spf2),
        )
            .into_configs()
    }
}

impl<SPF1, SPF2, SPF3, M1, M2, M3, In, Out> EventProcessor<(M1, M2, M3)> for (SPF1, SPF2, SPF3)
where
    In: SagaEvent,
    Out: Event,
    SPF1: SystemParamFunction<M1, In = In, Out = Out>,
    SPF2: SystemParamFunction<M2, In = In, Out = Out>,
    SPF3: SystemParamFunction<M3, In = In, Out = Out>,
    M1: 'static,
    M2: 'static,
    M3: 'static,
{
    type In = In;
    type Out = Out;

    fn register_processor(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem> {
        let (spf1, spf2, spf3) = self;
        (
            app.add_event_processor::<In, Out, _>(spf1),
            app.add_event_processor::<In, Out, _>(spf2),
            app.add_event_processor::<In, Out, _>(spf3),
        )
            .into_configs()
    }
}

impl<SPF1, SPF2, SPF3, SPF4, M1, M2, M3, M4, In, Out> EventProcessor<(M1, M2, M3, M4)>
    for (SPF1, SPF2, SPF3, SPF4)
where
    In: SagaEvent,
    Out: Event,
    SPF1: SystemParamFunction<M1, In = In, Out = Out>,
    SPF2: SystemParamFunction<M2, In = In, Out = Out>,
    SPF3: SystemParamFunction<M3, In = In, Out = Out>,
    SPF4: SystemParamFunction<M4, In = In, Out = Out>,
    M1: 'static,
    M2: 'static,
    M3: 'static,
    M4: 'static,
{
    type In = In;
    type Out = Out;

    fn register_processor(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem> {
        let (spf1, spf2, spf3, spf4) = self;
        (
            app.add_event_processor::<In, Out, _>(spf1),
            app.add_event_processor::<In, Out, _>(spf2),
            app.add_event_processor::<In, Out, _>(spf3),
            app.add_event_processor::<In, Out, _>(spf4),
        )
            .into_configs()
    }
}
