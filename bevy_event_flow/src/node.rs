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
    for <'a> &'a mut Func: FnMut(Input) -> Intermediary,
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
    use bevy::prelude::In;
    use crate::node::EventProcessor;

    fn a(_a: u8) -> u16 {
        1
    }

    fn b(_b: u16) -> u32 {
        1
    }

    fn c(_c: u32) -> u64 {
        1
    }

    #[test]
    fn test() {
        let linked = a.then(b).then(c);
    }
}