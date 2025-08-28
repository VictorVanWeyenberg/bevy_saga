use bevy::prelude::{Event, SystemInput};

mod handler;
mod option_processor;
mod plugin;
pub mod prelude;
mod processor;
mod result_handler;
mod result_processor;
mod saga;
mod util;

pub use plugin::RegisterSaga;

pub trait SagaEvent: Event + Clone + SystemInput<Inner<'static> = Self> {}
