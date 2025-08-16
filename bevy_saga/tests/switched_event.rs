use bevy_saga::{EventHandler, Saga, SagaEvent};
use bevy_saga_macros::saga_event;

enum RoutedEvent {
    One(One),
    Two(Two),
    Three(Three),
}

#[saga_event]
struct One;

#[saga_event]
struct Two;

#[saga_event]
struct Three;

// Traits

trait OneStage<Source, MarkerSource> {
    fn one<OneSaga, MarkerOneSaga>(
        self,
        one_sage: OneSaga,
    ) -> impl TwoStage<Source, OneSaga, MarkerSource>
    where
        OneSaga: Saga<MarkerOneSaga, In = One>;
}

trait TwoStage<Source, OneSaga, MarkerSource> {
    fn two<TwoSaga, MarkerTwoSaga>(
        self,
        two_saga: TwoSaga,
    ) -> impl ThreeStage<Source, OneSaga, TwoSaga, MarkerSource>
    where
        TwoSaga: Saga<MarkerTwoSaga, In = Two>;
}

trait ThreeStage<Source, OneSaga, TwoSaga, MarkerSource> {
    fn three<ThreeSaga, MarkerThreeSaga>(
        self,
        three_saga: ThreeSaga,
    ) -> RoutedEventHandler<Source, OneSaga, TwoSaga, ThreeSaga>
    where
        ThreeSaga: Saga<MarkerThreeSaga, In = Three>;
}

// Handler

struct RoutedEventHandler<Source, OneSaga, TwoSaga, ThreeSaga> {
    source: Source,
    one: OneSaga,
    two: TwoSaga,
    three: ThreeSaga,
}

impl<Source, OneSaga, TwoSaga, ThreeSaga>
    RoutedEventHandler<Source, OneSaga, TwoSaga, ThreeSaga>
{
    pub fn new(source: Source, one: OneSaga, two: TwoSaga, three: ThreeSaga) -> Self {
        Self {
            source,
            one,
            two,
            three,
        }
    }
}

impl<
    Source,
    OneSaga,
    TwoSaga,
    ThreeSaga,
    MarkerSource,
    MarkerOneSaga,
    MarkerTwoSaga,
    MarkerThreeSaga,
> EventHandler<(MarkerSource, MarkerOneSaga, MarkerTwoSaga, MarkerThreeSaga)>
    for RoutedEventHandler<Source, OneSaga, TwoSaga, ThreeSaga>
where
    Source: bevy::prelude::SystemParamFunction<MarkerSource, Out =RoutedEvent>,
    Source::In: SagaEvent,
    OneSaga: Saga<MarkerOneSaga, In = One>,
    TwoSaga: Saga<MarkerTwoSaga, In = Two>,
    ThreeSaga: Saga<MarkerThreeSaga, In = Three>,
    MarkerSource: 'static,
{
    type In = Source::In;

    fn register_handler(
        self,
        app: &mut bevy::prelude::App,
    ) -> bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::ScheduleSystem> {
        let RoutedEventHandler {
            source,
            one,
            two,
            three,
        } = self;
        bevy::prelude::IntoScheduleConfigs::chain((
            app.add_routed_event_handler(source),
            (one.register(app), two.register(app), three.register(app)),
        ))
    }
}

// Builder

struct OneStageBuilder<Source, OneSaga> {
    source: Source,
    one: OneSaga,
}

impl<Source, OneSaga> OneStageBuilder<Source, OneSaga> {
    fn new(source: Source, one: OneSaga) -> Self {
        OneStageBuilder { source, one }
    }
}

struct TwoStageBuilder<Source, OneSaga, TwoSaga> {
    source: Source,
    one: OneSaga,
    two: TwoSaga,
}

impl<Source, OneSaga, TwoSaga> TwoStageBuilder<Source, OneSaga, TwoSaga> {
    fn new(
        OneStageBuilder { source, one, .. }: OneStageBuilder<Source, OneSaga>,
        two: TwoSaga,
    ) -> Self {
        TwoStageBuilder { source, one, two }
    }
}

// Builder implementations

impl<Source, MarkerSource> OneStage<Source, MarkerSource> for Source
where
    Source: bevy::prelude::SystemParamFunction<MarkerSource, Out =RoutedEvent>,
    Source::In: SagaEvent,
    MarkerSource: 'static,
{
    fn one<OneSaga, MarkerOneSaga>(
        self,
        one_sage: OneSaga,
    ) -> impl TwoStage<Source, OneSaga, MarkerSource>
    where
        OneSaga: Saga<MarkerOneSaga, In = One>,
    {
        OneStageBuilder::new(self, one_sage)
    }
}

impl<Source, OneSaga, MarkerSource> TwoStage<Source, OneSaga, MarkerSource>
    for OneStageBuilder<Source, OneSaga>
where
    Source: bevy::prelude::SystemParamFunction<MarkerSource, Out =RoutedEvent>,
    Source::In: SagaEvent,
    MarkerSource: 'static,
{
    fn two<TwoSaga, MarkerTwoSaga>(
        self,
        two_saga: TwoSaga,
    ) -> impl ThreeStage<Source, OneSaga, TwoSaga, MarkerSource>
    where
        TwoSaga: Saga<MarkerTwoSaga, In = Two>,
    {
        TwoStageBuilder::new(self, two_saga)
    }
}

impl<Source, OneSaga, TwoSaga, MarkerSource> ThreeStage<Source, OneSaga, TwoSaga, MarkerSource>
    for TwoStageBuilder<Source, OneSaga, TwoSaga>
where
    Source: bevy::prelude::SystemParamFunction<MarkerSource, Out =RoutedEvent>,
    Source::In: SagaEvent,
{
    fn three<ThreeSaga, MarkerThreeSaga>(
        self,
        three_saga: ThreeSaga,
    ) -> RoutedEventHandler<Source, OneSaga, TwoSaga, ThreeSaga>
    where
        ThreeSaga: Saga<MarkerThreeSaga, In = Three>,
    {
        let TwoStageBuilder { source, one, two } = self;
        RoutedEventHandler::new(source, one, two, three_saga)
    }
}

// App Plugin

trait RoutedEventPlugin {
    fn add_routed_event_handler<R, M>(
        &mut self,
        handler: impl bevy::prelude::IntoSystem<R, RoutedEvent, M> + 'static,
    ) -> bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::ScheduleSystem>
    where
        R: SagaEvent;
}

impl RoutedEventPlugin for bevy::prelude::App {
    fn add_routed_event_handler<R, M>(
        &mut self,
        handler: impl bevy::prelude::IntoSystem<R, RoutedEvent, M> + 'static,
    ) -> bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::ScheduleSystem>
    where
        R: SagaEvent,
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
use bevy::prelude::{App, Update};
    use bevy_saga::RegisterSaga;
    use bevy_saga_macros::saga_event;
    use crate::{One, OneStage, RoutedEvent, Three, Two};

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
