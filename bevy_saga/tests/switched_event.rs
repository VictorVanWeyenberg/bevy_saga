#[bevy_saga_macros::saga_router]
enum RoutedEvent {
    One(One),
    Two(Two),
    Three(Three),
}

#[bevy_saga_macros::saga_event]
struct One;

#[bevy_saga_macros::saga_event]
struct Two;

#[bevy_saga_macros::saga_event]
struct Three;

#[cfg(test)]
mod tests {
    use crate::ThreeStage;
    use crate::TwoStage;
    use crate::{One, OneStage, RoutedEvent, Three, Two};
    use bevy::prelude::{App, Update};
    use bevy_saga::RegisterSaga;
    use bevy_saga_macros::saga_event;

    #[saga_event]
    struct Input;

    fn pre_route(input: Input) -> RoutedEvent {
        RoutedEvent::One(One)
    }
    fn one(one: One) {}
    fn two(two: Two) {}
    fn three(three: Three) {}
    #[test]
    fn does_it_work() {
        let mut app = App::new();
        app.add_saga(Update, pre_route.one(one).two(two).three(three));
    }
}
