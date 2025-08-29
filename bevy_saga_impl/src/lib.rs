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

/// The trait type that propagates through your sagas.
///
/// You don't have to implement this trait directly. Simple use the `#[saga_event]` attribute to
/// implement this trait.
///
/// Every type that is sent through a saga needs to implement this SagaEvent trait.
///
/// The attribute `#[saga_router]` indirectly also implements SagaEvent so you don't have to add
/// the `#[saga_event]` attribute if your type is already attributed with `#[saga_router]`.
pub trait SagaEvent: Event + Clone + SystemInput<Inner<'static> = Self> {}
