use crate::saga::Saga;
use crate::util::{send_option_response, send_response, send_result_response, EventHandlers, EventProcessors};
use crate::SagaEvent;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::{App, Event, IntoSystem};

pub trait RegisterSaga {
    fn add_saga<M, L>(&mut self, label: L, saga: impl Saga<M>) -> &mut Self
    where
        L: ScheduleLabel + Clone;
}

impl RegisterSaga for App {
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
        R: SagaEvent,
        Rs: Event;

    fn add_option_processor<R, Rs, M>(
        &mut self,
        handler: impl IntoSystem<R, Option<Rs>, M> + 'static,
    ) -> &mut Self
    where
        R: SagaEvent,
        Rs: Event;

    fn add_result_handler<R, Ok, Err, M>(&mut self, handler: impl IntoSystem<R, Result<Ok, Err>, M> + 'static) -> &mut Self
    where
        R: SagaEvent,
        Ok: Event,
        Err: Event;

    fn add_event_handler<R, M>(
        &mut self,
        handler: impl IntoSystem<R, (), M> + 'static,
    ) -> &mut Self
    where
        R: SagaEvent;
}

impl BevySagaUtil for App {
    fn add_event_processor<R, Rs, M>(
        &mut self,
        handler: impl IntoSystem<R, Rs, M> + 'static,
    ) -> &mut Self
    where
        R: SagaEvent,
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

    fn add_option_processor<R, Rs, M>(&mut self, handler: impl IntoSystem<R, Option<Rs>, M> + 'static) -> &mut Self
    where
        R: SagaEvent,
        Rs: Event
    {
        self.add_event::<R>();
        self.init_resource::<EventProcessors<R, Rs>>();
        let id = self.register_system(handler.pipe(send_option_response::<Rs>));
        self.world_mut()
            .resource_mut::<EventProcessors<R, Rs>>()
            .push(id);
        self
    }

    fn add_result_handler<R, Ok, Err, M>(&mut self, handler: impl IntoSystem<R, Result<Ok, Err>, M> + 'static) -> &mut Self
    where
        R: SagaEvent,
        Ok: Event,
        Err: Event,
    {
        self.add_event::<R>();
        self.init_resource::<EventHandlers<R>>();
        self.init_resource::<EventProcessors<R, Result<Ok, Err>>>();
        let id = self.register_system(handler.pipe(send_result_response::<Ok, Err>));
        self.world_mut()
            .resource_mut::<EventProcessors<R, Result<Ok, Err>>>()
            .push(id);
        self
    }

    fn add_event_handler<R, M>(
        &mut self,
        handler: impl IntoSystem<R, (), M> + 'static,
    ) -> &mut Self
    where
        R: SagaEvent,
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