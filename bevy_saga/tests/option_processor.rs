use bevy::prelude::{App, ResMut, Resource, Update};
use bevy_saga::{SagaRegistry, prelude::Saga};
use bevy_saga::saga_event;

#[derive(Default, Resource)]
struct Counter(u8);

#[saga_event]
struct Input;

#[saga_event]
struct Output(u8);

fn success(_: Input) -> Option<Output> {
    Some(Output(4))
}

fn failure(_: Input) -> Option<Output> {
    None
}

fn then(Output(output): Output, mut counter: ResMut<Counter>) {
    counter.0 = output;
}

fn test<M>(saga: impl Saga<M>) -> u8 {
    let mut app = App::new();
    app.init_resource::<Counter>();
    app.add_saga(Update, saga);
    app.world_mut().send_event(Input);
    app.update();
    app.world().resource::<Counter>().0
}

#[test]
fn test_success() {
    assert_eq!(test((success, then)), 4);
}

#[test]
fn test_failure() {
    assert_eq!(test((failure, then)), 0);
}
