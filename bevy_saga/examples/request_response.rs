use bevy::app::{App, Update};
use bevy::prelude::Event;
use bevy_saga::RegisterEventSaga;
use bevy_saga_macros::SagaEvent;

#[derive(Clone, Event, SagaEvent)]
struct SagaEvent {
    to: String,
}

#[derive(SagaEvent, Event, Clone)]
struct Response {
    message: String,
}

fn handle_request(SagaEvent { to }: SagaEvent) -> Response {
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
    app.world_mut().commands().send_event(SagaEvent {
        to: "Victor".to_string(),
    });
    app.world_mut().commands().send_event(SagaEvent {
        to: "Luna".to_string(),
    });
    app.update();
}