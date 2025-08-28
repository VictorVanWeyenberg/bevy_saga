use bevy::app::{App, Update};
use bevy::prelude::{Component, Entity, Query, ResMut, Resource};
use bevy_saga_impl::RegisterSaga;
use bevy_saga_macros::saga_event;

#[derive(Default, Resource)]
struct Counter(u8);

#[saga_event]
struct Input {
    entity: Entity,
}

#[saga_event]
enum Intermediary {
    Ok { entity: Entity },
    Err { message: String },
}

#[saga_event]
enum Output {
    Ok { entity: Entity },
    Err { message: String },
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Health(usize);

fn handle_input(
    Input { entity }: Input,
    query: Query<&Name>,
    mut counter: ResMut<Counter>,
) -> Intermediary {
    counter.0 += 1;
    if let Ok(Name(name)) = query.get(entity) {
        println!("Oh no, {name} is hit!");
        Intermediary::Ok { entity }
    } else {
        Intermediary::Err {
            message: "System handle_input query could not be fulfilled.".to_string(),
        }
    }
}

fn handle_intermediary(
    response: Intermediary,
    mut query: Query<(&Name, &mut Health)>,
    mut counter: ResMut<Counter>,
) -> Output {
    counter.0 += 1;
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

fn read_output(output: Output, query: Query<(&Name, &Health)>, mut counter: ResMut<Counter>) {
    counter.0 += 1;
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

#[test]
fn main() {
    let mut app = App::new();
    app.init_resource::<Counter>();
    app.add_saga(Update, (handle_input, handle_intermediary, read_output));
    let player = app
        .world_mut()
        .spawn((Name("Player".to_string()), Health(10)))
        .id();
    app.world_mut().send_event(Input { entity: player });
    app.update();
    assert_eq!(app.world_mut().resource::<Counter>().0, 3);
}
