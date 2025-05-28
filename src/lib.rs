use bevy::ecs::schedule::ScheduleLabel;
use bevy::ecs::system::SystemId;
use bevy::prelude::{App, Commands, Event, EventReader, EventWriter, In, IntoSystem, Res, Resource};

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

    fn add_piped_request<R, H>(&mut self, label: impl ScheduleLabel, handler: H) -> &mut Self
    where
        R: BevyRequest,
        H: IntoSystem<In<R>, (), ()> + 'static;
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

    fn add_piped_request<R, H>(&mut self, label: impl ScheduleLabel, handler: H) -> &mut Self
    where
        R: BevyRequest,
        H: IntoSystem<In<R>, (), ()> + 'static
    {
        self.add_event::<R>();
        self.add_event::<R::BevyResponse>();
        let handler_id = self.register_system(handler);
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

#[cfg(test)]
mod tests {
    use bevy::app::{App, PostUpdate, Update};
    use bevy::prelude::{Entity, Event, EventReader, In, IntoSystem, Query};
    use crate::{BevyRequest, BevyRequests};

    #[derive(Event, Clone)]
    struct Request {
        to: String,
    }

    #[derive(Event)]
    struct Response {
        message: String,
    }

    impl BevyRequest for Request {
        type BevyResponse = Response;
    }

    fn handle_request(In(request): In<Request>, _query: Query<Entity>) -> Response {
        println!("Handling request ...");
        let Request { to } = request;
        Response {
            message: format!("Hello, {to}!",),
        }
    }

    fn read_response_piped(In(Response { message }): In<Response>) {
        println!("Trying to read response.");
        println!("{}", message)
    }

    fn read_response(mut reader: EventReader<Response>) {
        println!("Trying to read response.");
        for Response { message } in reader.read() {
            println!("{}", message)
        }
    }

    #[test]
    fn request_response() {
        let mut app = App::new();
        app.add_event::<Request>()
            .add_event::<Response>()
            .add_piped_request(Update, IntoSystem::into_system(handle_request.pipe(read_response_piped)));
        app.world_mut().commands().send_event(Request {
            to: "Vicky".to_string(),
        });
        app.world_mut().commands().send_event(Request {
            to: "Luna".to_string(),
        });
        app.update();
    }

    #[test]
    fn piped_request() {
        let mut app = App::new();
        app.add_event::<Request>()
            .add_event::<Response>()
            .add_request(Update, IntoSystem::into_system(handle_request))
            .add_systems(PostUpdate, read_response);
        app.world_mut().commands().send_event(Request {
            to: "Vicky".to_string(),
        });
        app.world_mut().commands().send_event(Request {
            to: "Luna".to_string(),
        });
        app.update();
    }
}
