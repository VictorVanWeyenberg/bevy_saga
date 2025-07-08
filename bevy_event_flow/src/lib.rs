use bevy::prelude::{Event, SystemInput};

mod flow;
mod node;
mod event_processor;

pub use flow::EventFlow;

// Fallible Events that won't propagate. ?
// BUG?: Sending the same event multiple times, then updating once (with chaining).

pub trait Request: Event + Clone + SystemInput {}
