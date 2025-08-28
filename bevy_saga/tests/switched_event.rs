use bevy::prelude::{App, ResMut, Resource, Update};
use bevy_saga::RegisterSaga;
use bevy_saga::{saga_event, saga_router};

#[derive(Default, Resource)]
struct Identifier(Option<String>);

#[saga_router]
enum Foobar {
    Apple(Foo),
    Banana(Bar),
    Cherry(Baz),
}

#[saga_event]
struct Foo;
#[saga_event]
struct Bar;
#[saga_event]
struct Baz;

#[saga_event]
struct Input(Foobar);

fn pre_route(Input(input): Input) -> Foobar {
    input
}

fn sibling_input_handler(Input(_): Input) {

}

fn foo(_: Foo, mut identifier: ResMut<Identifier>) {
    identifier.0 = Some("apple".to_string())
}

fn bar(_: Bar, mut identifier: ResMut<Identifier>) {
    identifier.0 = Some("banana".to_string())
}

fn sibling_bar_handler(_: Bar) {

}

fn baz(_: Baz, mut identifier: ResMut<Identifier>) {
    identifier.0 = Some("cherry".to_string())
}

fn test(input: Input, expected: &str) {
    let mut app = App::new();
    app.init_resource::<Identifier>();
    app.add_saga(
        Update,
        (pre_route, sibling_input_handler).apple(foo).banana((bar, sibling_bar_handler)).cherry(baz),
    );
    app.world_mut().send_event(input);
    app.update();
    if let Some(identifier) = &app.world().resource::<Identifier>().0 {
        assert_eq!(identifier, expected)
    } else {
        panic!("Identifier was still None.")
    }
}

#[test]
fn switched_event() {
    test(Input(Foobar::Apple(Foo)), "apple");
    test(Input(Foobar::Banana(Bar)), "banana");
    test(Input(Foobar::Cherry(Baz)), "cherry");
}
