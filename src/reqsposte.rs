use bevy::app::App;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::ecs::system::SystemId;
use bevy::prelude::{Commands, Event, EventReader, EventWriter, In, IntoSystem, Res, Resource};

type RequestHandlerId<R> = SystemId<In<R>, ()>;

#[derive(Resource)]
struct RequestHandlerRef<R: BevyRequest> {
    handler_id: RequestHandlerId<R>,
}

pub trait BevyRequests {
    fn add_request<R, H>(&mut self, label: impl ScheduleLabel, handler: H) -> &mut Self
    where
        R: BevyRequest,
        H: IntoSystem<In<R>, R::BevyResponse, ()> + 'static;
}

impl BevyRequests for App {
    fn add_request<R, H>(&mut self, label: impl ScheduleLabel, handler: H) -> &mut Self
    where
        R: BevyRequest,
        H: IntoSystem<In<R>, R::BevyResponse, ()> + 'static,
    {
        self.add_event::<R>();
        self.add_event::<R::BevyResponse>();
        let handler_id = self.register_system(handler.pipe(send_response::<R>));
        self.insert_resource(RequestHandlerRef { handler_id });
        self.add_systems(label, process_request::<R>);
        self
    }
}

fn process_request<R>(
    mut reader: EventReader<R>,
    mut commands: Commands,
    handler: Res<RequestHandlerRef<R>>,
)
where
    R: BevyRequest,
{
    for event in reader.read() {
        commands.run_system_with_input(handler.handler_id, event.clone());
    }
}

fn send_response<R>(In(response): In<R::BevyResponse>, mut writer: EventWriter<R::BevyResponse>)
where
    R: BevyRequest {
    writer.send(response);
}

pub trait BevyRequest: Event + Clone {
    type BevyResponse: Event;
}
