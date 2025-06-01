use bevy::app::{App, PostUpdate, Update};
use bevy::prelude::{Component, Event, EventReader, Query, SystemInput};
use bevy_event_flow::EventFlow;
use bevy_event_flow_macros::Request;

#[derive(Request, Event, Clone)]
#[response(Response)]
struct Request {
    to: String,
}

#[derive(Event, Clone)]
struct Response {
    message: String,
}

#[derive(Component)]
#[allow(dead_code)]
struct Health(usize);

fn handle_request(Request { to }: Request, _query: Query<&mut Health>) -> Response {
    println!("Handling request ...");
    Response {
        message: format!("Hello, {to}!",),
    }
}

fn read_response(mut reader: EventReader<Response>) {
    println!("Handling response ...");
    for Response { message } in reader.read() {
        println!("{}", message)
    }
}

fn main() {
    let mut app = App::new();
    app.add_event_flow(Update, handle_request)
        .add_systems(PostUpdate, read_response);
    app.world_mut().commands().send_event(Request {
        to: "Vicky".to_string(),
    });
    app.world_mut().commands().send_event(Request {
        to: "Luna".to_string(),
    });
    app.update();
}