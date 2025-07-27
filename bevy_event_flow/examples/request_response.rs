use bevy::app::{App, Update};
use bevy::prelude::Event;
use bevy_event_flow::RegisterEventFlow;
use bevy_event_flow_macros::Request;

#[derive(Clone, Event, Request)]
struct Request {
    to: String,
}

#[derive(Request, Event, Clone)]
struct Response {
    message: String,
}

fn handle_request(Request { to }: Request) -> Response {
    Response {
        message: format!("Hello, {to}!",),
    }
}

fn read_response(Response { message }: Response) {
    println!("{}", message)
}

fn main() {
    let mut app = App::new();
    app.add_event_processor_flow(Update, (handle_request, read_response));
    app.world_mut().commands().send_event(Request {
        to: "Victor".to_string(),
    });
    app.world_mut().commands().send_event(Request {
        to: "Luna".to_string(),
    });
    app.update();
}