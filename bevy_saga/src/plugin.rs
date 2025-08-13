use crate::SagaEvent;
use crate::saga::Saga;
use crate::util::{EventProcessors, process_event, send_option_response, send_response, send_result_response, handle_event};
use bevy::ecs::schedule::{ScheduleConfigs, ScheduleLabel};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{App, Event, IntoScheduleConfigs, IntoSystem};

pub trait RegisterSaga {
    fn add_saga<M, L>(&mut self, label: L, saga: impl Saga<M>) -> &mut Self
    where
        L: ScheduleLabel + Clone;
}

impl RegisterSaga for App {
    fn add_saga<M, L>(&mut self, label: L, saga: impl Saga<M>) -> &mut Self
    where
        L: ScheduleLabel + Clone,
    {
        // TODO: register is visible to everything that knows Saga.
        let schedules = saga.register(self);
        self.add_systems(label, schedules)
    }
}

pub trait BevySagaUtil {
    fn add_event_processor<R, Rs, M>(
        &mut self,
        handler: impl IntoSystem<R, Rs, M> + 'static,
    ) -> ScheduleConfigs<ScheduleSystem>
    where
        R: SagaEvent,
        Rs: Event;

    fn add_option_processor<R, Rs, M>(
        &mut self,
        handler: impl IntoSystem<R, Option<Rs>, M> + 'static,
    ) -> ScheduleConfigs<ScheduleSystem>
    where
        R: SagaEvent,
        Rs: Event;

    fn add_result_handler<R, Ok, Err, M>(
        &mut self,
        handler: impl IntoSystem<R, Result<Ok, Err>, M> + 'static,
    ) -> ScheduleConfigs<ScheduleSystem>
    where
        R: SagaEvent,
        Ok: Event,
        Err: Event;

    fn add_event_handler<R, M>(
        &mut self,
        handler: impl IntoSystem<R, (), M> + 'static,
    ) -> ScheduleConfigs<ScheduleSystem>
    where
        R: SagaEvent;
}

impl BevySagaUtil for App {
    fn add_event_processor<R, Rs, M>(
        &mut self,
        handler: impl IntoSystem<R, Rs, M> + 'static,
    ) -> ScheduleConfigs<ScheduleSystem>
    where
        R: SagaEvent,
        Rs: Event,
    {
        self.add_event::<R>();
        self.init_resource::<EventProcessors<R>>();
        let id = self.register_system(handler.pipe(send_response::<Rs>));
        self.world_mut()
            .resource_mut::<EventProcessors<R>>()
            .push(id);
        process_event::<R, Rs>.into_configs()
    }

    fn add_option_processor<R, Rs, M>(
        &mut self,
        handler: impl IntoSystem<R, Option<Rs>, M> + 'static,
    ) -> ScheduleConfigs<ScheduleSystem>
    where
        R: SagaEvent,
        Rs: Event,
    {
        self.add_event::<R>();
        self.init_resource::<EventProcessors<R>>();
        let id = self.register_system(handler.pipe(send_option_response::<Rs>));
        self.world_mut()
            .resource_mut::<EventProcessors<R>>()
            .push(id);
        process_event::<R, Rs>.into_configs()
    }

    fn add_result_handler<R, Ok, Err, M>(
        &mut self,
        handler: impl IntoSystem<R, Result<Ok, Err>, M> + 'static,
    ) -> ScheduleConfigs<ScheduleSystem>
    where
        R: SagaEvent,
        Ok: Event,
        Err: Event,
    {
        self.add_event::<R>();
        self.init_resource::<EventProcessors<R>>();
        self.init_resource::<EventProcessors<R>>();
        let id = self.register_system(handler.pipe(send_result_response::<Ok, Err>));
        self.world_mut().resource_mut::<EventProcessors<R>>().push(id);
        handle_event::<R>.into_configs()
    }

    fn add_event_handler<R, M>(
        &mut self,
        handler: impl IntoSystem<R, (), M> + 'static,
    ) -> ScheduleConfigs<ScheduleSystem>
    where
        R: SagaEvent,
    {
        self.add_event::<R>();
        self.init_resource::<EventProcessors<R>>();
        let id = self.register_system(handler);
        self.world_mut().resource_mut::<EventProcessors<R>>().push(id);
        handle_event::<R>.into_configs()
    }
}
