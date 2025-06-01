use crate::Request;
use bevy::app::App;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::ecs::system::SystemId;
use bevy::prelude::{
    Commands, Event, EventWriter, Events, In, IntoScheduleConfigs, IntoSystem, Res, ResMut,
    Resource, SystemInput,
};
use std::marker::PhantomData;

pub trait EventFlow {
    fn add_event_flow<R, Rs, M>(
        &mut self,
        label: impl ScheduleLabel,
        handler: impl IntoSystem<R, Rs, M> + 'static,
    ) -> &mut Self
    where
        R: Request + SystemInput<Inner<'static> = R>,
        Rs: Event;

    fn add_event_flow_after<P, R, Rs, M>(
        &mut self,
        label: impl ScheduleLabel,
        handler: impl IntoSystem<R, Rs, M> + 'static,
    ) -> &mut Self
    where
        R: Request + SystemInput<Inner<'static> = R>,
        Rs: Event,
        P: Request + SystemInput<Inner<'static> = P>;

    fn add_event_handler<R, M>(
        &mut self,
        label: impl ScheduleLabel,
        handler: impl IntoSystem<R, (), M> + 'static,
    ) -> &mut Self
    where
        R: Request + SystemInput<Inner<'static> = R>;

    fn add_event_handler_after<P, R, M>(
        &mut self,
        label: impl ScheduleLabel,
        handler: impl IntoSystem<R, (), M> + 'static,
    ) -> &mut Self
    where
        R: Request + SystemInput<Inner<'static> = R>,
        P: Request + SystemInput<Inner<'static> = P>;
}

impl EventFlow for App {
    fn add_event_flow<R, Rs, M>(
        &mut self,
        label: impl ScheduleLabel,
        handler: impl IntoSystem<R, Rs, M> + 'static,
    ) -> &mut Self
    where
        R: Request + SystemInput<Inner<'static> = R>,
        Rs: Event,
    {
        self.add_event::<R>();
        self.add_event::<Rs>();
        self.init_resource::<EventProcessors<R, Rs>>();
        let id = self.register_system(handler.pipe(send_response::<Rs>));
        self.world_mut()
            .resource_mut::<EventProcessors<R, Rs>>()
            .ids
            .push(id);
        self.add_systems(label, process_event::<R, Rs>);
        self
    }

    fn add_event_flow_after<P, R, Rs, M>(
        &mut self,
        label: impl ScheduleLabel,
        handler: impl IntoSystem<R, Rs, M> + 'static,
    ) -> &mut Self
    where
        R: Request + SystemInput<Inner<'static> = R>,
        P: Request + SystemInput<Inner<'static> = P>,
        Rs: Event,
    {
        self.add_event::<R>();
        self.add_event::<Rs>();
        self.init_resource::<EventProcessors<R, Rs>>();
        let id = self.register_system(handler.pipe(send_response::<Rs>));
        self.world_mut()
            .resource_mut::<EventProcessors<R, Rs>>()
            .ids
            .push(id);
        self.add_systems(label, process_event::<R, Rs>.after(process_event::<P, R>));
        self
    }

    fn add_event_handler<R, M>(
        &mut self,
        label: impl ScheduleLabel,
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
            .ids
            .push(id);
        self.add_systems(label, handle_event::<R>);
        self
    }

    fn add_event_handler_after<P, R, M>(
        &mut self,
        label: impl ScheduleLabel,
        handler: impl IntoSystem<R, (), M> + 'static,
    ) -> &mut Self
    where
        R: Request + SystemInput<Inner<'static> = R>,
        P: Request + SystemInput<Inner<'static> = P>,
    {
        self.add_event::<R>();
        self.init_resource::<EventHandlers<R>>();
        let id = self.register_system(handler);
        self.world_mut()
            .resource_mut::<EventHandlers<R>>()
            .ids
            .push(id);
        self.add_systems(label, handle_event::<R>.after(process_event::<P, R>));
        self
    }
}

#[derive(Resource)]
struct EventProcessors<R, Rs>
where
    R: Request + SystemInput,
    Rs: Event,
{
    ids: Vec<SystemId<R, ()>>,
    _marker: PhantomData<Rs>,
}

impl<R, Rs> Default for EventProcessors<R, Rs>
where
    R: Request + SystemInput,
    Rs: Event,
{
    fn default() -> Self {
        EventProcessors {
            ids: vec![],
            _marker: PhantomData::default(),
        }
    }
}

#[derive(Resource)]
struct EventHandlers<R>
where
    R: Request + SystemInput,
{
    ids: Vec<SystemId<R, ()>>,
}

impl<R> Default for EventHandlers<R>
where
    R: Request + SystemInput,
{
    fn default() -> Self {
        EventHandlers { ids: vec![] }
    }
}

fn process_event<R, Rs>(
    mut reader: ResMut<Events<R>>,
    handler: Res<EventProcessors<R, Rs>>,
    mut commands: Commands,
) where
    R: Request + SystemInput<Inner<'static> = R>,
    Rs: Event,
{
    for event in reader.drain() {
        for id in &handler.ids {
            commands.run_system_with(*id, event.clone())
        }
    }
}

fn send_response<Rs>(In(response): In<Rs>, mut writer: EventWriter<Rs>)
where
    Rs: Event,
{
    writer.write(response);
}

fn handle_event<R>(
    mut reader: ResMut<Events<R>>,
    handler: Res<EventHandlers<R>>,
    mut commands: Commands,
) where
    R: Request + SystemInput<Inner<'static> = R>,
{
    for event in reader.drain() {
        for id in &handler.ids {
            commands.run_system_with(*id, event.clone())
        }
    }
}
