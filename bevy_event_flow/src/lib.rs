use bevy::ecs::schedule::ScheduleLabel;
use bevy::ecs::system::SystemId;
use bevy::prelude::{App, Commands, Event, EventReader, EventWriter, In, IntoSystem, Res, Resource};

pub trait EventFlow {
    fn add_request<M, R, H, L>(&mut self, label: L, handler: H) -> &mut Self
    where
        R: Request,
        H: IntoSystem<In<R>, R::Response, M> + 'static,
        L: ScheduleLabel;
}

impl EventFlow for App {
    fn add_request<M, R, H, L>(&mut self, label: L, handler: H) -> &mut Self
    where
        R: Request,
        H: IntoSystem<In<R>, R::Response, M> + 'static,
        L: ScheduleLabel,
    {
        self.add_event::<R>();
        self.add_event::<R::Response>();
        let id = self.register_system(handler.pipe(send_response::<R>));
        self.insert_resource(EventHandlerId { id });
        self.add_systems(label, process_event::<R>);
        self
    }
}

#[derive(Resource)]
struct EventHandlerId<R>
where
    R: Request,
{
    id: SystemId<In<R>, ()>,
}

pub trait Request: Event + Clone {
    type Response: Event;
}

fn process_event<R>(
    mut reader: EventReader<R>,
    handler: Res<EventHandlerId<R>>,
    mut commands: Commands,
) where
    R: Request,
{
    for event in reader.read().cloned() {
        commands.run_system_with(handler.id, event)
    }
}

fn send_response<R>(In(response): In<R::Response>, mut writer: EventWriter<R::Response>)
where
    R: Request,
{
    writer.write(response);
}
