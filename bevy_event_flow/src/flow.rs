use crate::Request;
use bevy::app::App;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::ecs::system::SystemId;
use bevy::prelude::{
    Commands, Event, EventReader, EventWriter, In, IntoScheduleConfigs, IntoSystem, Res, Resource,
    SystemInput,
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
        self.init_resource::<EventHandlers<R, Rs>>();
        let id = self.register_system(handler.pipe(send_response::<Rs>));
        self.world_mut()
            .resource_mut::<EventHandlers<R, Rs>>()
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
        self.init_resource::<EventHandlers<R, Rs>>();
        let id = self.register_system(handler.pipe(send_response::<Rs>));
        self.world_mut()
            .resource_mut::<EventHandlers<R, Rs>>()
            .ids
            .push(id);
        self.add_systems(label, process_event::<R, Rs>.after(process_event::<P, R>));
        self
    }
}

#[derive(Resource)]
struct EventHandlers<R, Rs>
where
    R: Request + SystemInput,
    Rs: Event,
{
    ids: Vec<SystemId<R, ()>>,
    _marker: PhantomData<Rs>,
}

impl<R, Rs> Default for EventHandlers<R, Rs>
where
    R: Request + SystemInput,
    Rs: Event,
{
    fn default() -> Self {
        EventHandlers {
            ids: vec![],
            _marker: PhantomData::default(),
        }
    }
}

fn process_event<R, Rs>(
    mut reader: EventReader<R>,
    handler: Res<EventHandlers<R, Rs>>,
    mut commands: Commands,
) where
    R: Request + SystemInput<Inner<'static> = R>,
    Rs: Event,
{
    for event in reader.read() {
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
