use bevy::prelude::{App, ResMut, Resource, Update};
use bevy_saga::SagaRegistry;
use bevy_saga::saga_event;

#[derive(Default, Resource)]
struct Counter(u8);

#[saga_event]
struct Input;

#[saga_event]
struct Output;

fn process_input(_input: Input, mut counter: ResMut<Counter>) -> Output {
    assert!(counter.0 < 2);
    counter.0 += 1;
    Output
}

fn handle_input(_input: Input, mut counter: ResMut<Counter>) {
    assert!(counter.0 < 2);
    counter.0 += 1;
}

fn handle_output(_output: Output, mut counter: ResMut<Counter>) {
    assert_eq!(counter.0, 2);
    counter.0 += 1;
}

#[test]
fn main() {
    let mut app = App::new();
    app.init_resource::<Counter>();
    app.add_saga(Update, ((process_input, handle_input), handle_output));
    app.world_mut().send_event(Input);
    app.update();
    assert_eq!(app.world_mut().resource::<Counter>().0, 3);
}
