use bevy::prelude::{Event, SystemInput};

mod flow;
mod handler_set;
mod processor_set;
mod processor_flow;
mod util;

pub use flow::RegisterEventFlow;
pub use processor_set::EventProcessorSet;
pub use processor_flow::EventProcessorFlow;

// Fallible Events that won't propagate. ?
// BUG?: Sending the same event multiple times, then updating once (with chaining).

pub trait Request: Event + Clone + SystemInput {}
