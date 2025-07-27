use crate::util::{send_response, EventHandlers, EventProcessors};
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::{App, Event, IntoSystem, SystemInput};
use crate::processor_saga::Saga;
use crate::SagaEvent;

pub trait RegisterEventSaga {
    fn add_saga<M, L>(&mut self, label: L, saga: impl Saga<M>) -> &mut Self
    where
        L: ScheduleLabel + Clone;
}

impl RegisterEventSaga for App {
    fn add_saga<M, L>(&mut self, label: L, saga: impl Saga<M>) -> &mut Self
    where
        L: ScheduleLabel + Clone
    {
        // TODO: register is visible to everything that knows Saga.
        saga.register(label, self);
        self
    }
}

pub trait BevySagaUtil {
    fn add_event_processor<R, Rs, M>(
        &mut self,
        handler: impl IntoSystem<R, Rs, M> + 'static,
    ) -> &mut Self
    where
        R: SagaEvent + SystemInput<Inner<'static> = R>,
        Rs: Event;

    fn add_event_handler<R, M>(
        &mut self,
        handler: impl IntoSystem<R, (), M> + 'static,
    ) -> &mut Self
    where
        R: SagaEvent + SystemInput<Inner<'static> = R>;
}

impl BevySagaUtil for App {
    fn add_event_processor<R, Rs, M>(
        &mut self,
        handler: impl IntoSystem<R, Rs, M> + 'static,
    ) -> &mut Self
    where
        R: SagaEvent + SystemInput<Inner<'static> = R>,
        Rs: Event,
    {
        self.add_event::<R>();
        self.init_resource::<EventProcessors<R, Rs>>();
        let id = self.register_system(handler.pipe(send_response::<Rs>));
        self.world_mut()
            .resource_mut::<EventProcessors<R, Rs>>()
            .push(id);
        self
    }

    fn add_event_handler<R, M>(
        &mut self,
        handler: impl IntoSystem<R, (), M> + 'static,
    ) -> &mut Self
    where
        R: SagaEvent + SystemInput<Inner<'static> = R>,
    {
        self.add_event::<R>();
        self.init_resource::<EventHandlers<R>>();
        let id = self.register_system(handler);
        self.world_mut()
            .resource_mut::<EventHandlers<R>>()
            .push(id);
        self
    }
}