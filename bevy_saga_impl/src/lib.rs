use bevy::prelude::{Event, SystemInput};

mod extension;
mod handler;
mod option_processor;
pub mod prelude;
mod processor;
mod result_handler;
mod result_processor;
mod saga;
mod util;

pub use extension::SagaRegistry;

pub trait SagaEvent: Event + Clone + SystemInput<Inner<'static> = Self> {}
