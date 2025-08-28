use bevy::prelude::{App, Update, ResMut, Resource};
use bevy_saga_impl::RegisterSaga;
use bevy_saga_macros::saga_event;

#[derive(Default, Resource)]
struct Counter(u8);

#[saga_event]
struct Request {
    to: String,
}

#[saga_event]
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

#[test]
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
