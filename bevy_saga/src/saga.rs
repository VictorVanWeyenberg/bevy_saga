use crate::handler::EventHandler;
use crate::processor::EventProcessor;
use crate::util::process_event;
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

impl<S1, S2, M1, M2, In> Saga<(M1, M2)> for (S1, S2)
where
    In: SagaEvent,
    S1: EventProcessor<M1, In = In>,
    S2: EventHandler<M2, In = S1::Out>,
    S2::In: SagaEvent,
{
    type In = In;

    fn register(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem> {
        let (s1, s2) = self;
        (s1.register_processor(app), s2.register_handler(app)).chain()
    }
}

impl<S1, S2, S3, M1, M2, M3, In> Saga<(M1, M2, M3)> for (S1, S2, S3)
where
    In: SagaEvent,
    S1: EventProcessor<M1, In = In>,
    S2: EventProcessor<M2, In = S1::Out>,
    S3: EventHandler<M3, In = S2::Out>,
    S2::In: SagaEvent,
    S3::In: SagaEvent,
{
    type In = In;

    fn register(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem> {
        let (s1, s2, s3) = self;
        s1.register_processor(app);
        s2.register_processor(app);
        s3.register_handler(app);
        (
            process_event::<S1::In>,
            process_event::<S2::In>,
            process_event::<S3::In>,
        )
            .chain()
    }
}

impl<S1, S2, S3, S4, M1, M2, M3, M4, In> Saga<(M1, M2, M3, M4)> for (S1, S2, S3, S4)
where
    In: SagaEvent,
    S1: EventProcessor<M1, In = In>,
    S2: EventProcessor<M2, In = S1::Out>,
    S3: EventProcessor<M3, In = S2::Out>,
    S4: EventHandler<M4, In = S3::Out>,
    S2::In: SagaEvent,
    S3::In: SagaEvent,
    S4::In: SagaEvent,
{
    type In = In;

    fn register(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem> {
        let (s1, s2, s3, s4) = self;
        s1.register_processor(app);
        s2.register_processor(app);
        s3.register_processor(app);
        s4.register_handler(app);
        (
            process_event::<S1::In>,
            process_event::<S2::In>,
            process_event::<S3::In>,
            process_event::<S4::In>,
        )
            .chain()
    }
}
