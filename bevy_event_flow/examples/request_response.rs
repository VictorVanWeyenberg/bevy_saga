use bevy::app::{App, PostUpdate, Update};
use bevy::prelude::{Event, EventReader, SystemInput};
use bevy_event_flow::EventFlow;
use bevy_event_flow_macros::Request;

#[derive(Request, Event, Clone)]
#[response(Response)]
struct Request {
    to: String,
}

#[derive(Event)]
struct Response {
    message: String,
}

fn handle_request(Request { to }: Request) -> Response {
    Response {
        message: format!("Hello, {to}!",),
    }
}

fn read_response(mut reader: EventReader<Response>) {
    for Response { message } in reader.read() {
        println!("{}", message)
    }
}

fn main() {
    let mut app = App::new();
    app.add_event_flow(Update, handle_request)
        .add_systems(PostUpdate, read_response);
    app.world_mut().commands().send_event(Request {
        to: "Victor".to_string(),
    });
    app.world_mut().commands().send_event(Request {
        to: "Luna".to_string(),
    });
    app.update();
}