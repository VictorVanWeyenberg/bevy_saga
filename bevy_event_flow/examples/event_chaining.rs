use bevy::app::{App, PostUpdate, Update};
use bevy::prelude::{Component, Event, EventReader, IntoScheduleConfigs, Query, SystemInput};
use bevy_event_flow::EventFlow;
use bevy_event_flow_macros::Request;

#[derive(Request, Event, Clone)]
#[response(Response)]
struct Request {
    to: String,
}

#[derive(Request, Event, Clone)]
#[response(ResponseBack)]
struct Response {
    message: String,
}

#[derive(Event)]
#[allow(dead_code)]
struct ResponseBack {
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

fn handle_response(_response: Response) -> ResponseBack {
    println!("Handling response ...");
    ResponseBack {
        message: "Hello, back!".to_string(),
    }
}

fn read_response(mut reader: EventReader<Response>) {
    println!("Handling response back ...");
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

fn main() {
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