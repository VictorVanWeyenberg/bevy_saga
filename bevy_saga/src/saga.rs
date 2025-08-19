use crate::handler::EventHandler;
use crate::processor::EventProcessor;
use crate::SagaEvent;
use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{App, IntoScheduleConfigs};

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

impl<S1, H, M1, MH, In> Saga<(M1, MH)> for (S1, H)
where
    S1: EventProcessor<M1, In = In>,
    H: EventHandler<MH, In = S1::Out>,
    In: SagaEvent,
    H::In: SagaEvent,
{
    type In = In;

    fn register(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem> {
        let (s1, h) = self;
        (s1.register_processor(app), h.register_handler(app)).chain()
    }
}

impl<S1, S2, H, M1, M2, MH, In> Saga<(M1, M2, MH)> for (S1, S2, H)
where
    S1: EventProcessor<M1, In = In>,
    S2: EventProcessor<M2, In = S1::Out>,
    H: EventHandler<MH, In = S2::Out>,
    In: SagaEvent,
    S2::In: SagaEvent,
    H::In: SagaEvent,
{
    type In = In;

    fn register(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem> {
        let (s1, s2, h) = self;
        (
            s1.register_processor(app),
            s2.register_processor(app),
            h.register_handler(app),
        )
            .chain()
    }
}

impl<S1, S2, S3, H, M1, M2, M3, MH, In> Saga<(M1, M2, M3, MH)> for (S1, S2, S3, H)
where
    S1: EventProcessor<M1, In = In>,
    S2: EventProcessor<M2, In = S1::Out>,
    S3: EventProcessor<M3, In = S2::Out>,
    H: EventHandler<MH, In = S3::Out>,
    In: SagaEvent,
    S2::In: SagaEvent,
    S3::In: SagaEvent,
    H::In: SagaEvent,
{
    type In = In;

    fn register(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem> {
        let (s1, s2, s3, h) = self;
        (
            s1.register_processor(app),
            s2.register_processor(app),
            s3.register_processor(app),
            h.register_handler(app),
        )
            .chain()
    }
}
