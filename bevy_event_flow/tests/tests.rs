use bevy::app::{App, PostUpdate, Update};
use bevy::prelude::{Component, Event, EventReader, In, Query};
use bevy_event_flow::EventFlow;

#[derive(Event, Clone)]
struct Request {
    to: String,
}

#[derive(Event)]
struct Response {
    message: String,
}

impl bevy_event_flow::Request for Request {
    type Response = Response;
}

#[derive(Component)]
#[allow(dead_code)]
struct Health(usize);

#[derive(Component)]
#[allow(dead_code)]
struct Name(String);

fn handle_request(In(request): In<Request>, _query: Query<&mut Health>, _query2: Query<&Name>) -> Response {
    println!("Handling request ...");
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

#[test]
fn request() {
    let mut app = App::new();
    app.add_request(Update, handle_request)
        .add_systems(PostUpdate, read_response);
    app.world_mut().commands().send_event(Request {
        to: "Vicky".to_string(),
    });
    app.world_mut().commands().send_event(Request {
        to: "Luna".to_string(),
    });
    app.update();
}