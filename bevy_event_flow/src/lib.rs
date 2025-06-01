use bevy::prelude::{Event, SystemInput};

mod flow;

pub use flow::EventFlow;

pub trait Request: Event + Clone + SystemInput {}
