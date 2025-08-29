use crate::SagaEvent;
use crate::saga::Saga;
use crate::util::{EventProcessors, process_event, send_option_response, send_response, send_result_response};
use bevy::ecs::schedule::{ScheduleConfigs, ScheduleLabel};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{App, Event, IntoScheduleConfigs, IntoSystem};

/// The extension trait where sagas are added to the bevy App.
/// 
/// This is the place where you add sagas to the [app](App). Define a saga and pass it to the 
/// [add_saga](SagaRegistry::add_saga) method together with a [ScheduleLabel](ScheduleLabel).
/// 
/// During the update cycle, when the schedule under the label is executed, all sent events will be 
/// propagated through the saga in one update cycle.
/// 
/// If multiple sagas are registered under the same label, they will be executed concurrently.
/// To order sagas in reference to each other, we recommend to add extra 
/// [ScheduleLabels](ScheduleLabel).
/// 
/// Learn how to write a saga [here](Saga).
pub trait SagaRegistry {
    fn add_saga<M, L>(&mut self, label: L, saga: impl Saga<M>) -> &mut Self
    where
        L: ScheduleLabel + Clone;
}

impl SagaRegistry for App {
    fn add_saga<M, L>(&mut self, label: L, saga: impl Saga<M>) -> &mut Self
    where
        L: ScheduleLabel + Clone,
    {
        // TODO: register is visible to everything that knows Saga.
        let schedules = saga.register(self);
        self.add_systems(label, schedules)
    }
}

/// A trait used by bevy_saga to add the SystemIds of your event processors and handlers to the 
/// [EventProcessors](EventProcessors) resource.
///
/// It's not recommended to use this trait in your own code. It's exported from the crate for the
///`#[saga_router]` macro.
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
        process_event::<R>.into_configs()
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
        process_event::<R>.into_configs()
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
        let id = self.register_system(handler.pipe(send_result_response::<Ok, Err>));
        self.world_mut().resource_mut::<EventProcessors<R>>().push(id);
        process_event::<R>.into_configs()
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
        process_event::<R>.into_configs()
    }
}
