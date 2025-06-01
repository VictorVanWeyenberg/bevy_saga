use crate::Request;
use bevy::app::App;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::ecs::system::SystemId;
use bevy::prelude::{Commands, Event, EventReader, EventWriter, In, IntoScheduleConfigs, IntoSystem, Res, Resource, SystemInput};

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
        let id = self.register_system(handler.pipe(send_response::<Rs>));
        self.insert_resource(EventHandlerId { id });
        self.add_systems(label, process_event::<R>);
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
        let id = self.register_system(handler.pipe(send_response::<Rs>));
        self.insert_resource(EventHandlerId { id });
        self.add_systems(label, process_event::<R>.after(process_event::<P>));
        self
    }
}

#[derive(Resource)]
struct EventHandlerId<R>
where
    R: Request + SystemInput,
{
    id: SystemId<R, ()>,
}

fn process_event<R>(
    mut reader: EventReader<R>,
    handler: Res<EventHandlerId<R>>,
    mut commands: Commands,
) where
    R: Request + SystemInput<Inner<'static> = R>,
{
    for event in reader.read().cloned() {
        commands.run_system_with(handler.id, event)
    }
}

fn send_response<Rs>(In(response): In<Rs>, mut writer: EventWriter<Rs>)
where
    Rs: Event,
{
    writer.write(response);
}
