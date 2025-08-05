use bevy::prelude::{Event, SystemInput};

mod plugin;
mod handler;
mod processor;
mod option_processor;
mod result_handler;
mod saga;
mod util;

pub use saga::Saga;
pub use plugin::RegisterSaga;
pub use result_handler::{OkStage, ErrStage};

// Fallible Events that won't propagate. ?
// BUG?: Sending the same event multiple times, then updating once (with chaining).

pub trait SagaEvent: Event + Clone + SystemInput<Inner<'static> = Self> {}
