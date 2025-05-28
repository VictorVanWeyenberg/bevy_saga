use crate::reqsposte::{BevyRequest, BevyRequests};
use bevy::prelude::{App, Entity, Event, EventReader, In, IntoSystem, IntoSystemConfigs, Query, Update};

mod reqsposte;

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
    let Request { to } = request;
    Response {
        message: format!("Hello, {to}!",),
    }
}

fn read_response(mut reader: EventReader<Response>) {
    println!("Trying to read response.");
    for Response { message } in reader.read() {
        println!("{}", message)
    }
}

fn main() {
    let mut app = App::new();
    app.add_event::<Request>()
        .add_event::<Response>()
        .add_request(Update, IntoSystem::into_system(handle_request))
        .add_systems(Update, read_response.after(handle_request));
    app.world_mut().commands().send_event(Request {
        to: "Vicky".to_string(),
    });
    app.world_mut().commands().send_event(Request {
        to: "Luna".to_string(),
    });
    app.world_mut().commands().send_event(Request {
        to: "Fluffy".to_string(),
    });
    app.world_mut().commands().send_event(Request {
        to: "Drunky".to_string(),
    });
    app.update();
    app.update();
}
