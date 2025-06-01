use bevy::prelude::{Event, SystemInput};

mod flow;

pub use flow::EventFlow;

pub trait Request: bevy::prelude::Event + Clone {
    type Response: bevy::prelude::Event;
}
