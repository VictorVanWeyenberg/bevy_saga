use bevy::app::{App, Update};
use bevy::prelude::{Component, Entity, Event, Query};
use bevy_event_flow::RegisterEventFlow;
use bevy_event_flow_macros::Request;

#[derive(Request, Event, Clone)]
struct Input {
    entity: Entity,
}

#[derive(Request, Event, Clone)]
enum Intermediary {
    Ok { entity: Entity },
    Err { message: String },
}

#[derive(Request, Event, Clone)]
enum Output {
    Ok { entity: Entity },
    Err { message: String },
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Health(usize);

fn handle_input(Input { entity }: Input, query: Query<&Name>) -> Intermediary {
    if let Ok(Name(name)) = query.get(entity) {
        println!("Oh no, {name} is hit!");
        Intermediary::Ok { entity }
    } else {
        Intermediary::Err {
            message: "System handle_input query could not be fulfilled.".to_string(),
        }
    }
}

fn handle_intermediary(response: Intermediary, mut query: Query<(&Name, &mut Health)>) -> Output {
    match response {
        Intermediary::Ok { entity } => {
            if let Ok((Name(name), mut health)) = query.get_mut(entity) {
                health.0 -= 1;
                println!("{name} took 1 damage!");
                Output::Ok { entity }
            } else {
                Output::Err {
                    message: "System handle_intermediary query could not be fulfilled.".to_string(),
                }
            }
        }
        Intermediary::Err { message } => Output::Err { message },
    }
}

fn read_output(output: Output, query: Query<(&Name, &Health)>) {
    match output {
        Output::Ok { entity } => {
            if let Ok((Name(name), Health(health))) = query.get(entity) {
                println!("Player {name} now has {health} health.")
            } else {
                println!("System read_output query could not be fulfilled.")
            }
        }
        Output::Err { message } => println!("{}", message),
    }
}

fn main() {
    let mut app = App::new();
    app.add_event_processor_flow(Update, (handle_input, handle_intermediary, read_output));
    let victor = app.world_mut().spawn((Name("Victor".to_string()), Health(10))).id();
    let luna = app.world_mut().spawn((Name("Luna".to_string()), Health(10))).id();
    app.world_mut().commands().send_event(Input {
        entity: victor
    });
    app.world_mut().commands().send_event(Input {
        entity: luna
    });
    app.update();
}
