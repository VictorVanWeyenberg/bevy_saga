use bevy::prelude::{App, ResMut, Resource, Update};
use bevy_saga::SagaRegistry;
use bevy_saga::saga_event;

#[derive(Default, Resource)]
struct EventsConsumed(usize);

#[saga_event]
struct Producer;

#[saga_event]
struct Produced1;

#[saga_event]
struct Produced2;

#[saga_event]
struct Produced3;

#[saga_event]
struct Consumed;

fn producer1(_: Producer) -> Produced1 {
    Produced1
}

fn producer2(_: Producer) -> Produced2 {
    Produced2
}

fn producer3(_: Producer) -> Produced3 {
    Produced3
}

fn process1(_: Produced1) -> Consumed {
    Consumed
}

fn process2(_: Produced2) -> Consumed {
    Consumed
}

fn process3(_: Produced3) -> Consumed {
    Consumed
}

fn consumer(_: Consumed, mut number_consumed: ResMut<EventsConsumed>) {
    number_consumed.0 += 1;
    println!("{} events consumed", number_consumed.0)
}

#[test]
fn main() {
    let mut app = App::new();
    app.init_resource::<EventsConsumed>();
    app.add_saga(Update, (producer1, process1, consumer));
    app.add_saga(Update, (producer2, process2, consumer));
    app.add_saga(Update, (producer3, process3, consumer));

    // use bevy_mod_debugdump::{schedule_graph, schedule_graph_dot};
    // let dot = schedule_graph_dot(&mut app, Update, &schedule_graph::Settings::default());
    // std::fs::write("schedule_graph.dot", dot).unwrap();
    // > dot -Tpng schedule_graph.dot -o schedule_graph.png

    app.world_mut().send_event(Producer);
    app.update();
    assert_ne!(3, app.world().resource::<EventsConsumed>().0);
}
