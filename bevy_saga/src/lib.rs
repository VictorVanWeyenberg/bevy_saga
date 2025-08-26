use bevy::prelude::{Event, SystemInput};

mod handler;
mod option_processor;
mod plugin;
mod processor;
mod result_handler;
mod result_processor;
mod saga;
mod util;

pub use saga::Saga;
pub use handler::EventHandler;
pub use processor::EventProcessor;
pub use plugin::RegisterSaga;
pub use result_handler::{OkStage, ErrStage};
pub use util::{process_event, EventProcessors};

pub trait SagaEvent: Event + Clone + SystemInput<Inner<'static> = Self> {}
