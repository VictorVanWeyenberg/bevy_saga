use bevy::prelude::{App, ResMut, Resource, Update};
use bevy_saga_impl::RegisterSaga;
use bevy_saga_macros::saga_event;

#[test]
fn big_big_saga_for_big_decisions() {
    let mut app = App::new();
    app.init_resource::<Counter>();
    app.add_saga(Update, (f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12, f13, f14, f15, f16));
    app.world_mut().send_event(Input);
    app.update();
    assert_eq!(16, app.world().resource::<Counter>().0);
}

#[derive(Default, Resource)]
struct Counter(u8);

#[saga_event]
struct Input;

#[saga_event]
struct Intermediate1;

#[saga_event]
struct Intermediate2;

#[saga_event]
struct Intermediate3;

#[saga_event]
struct Intermediate4;

#[saga_event]
struct Intermediate5;

#[saga_event]
struct Intermediate6;

#[saga_event]
struct Intermediate7;

#[saga_event]
struct Intermediate8;

#[saga_event]
struct Intermediate9;

#[saga_event]
struct Intermediate10;

#[saga_event]
struct Intermediate11;

#[saga_event]
struct Intermediate12;

#[saga_event]
struct Intermediate13;

#[saga_event]
struct Intermediate14;

#[saga_event]
struct Output;

fn f1(_: Input, mut counter: ResMut<Counter>) -> Intermediate1 {
    counter.0 += 1;
    Intermediate1
}

fn f2(_: Intermediate1, mut counter: ResMut<Counter>) -> Intermediate2 {
    counter.0 += 1;
    Intermediate2
}

fn f3(_: Intermediate2, mut counter: ResMut<Counter>) -> Intermediate3 {
    counter.0 += 1;
    Intermediate3
}

fn f4(_: Intermediate3, mut counter: ResMut<Counter>) -> Intermediate4 {
    counter.0 += 1;
    Intermediate4
}

fn f5(_: Intermediate4, mut counter: ResMut<Counter>) -> Intermediate5 {
    counter.0 += 1;
    Intermediate5
}

fn f6(_: Intermediate5, mut counter: ResMut<Counter>) -> Intermediate6 {
    counter.0 += 1;
    Intermediate6
}

fn f7(_: Intermediate6, mut counter: ResMut<Counter>) -> Intermediate7 {
    counter.0 += 1;
    Intermediate7
}

fn f8(_: Intermediate7, mut counter: ResMut<Counter>) -> Intermediate8 {
    counter.0 += 1;
    Intermediate8
}

fn f9(_: Intermediate8, mut counter: ResMut<Counter>) -> Intermediate9 {
    counter.0 += 1;
    Intermediate9
}

fn f10(_: Intermediate9, mut counter: ResMut<Counter>) -> Intermediate10 {
    counter.0 += 1;
    Intermediate10
}

fn f11(_: Intermediate10, mut counter: ResMut<Counter>) -> Intermediate11 {
    counter.0 += 1;
    Intermediate11
}

fn f12(_: Intermediate11, mut counter: ResMut<Counter>) -> Intermediate12 {
    counter.0 += 1;
    Intermediate12
}

fn f13(_: Intermediate12, mut counter: ResMut<Counter>) -> Intermediate13 {
    counter.0 += 1;
    Intermediate13
}

fn f14(_: Intermediate13, mut counter: ResMut<Counter>) -> Intermediate14 {
    counter.0 += 1;
    Intermediate14
}

fn f15(_: Intermediate14, mut counter: ResMut<Counter>) -> Output {
    counter.0 += 1;
    Output
}

fn f16(_: Output, mut counter: ResMut<Counter>) {
    counter.0 += 1;
}