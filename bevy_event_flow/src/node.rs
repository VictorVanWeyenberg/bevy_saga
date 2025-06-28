use bevy::ecs::system::SystemParam;
use bevy::prelude::SystemInput;

trait EventProcessor<Marker>: Sized {

    type Input;
    type Intermediary;

    fn then<Processor, Output, NextMarker>(self, next: Processor) -> Link<Self, Processor>
    where
        Processor: EventProcessor<NextMarker, Input = Self::Intermediary, Intermediary = Output>;

}

struct Link<A, B> {
    a: A,
    b: B,
}

impl<A, B> Link<A, B> {
    fn new(a: A, b: B) -> Self {
        Link { a, b }
    }
}

impl<InputA, Intermediary, MarkerA, MarkerB, A, B> EventProcessor<(MarkerA, MarkerB)> for Link<A, B>
where
    A: EventProcessor<MarkerA, Input = InputA>,
    B: EventProcessor<MarkerB, Intermediary = Intermediary>,
{
    type Input = InputA;
    type Intermediary = Intermediary;

    fn then<Processor, Output, NextMarker>(self, next: Processor) -> Link<Self, Processor>
    where
        Processor: EventProcessor<NextMarker>
    {
        Link::new(self, next)
    }
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

    fn then<Processor, Output, NextMarker>(self, next: Processor) -> Link<Self, Processor>
    where
        Processor: EventProcessor<NextMarker>
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

    fn then<Processor, Output, NextMarker>(self, next: Processor) -> Link<Self, Processor>
    where
        Processor: EventProcessor<NextMarker>
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

    fn then<Processor, Output, NextMarker>(self, next: Processor) -> Link<Self, Processor>
    where
        Processor: EventProcessor<NextMarker>
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

    fn then<Processor, Output, NextMarker>(self, next: Processor) -> Link<Self, Processor>
    where
        Processor: EventProcessor<NextMarker, Input=Self::Intermediary, Intermediary=Output>
    {
        Link::new(self, next)
    }
}

//endregion

#[cfg(test)]
mod tests {
    use crate::node::EventProcessor;
    use bevy::prelude::{Component, Query, SystemInput};

    #[derive(Component)]
    struct A;

    impl SystemInput for A {
        type Param<'i> = A;
        type Inner<'i> = A;

        fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> {
            this
        }
    }

    fn a(_a: A) -> A {
        A
    }

    fn b1(_b: A, _query: Query<&A>) -> A {
        A
    }

    fn b2(_b: A, _query1: Query<&A>, _query2: Query<&A>) -> A {
        A
    }

    fn c(_c: A, _query1: Query<&A>, _query2: Query<&A>) -> A {
        A
    }

    #[test]
    fn test() {
        let linked = a.then((b1, b2)).then(c);
    }
}