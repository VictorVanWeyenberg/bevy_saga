use bevy::prelude::{App, Event, ResMut, Resource, Update};
use bevy_event_flow::EventFlow;
use bevy_event_flow_macros::Request;

#[derive(Default, Resource)]
struct EventsConsumed(usize);

#[derive(Clone, Event, Request)]
struct Producer;

#[derive(Clone, Event, Request)]
struct Produced1;

#[derive(Clone, Event, Request)]
struct Produced2;

#[derive(Clone, Event, Request)]
struct Produced3;

#[derive(Clone, Event, Request)]
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

fn main() {
    let mut app = App::new();
    app.init_resource::<EventsConsumed>();
    app.add_event_flow(Update, producer1)
        .add_event_flow(Update, producer2)
        .add_event_flow(Update, producer3)
        .add_event_flow_after::<Producer, _, _, _>(Update, process1)
        .add_event_flow_after::<Producer, _, _, _>(Update, process2)
        .add_event_flow_after::<Producer, _, _, _>(Update, process3)
        .add_event_handler_after::<Produced1, _, _>(Update, consumer)
        .add_event_handler_after::<Produced2, _, _>(Update, consumer)
        .add_event_handler_after::<Produced3, _, _>(Update, consumer);

    // use bevy_mod_debugdump::{schedule_graph, schedule_graph_dot};
    // let dot = schedule_graph_dot(&mut app, Update, &schedule_graph::Settings::default());
    // std::fs::write("schedule_graph.dot", dot).unwrap();
    // > dot -Tpng schedule_graph.dot -o schedule_graph.png

    app.world_mut().send_event(Producer);
    app.update();
    assert_eq!(3, app.world().resource::<EventsConsumed>().0);
}
