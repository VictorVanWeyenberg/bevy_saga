use bevy::prelude::{Event, SystemInput};

mod saga;
mod handler_set;
mod processor_set;
mod processor_saga;
mod util;

pub use saga::RegisterSaga;

// Fallible Events that won't propagate. ?
// BUG?: Sending the same event multiple times, then updating once (with chaining).

pub trait SagaEvent: Event + Clone + SystemInput {}
