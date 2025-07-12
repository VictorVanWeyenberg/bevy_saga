use bevy::ecs::system::SystemParam;
use bevy::prelude::SystemInput;
use crate::node::Link;

pub trait EventProcessor<Marker>: Sized {

    type Input;
    type Intermediary;

    fn then<Processor, Output, NextMarker>(self, next: Processor) -> Link<Self, Processor, Marker, NextMarker>
    where
        Processor: EventProcessor<NextMarker, Input = Self::Intermediary, Intermediary = Output>;

}

// region Function implementations.

impl<Input, Intermediary, Func> EventProcessor<fn(Input) -> Intermediary> for Func
where
    Func: Send + Sync + 'static,
    for <'a> &'a mut Func: FnMut(Input) -> Intermediary + FnMut(<Input as SystemInput>::Param<'_>) -> Intermediary,
    Input: SystemInput + 'static,
    Intermediary: 'static,
{
    type Input = Input;
    type Intermediary = Intermediary;

    fn then<Processor, Output, NextMarker>(self, next: Processor) -> Link<Self, Processor, fn(Input) -> Intermediary, NextMarker>
    where
        Processor: EventProcessor<NextMarker, Input = Self::Intermediary>,
    {
        Link::new(self, next)
    }
}

impl<Input, Intermediary, Func, P1> EventProcessor<fn(Input, P1) -> Intermediary> for Func
where
    Func: Send + Sync + 'static,
    for <'a> &'a mut Func: FnMut(Input, P1) -> Intermediary + FnMut(<Input as SystemInput>::Param<'_>, <P1 as SystemParam>::Item<'_, '_>) -> Intermediary,
    Input: SystemInput + 'static,
    P1: SystemParam,
    Intermediary: 'static,
{
    type Input = Input;
    type Intermediary = Intermediary;

    fn then<Processor, Output, NextMarker>(self, next: Processor) -> Link<Self, Processor, fn(Input, P1) -> Intermediary, NextMarker>
    where
        Processor: EventProcessor<NextMarker, Input = Self::Intermediary>
    {
        Link::new(self, next)
    }
}

impl<Input, Intermediary, Func, P1, P2> EventProcessor<fn(Input, P1, P2) -> Intermediary> for Func
where
    Func: Send + Sync + 'static,
    for <'a> &'a mut Func: FnMut(Input, P1, P2) -> Intermediary + FnMut(<Input as SystemInput>::Param<'_>, <P1 as SystemParam>::Item<'_, '_>, <P2 as SystemParam>::Item<'_, '_>) -> Intermediary,
    Input: SystemInput + 'static,
    P1: SystemParam,
    P2: SystemParam,
    Intermediary: 'static,
{
    type Input = Input;
    type Intermediary = Intermediary;

    fn then<Processor, Output, NextMarker>(self, next: Processor) -> Link<Self, Processor, fn(Input, P1, P2) -> Intermediary, NextMarker>
    where
        Processor: EventProcessor<NextMarker, Input = Self::Intermediary>
    {
        Link::new(self, next)
    }
}

// endregion

// region Tuple implementations

impl<Input, Intermediary, Marker1, Marker2, E1, E2> EventProcessor<(Marker1, Marker2)> for (E1, E2)
where
    E1: EventProcessor<Marker1, Input = Input, Intermediary = Intermediary>,
    E2: EventProcessor<Marker2, Input = Input, Intermediary = Intermediary>,
{
    type Input = Input;
    type Intermediary = Intermediary;

    fn then<Processor, Output, NextMarker>(self, next: Processor) -> Link<Self, Processor, (Marker1, Marker2), NextMarker>
    where
        Processor: EventProcessor<NextMarker, Input=Self::Intermediary>
    {
        Link::new(self, next)
    }
}

impl<Input, Intermediary, Marker1, Marker2, Marker3, E1, E2, E3> EventProcessor<(Marker1, Marker2, Marker3)> for (E1, E2, E3)
where
    E1: EventProcessor<Marker1, Input = Input, Intermediary = Intermediary>,
    E2: EventProcessor<Marker2, Input = Input, Intermediary = Intermediary>,
    E3: EventProcessor<Marker3, Input = Input, Intermediary = Intermediary>,
{
    type Input = Input;
    type Intermediary = Intermediary;

    fn then<Processor, Output, NextMarker>(self, next: Processor) -> Link<Self, Processor, (Marker1, Marker2, Marker3), NextMarker>
    where
        Processor: EventProcessor<NextMarker, Input=Self::Intermediary>
    {
        Link::new(self, next)
    }
}

//endregion