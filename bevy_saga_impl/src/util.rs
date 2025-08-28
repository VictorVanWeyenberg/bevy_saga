use crate::SagaEvent;
use bevy::ecs::system::SystemId;
use bevy::prelude::{Commands, Event, EventWriter, Events, In, Res, ResMut, Resource};

#[derive(Resource)]
pub struct EventProcessors<R>
where
    R: SagaEvent,
{
    #[cfg(test)]
    pub ids: Vec<SystemId<R, ()>>,
    #[cfg(not(test))]
    ids: Vec<SystemId<R, ()>>,
}

impl<R> Default for EventProcessors<R>
where
    R: SagaEvent,
{
    fn default() -> Self {
        EventProcessors { ids: vec![] }
    }
}

impl<R> EventProcessors<R>
where
    R: SagaEvent,
{
    pub fn push(&mut self, system_id: SystemId<R, ()>) {
        self.ids.push(system_id)
    }
}

pub fn process_event<R>(
    mut reader: ResMut<Events<R>>,
    handler: Res<EventProcessors<R>>,
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

pub fn send_response<Rs>(In(response): In<Rs>, mut writer: EventWriter<Rs>)
where
    Rs: Event,
{
    writer.write(response);
}

pub fn send_option_response<Rs>(In(response): In<Option<Rs>>, mut writer: EventWriter<Rs>)
where
    Rs: Event,
{
    if let Some(response) = response {
        writer.write(response);
    }
}

pub fn send_result_response<Ok, Err>(In(result): In<Result<Ok, Err>>, mut ok_writer: EventWriter<Ok>, mut err_writer: EventWriter<Err>)
where
    Ok: Event,
    Err: Event,
{
    match result {
        Ok(ok) => {
            ok_writer.write(ok);
        },
        Err(err) => {
            err_writer.write(err);
        },
    }
}
