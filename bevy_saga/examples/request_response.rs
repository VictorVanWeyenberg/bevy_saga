use bevy::app::{App, Update};
use bevy::prelude::Event;
use bevy_saga::RegisterSaga;
use bevy_saga_macros::SagaEvent;

#[derive(Clone, Event, SagaEvent)]
struct Request {
    to: String,
}

#[derive(SagaEvent, Event, Clone)]
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
    app.add_saga(Update, (handle_request, read_response));
    app.world_mut().commands().send_event(Request {
        to: "Victor".to_string(),
    });
    app.world_mut().commands().send_event(Request {
        to: "Luna".to_string(),
    });
    app.update();
}