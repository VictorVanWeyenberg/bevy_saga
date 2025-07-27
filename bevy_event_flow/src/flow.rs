use crate::util::{send_response, EventHandlers, EventProcessors};
use crate::{EventProcessorFlow, Request};
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::{App, Event, IntoSystem, SystemInput};

pub trait RegisterEventFlow {
    fn add_event_processor_flow<M, L>(&mut self, label: L, flow: impl EventProcessorFlow<M>) -> &mut Self
    where
        L: ScheduleLabel + Clone;
}

impl RegisterEventFlow for App {
    fn add_event_processor_flow<M, L>(&mut self, label: L, flow: impl EventProcessorFlow<M>) -> &mut Self
    where
        L: ScheduleLabel + Clone
    {
        // TODO: register is visible to everything that knows EventProcessorFlow.
        flow.register_flow(label, self);
        self
    }
}

pub trait EventFlow {
    fn add_event_flow<R, Rs, M>(
        &mut self,
        handler: impl IntoSystem<R, Rs, M> + 'static,
    ) -> &mut Self
    where
        R: Request + SystemInput<Inner<'static> = R>,
        Rs: Event;

    fn add_event_handler<R, M>(
        &mut self,
        handler: impl IntoSystem<R, (), M> + 'static,
    ) -> &mut Self
    where
        R: Request + SystemInput<Inner<'static> = R>;
}

impl EventFlow for App {
    fn add_event_flow<R, Rs, M>(
        &mut self,
        handler: impl IntoSystem<R, Rs, M> + 'static,
    ) -> &mut Self
    where
        R: Request + SystemInput<Inner<'static> = R>,
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
        R: Request + SystemInput<Inner<'static> = R>,
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