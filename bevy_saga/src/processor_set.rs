use crate::{
    saga::BevySagaUtil,
    SagaEvent,
};
use bevy::prelude::{App, Event, SystemInput, SystemParamFunction};

pub trait EventProcessorSet<M> {
    type In: SagaEvent + SystemInput<Inner<'static> = Self::In>;
    type Out: Event;

    fn register_processor(self, app: &mut App);
}

impl<SPF, M, In, Out> EventProcessorSet<(M,)> for SPF
where
    In: SagaEvent + SystemInput<Inner<'static> = In>,
    Out: Event,
    SPF: SystemParamFunction<M, In = In, Out = Out>,
    M: 'static,
{
    type In = In;
    type Out = Out;

    fn register_processor(self, app: &mut App) {
        app.add_event_processor::<In, Out, _>(self);
    }
}

impl<SPF1, SPF2, M1, M2, In, Out> EventProcessorSet<(M1, M2)> for (SPF1, SPF2)
where
    In: SagaEvent + SystemInput<Inner<'static> = In>,
    Out: Event,
    SPF1: SystemParamFunction<M1, In = In, Out = Out>,
    SPF2: SystemParamFunction<M2, In = In, Out = Out>,
    M1: 'static,
    M2: 'static,
{
    type In = In;
    type Out = Out;

    fn register_processor(self, app: &mut App) {
        let (spf1, spf2) = self;
        app.add_event_processor::<In, Out, _>(spf1);
        app.add_event_processor::<In, Out, _>(spf2);
    }
}

impl<SPF1, SPF2, SPF3, M1, M2, M3, In, Out> EventProcessorSet<(M1, M2, M3)> for (SPF1, SPF2, SPF3)
where
    In: SagaEvent + SystemInput<Inner<'static> = In>,
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

    fn register_processor(self, app: &mut App) {
        let (spf1, spf2, spf3) = self;
        app.add_event_processor::<In, Out, _>(spf1);
        app.add_event_processor::<In, Out, _>(spf2);
        app.add_event_processor::<In, Out, _>(spf3);
    }
}

impl<SPF1, SPF2, SPF3, SPF4, M1, M2, M3, M4, In, Out> EventProcessorSet<(M1, M2, M3, M4)> for (SPF1, SPF2, SPF3, SPF4)
where
    In: SagaEvent + SystemInput<Inner<'static> = In>,
    Out: SagaEvent,
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

    fn register_processor(self, app: &mut App) {
        let (spf1, spf2, spf3, spf4) = self;
        app.add_event_processor::<In, Out, _>(spf1);
        app.add_event_processor::<In, Out, _>(spf2);
        app.add_event_processor::<In, Out, _>(spf3);
        app.add_event_processor::<In, Out, _>(spf4);
    }
}

