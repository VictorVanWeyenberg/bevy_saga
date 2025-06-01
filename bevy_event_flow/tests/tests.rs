use bevy::app::{App, PostUpdate, Update};
use bevy::prelude::{Component, Event, EventReader, In, IntoScheduleConfigs, Query};
use bevy_event_flow::EventFlow;

#[derive(Event, Clone)]
struct Request {
    to: String,
}

#[derive(Event, Clone)]
struct Response {
    message: String,
}

impl bevy_event_flow::Request for Request {
    type Response = Response;
}

#[derive(Event)]
#[allow(dead_code)]
struct ResponseBack {
    message: String,
}

impl bevy_event_flow::Request for Response {
    type Response = ResponseBack;
}

#[derive(Component)]
#[allow(dead_code)]
struct Health(usize);

fn handle_request(In(Request { to }): In<Request>, _query: Query<&mut Health>) -> Response {
    println!("Handling request ...");
    Response {
        message: format!("Hello, {to}!",),
    }
}

fn handle_response(In(_response): In<Response>) -> ResponseBack {
    println!("Handling request ...");
    ResponseBack {
        message: "Hello, back!".to_string(),
    }
}

fn read_response(mut reader: EventReader<Response>) {
    println!("Trying to read response.");
    for Response { message } in reader.read() {
        println!("{}", message)
    }
}

fn read_response_back(mut reader: EventReader<ResponseBack>) {
    println!("Trying to read response.");
    for ResponseBack { message } in reader.read() {
        println!("{}", message)
    }
}

#[test]
fn request() {
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

#[test]
fn request_after() {
    let mut app = App::new();
    app.add_event_flow(Update, handle_request)
        .add_event_flow_after::<Response, Request, _>(Update, handle_response)
        .add_systems(PostUpdate, (read_response, read_response_back.after(read_response)));
    app.world_mut().commands().send_event(Request {
        to: "Vicky".to_string(),
    });
    app.world_mut().commands().send_event(Request {
        to: "Luna".to_string(),
    });
    app.update();
}