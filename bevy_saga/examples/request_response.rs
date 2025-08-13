use bevy::app::{App, Update};
use bevy::prelude::{Event, ResMut, Resource};
use bevy_saga::RegisterSaga;
use bevy_saga_macros::SagaEvent;

#[derive(Default, Resource)]
struct Counter(u8);

#[derive(Clone, Event, SagaEvent)]
struct Request {
    to: String,
}

#[derive(SagaEvent, Event, Clone)]
struct Response {
    message: String,
}

fn handle_request(Request { to }: Request, mut counter: ResMut<Counter>) -> Response {
    counter.0 += 1;
    Response {
        message: format!("Hello, {to}!",),
    }
}

fn read_response(Response { message }: Response, mut counter: ResMut<Counter>) {
    counter.0 += 2;
    println!("{}", message)
}

fn main() {
    let mut app = App::new();
    app.init_resource::<Counter>();
    app.add_saga(Update, (handle_request, read_response));
    app.world_mut().send_event(Request {
        to: "Player".to_string(),
    });
    app.update();
    assert_eq!(app.world_mut().resource::<Counter>().0, 3)
}