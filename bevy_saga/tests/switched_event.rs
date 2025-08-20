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

// Builder implementations

impl<Source, MarkerSource> OneStage<Source, MarkerSource> for Source
where
    Source: bevy::prelude::SystemParamFunction<MarkerSource, Out = RoutedEvent>,
    Source::In: bevy_saga::SagaEvent,
    MarkerSource: 'static,
{
    fn one<OneSaga, MarkerOneSaga>(
        self,
        one_saga: OneSaga,
    ) -> impl TwoStage<Source, MarkerSource, OneSaga>
    where
        OneSaga: bevy_saga::Saga<MarkerOneSaga, In = One>,
    {
        let source = self;
        OneStageBuilder { source, one_saga }
    }
}

impl<Source, OneSaga, MarkerSource> TwoStage<Source, MarkerSource, OneSaga>
    for OneStageBuilder<Source, OneSaga>
where
    Source: bevy::prelude::SystemParamFunction<MarkerSource, Out = RoutedEvent>,
    Source::In: bevy_saga::SagaEvent,
    MarkerSource: 'static,
{
    fn two<TwoSaga, MarkerTwoSaga>(
        self,
        two_saga: TwoSaga,
    ) -> impl ThreeStage<Source, MarkerSource, OneSaga, TwoSaga>
    where
        TwoSaga: bevy_saga::Saga<MarkerTwoSaga, In = Two>,
    {
        let OneStageBuilder { source, one_saga } = self;
        TwoStageBuilder {
            source,
            one_saga,
            two_saga,
        }
    }
}

impl<Source, OneSaga, TwoSaga, MarkerSource> ThreeStage<Source, MarkerSource, OneSaga, TwoSaga>
    for TwoStageBuilder<Source, OneSaga, TwoSaga>
where
    Source: bevy::prelude::SystemParamFunction<MarkerSource, Out = RoutedEvent>,
    Source::In: bevy_saga::SagaEvent,
{
    fn three<ThreeSaga, MarkerThreeSaga>(
        self,
        three_saga: ThreeSaga,
    ) -> RoutedEventRouter<Source, OneSaga, TwoSaga, ThreeSaga>
    where
        ThreeSaga: bevy_saga::Saga<MarkerThreeSaga, In = Three>,
    {
        let TwoStageBuilder {
            source,
            one_saga,
            two_saga,
        } = self;
        RoutedEventRouter::new(source, one_saga, two_saga, three_saga)
    }
}

// App Plugin

trait RoutedEventPlugin {
    fn add_routed_event_handler<R, M>(
        &mut self,
        handler: impl bevy::prelude::IntoSystem<R, RoutedEvent, M> + 'static,
    ) -> bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::ScheduleSystem>
    where
        R: bevy_saga::SagaEvent;
}

impl RoutedEventPlugin for bevy::prelude::App {
    fn add_routed_event_handler<R, M>(
        &mut self,
        handler: impl bevy::prelude::IntoSystem<R, RoutedEvent, M> + 'static,
    ) -> bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::ScheduleSystem>
    where
        R: bevy_saga::SagaEvent,
    {
        self.add_event::<R>();
        self.init_resource::<bevy_saga::EventProcessors<R>>();
        let id = self.register_system(handler.pipe(send_routed_event_response));
        self.world_mut()
            .resource_mut::<bevy_saga::EventProcessors<R>>()
            .push(id);
        bevy::prelude::IntoScheduleConfigs::into_configs(bevy_saga::process_event::<R>)
    }
}

fn send_routed_event_response(
    bevy::prelude::In(routed_event): bevy::prelude::In<RoutedEvent>,
    mut one_writer: bevy::prelude::EventWriter<One>,
    mut two_writer: bevy::prelude::EventWriter<Two>,
    mut three_writer: bevy::prelude::EventWriter<Three>,
) {
    match routed_event {
        RoutedEvent::One(one) => {
            one_writer.write(one);
        }
        RoutedEvent::Two(two) => {
            two_writer.write(two);
        }
        RoutedEvent::Three(three) => {
            three_writer.write(three);
        }
    }
}

// test

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
