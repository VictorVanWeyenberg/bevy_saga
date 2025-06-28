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

#[cfg(test)]
mod tests {
    use crate::node::EventProcessor;
    use bevy::prelude::SystemInput;

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

    fn b(_b: A) -> A {
        A
    }

    fn c(_c: A) -> A {
        A
    }

    #[test]
    fn test() {
        let linked = a.then(b).then(c);
    }
}