use crate::SagaEvent;
use bevy::ecs::system::SystemId;
use bevy::prelude::{Commands, Event, EventWriter, Events, In, Res, ResMut, Resource};
use std::marker::PhantomData;

#[derive(Resource)]
pub struct EventProcessors<R, Rs>
where
    R: SagaEvent,
    Rs: Sized + Send + Sync,
{
    ids: Vec<SystemId<R, ()>>,
    _marker: PhantomData<Rs>,
}

impl<R, Rs> Default for EventProcessors<R, Rs>
where
    R: SagaEvent,
    Rs: Sized + Send + Sync,
{
    fn default() -> Self {
        EventProcessors {
            ids: vec![],
            _marker: PhantomData,
        }
    }
}

impl<R, Rs> EventProcessors<R, Rs>
where
    R: SagaEvent,
    Rs: Sized + Send + Sync,
{
    pub fn push(&mut self, system_id: SystemId<R, ()>) {
        self.ids.push(system_id)
    }
}

#[derive(Resource)]
pub struct EventHandlers<R>
where
    R: SagaEvent,
{
    ids: Vec<SystemId<R, ()>>,
}

impl<R> Default for EventHandlers<R>
where
    R: SagaEvent,
{
    fn default() -> Self {
        EventHandlers { ids: vec![] }
    }
}

impl<R> EventHandlers<R>
where
    R: SagaEvent,
{
    pub fn push(&mut self, system_id: SystemId<R, ()>) {
        self.ids.push(system_id)
    }
}

pub fn process_event<R, Rs>(
    mut reader: ResMut<Events<R>>,
    handler: Res<EventProcessors<R, Rs>>,
    mut commands: Commands,
) where
    R: SagaEvent,
    Rs: Event,
{
    for event in reader.drain() {
        for id in &handler.ids {
            commands.run_system_with(*id, event.clone())
        }
    }
}

pub fn send_response<Rs>(In(response): In<Rs>, mut writer: EventWriter<Rs>)
where
    Rs: Event,
{
    writer.write(response);
}

pub fn handle_event<R>(
    mut reader: ResMut<Events<R>>,
    handler: Res<EventHandlers<R>>,
    mut commands: Commands,
) where
    R: SagaEvent,
{
    for event in reader.drain() {
        for id in &handler.ids {
            commands.run_system_with(*id, event.clone())
        }
    }
}
