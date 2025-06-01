use bevy::prelude::{App, Event, EventReader, Local, PostUpdate, Update};
use bevy_event_flow::EventFlow;
use bevy_event_flow_macros::Request;

#[derive(Clone, Event, Request)]
struct Producer;

#[derive(Clone, Event, Request)]
struct Produced1;

#[derive(Clone, Event, Request)]
struct Produced2;

#[derive(Clone, Event, Request)]
struct Produced3;

#[derive(Event)]
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

fn consumer(mut reader: EventReader<Consumed>, mut number_consumed: Local<usize>) {
    for _ in reader.read() {
        *number_consumed += 1;
        println!("{} events consumed", *number_consumed)
    }
}

fn main() {
    let mut app = App::new();
    app.add_event_flow(Update, producer1)
        .add_event_flow(Update, producer2)
        .add_event_flow(Update, producer3)
        .add_event_flow_after::<Producer, _, _, _>(Update, process1)
        .add_event_flow_after::<Producer, _, _, _>(Update, process2)
        .add_event_flow_after::<Producer, _, _, _>(Update, process3)
        .add_systems(PostUpdate, consumer);
    app.world_mut().send_event(Producer);
    app.update();
}

