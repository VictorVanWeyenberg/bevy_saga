use crate::event_processor::EventProcessor;
use std::marker::PhantomData;

pub struct Link<A, B, MarkerA, MarkerB>
where
    A: EventProcessor<MarkerA, Intermediary = B::Input>,
    B: EventProcessor<MarkerB>,
    MarkerA: 'static,
    MarkerB: 'static,
{
    a: A,
    b: B,
    _phantom: PhantomData<(MarkerA, MarkerB)>
}

impl<A, B, MarkerA, MarkerB> Link<A, B, MarkerA, MarkerB>
where
    A: EventProcessor<MarkerA, Intermediary = B::Input>,
    B: EventProcessor<MarkerB>,
    MarkerA: 'static,
    MarkerB: 'static,
{
    pub fn new(a: A, b: B) -> Self {
        Link { a, b, _phantom: PhantomData }
    }
}

impl<InputA, Intermediary, MarkerA, MarkerB, A, B> EventProcessor<(MarkerA, MarkerB)> for Link<A, B, MarkerA, MarkerB>
where
    A: EventProcessor<MarkerA, Input = InputA, Intermediary = B::Input>,
    B: EventProcessor<MarkerB, Intermediary = Intermediary>,
    MarkerA: 'static,
    MarkerB: 'static,
{
    type Input = InputA;
    type Intermediary = Intermediary;

    fn then<Processor, Output, NextMarker>(self, next: Processor) -> Link<Self, Processor, (MarkerA, MarkerB), NextMarker>
    where
        Processor: EventProcessor<NextMarker, Input = Self::Intermediary>
    {
        Link::new(self, next)
    }
}

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

    fn d(_a: A) {}

    #[test]
    fn test() {
        let _linked = a.then((b1, b2)).then(c).then(d);
    }
}