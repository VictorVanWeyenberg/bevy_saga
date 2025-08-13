use bevy::app::{App, Update};
use bevy::prelude::{Event, ResMut, Resource};
use bevy_saga::ErrStage;
use bevy_saga::{OkStage, RegisterSaga, Saga};
use bevy_saga_macros::SagaEvent;

#[derive(Default, Resource)]
struct Counter(u8);

#[derive(Clone)]
enum OkOrErr {
    Ok,
    Err,
}

#[derive(SagaEvent, Event, Clone)]
struct Input(OkOrErr);

#[derive(SagaEvent, Event, Clone)]
struct OkPath(u8);

#[derive(SagaEvent, Event, Clone)]
struct ErrPath(u8);

fn result_processor(Input(input): Input) -> Result<OkPath, ErrPath> {
    match input {
        OkOrErr::Ok => Ok(OkPath(1)),
        OkOrErr::Err => Err(ErrPath(2)),
    }
}

fn ok_path(OkPath(value): OkPath, mut counter: ResMut<Counter>) {
    counter.0 = value
}

fn err_path(ErrPath(value): ErrPath, mut counter: ResMut<Counter>) {
    counter.0 = value
}

fn test<M>(saga: impl Saga<M>, input: OkOrErr) -> u8 {
    let mut app = App::new();
    app.init_resource::<Counter>();
    app.add_saga(Update, saga);
    app.world_mut().send_event(Input(input));
    app.update();
    app.world().resource::<Counter>().0
}

#[test]
fn test_ok() {
    assert_eq!(
        1,
        test(result_processor.ok(ok_path).err(err_path), OkOrErr::Ok)
    );
}

#[test]
fn test_err() {
    assert_eq!(
        2,
        test(result_processor.ok(ok_path).err(err_path), OkOrErr::Err)
    );
}
