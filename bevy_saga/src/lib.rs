//! An event and system management extension for Bevy.
//!
//! Bevy_saga is an extension for Bevy that allows you to easily process events by chaining event 
//! processors. Most public items are only made public for the internal workings of bevy_saga.
//! The only item you'll directly interact with is the extension itself: [SagaRegistry].
//!
//! Read the extension documentation [here](SagaRegistry).
//!
//! # Benefits
//!
//! - You no longer have to write the boilerplate code to read or write events from or to
//!   [EventReaders](bevy::prelude::EventReader) or [EventWriters](bevy::prelude::EventWriter).
//! - You no longer have to order the systems manually. The extension does that automatically for 
//!   you.
//! - Bevy_saga encourages single-responsibility design. You can write smaller systems that are
//!   easier to unit test.
//! - Contrary to Bevy, you'll get a compilation error if you change input or output events on
//!   processors or handlers.
//! - All sagas fully handle all sent events in one update cycle. Bevy_saga guarantees no frame
//!   delay.
//!
//! # How it works
//!
//! It is recommended to first read the [extension documentation](SagaRegistry) before reading this
//! section.
//!
//! In bevy_saga you register _event processor_ functions.
//! Event processor functions have two events: one as input parameter and one as output parameter.
//! By structuring those event processors in tuples, Rust can check if the output of one
//! system is the same type as the input of the following system. This is done using
//! [generic bounds](https://doc.rust-lang.org/std/keyword.where.html).
//!
//! Bevy_saga then takes each event processor and pipes it to a _send system_. If the definition of
//! your event processor looks like this:
//!
//! ```
//! # struct Input;
//! # struct Output;
//! fn event_processor(event: Input) -> Output {
//!     Output
//! }
//! ```
//!
//! ... Then the _send system_ looks like this:
//!
//! ```
//! # use bevy::prelude::{Event, EventWriter, In};
//! pub fn send_response<Rs>(In(response): In<Rs>, mut writer: EventWriter<Rs>)
//! where
//!     Rs: Event,
//! {
//!     writer.write(response);
//! }
//! ```
//!
//! The _send system_ hides the [EventWriter](bevy::prelude::EventWriter) boilerplate for the
//! developer.
//!
//! By piping `event_processor` to `send_response`, we get one
//! [PipeSystem](bevy::ecs::system::PipeSystem) with an input (your event) and no output.
//! This system can be [registered in the app](bevy::prelude::App::register_system) and
//! the [SystemId](bevy::ecs::system::SystemId) is then stored in the
//! [EventProcessors](prelude::EventProcessors) resource.
//!
//! Finally, bevy_saga_impl owns some generic [EventReader](bevy::prelude::EventReader) systems.
//! Such a system looks like this:
//!
//! ```
//! # use bevy::prelude::{ResMut, Events, Res, Commands};
//! # use bevy_saga_impl::{SagaEvent, prelude::{EventProcessors}};
//! pub fn process_event<R>(
//!     mut reader: ResMut<Events<R>>,
//!     handler: Res<EventProcessors<R>>,
//!     mut commands: Commands,
//! ) where
//!     R: SagaEvent,
//! {
//!     // Pass all the `events` in reader to all processors in `handler`. 
//! }
//! ```
//!
//! Bevy_saga will drain all events from the [Events](bevy::prelude::Events) resource. The
//! [EventProcessors](prelude::EventProcessors) resource holds all the 
//! [SystemIds](bevy::ecs::system::SystemId) for the piped systems that handle your input event. 
//! The piped systems are run through the [Commands](bevy::prelude::Commands).
//!
//! The last thing bevy_saga needs to do is order all the event reading systems. This is done
//! through recursively returning those systems while we register the pipes in the extension. When
//! you write a saga and register it in the [extension](SagaRegistry), bevy_saga will generate
//! one big [schedule system](bevy::prelude::IntoScheduleConfigs) that uses all your event
//! processor systems.
//!
//! Bevy_saga hides all the boilerplate in generic methods. That boilerplate is prepended and
//! appended to the event processors you provide. Finally, those composite systems are ordered.
//! Everything is checked at compile time.
//!
//! # Pitfalls
//!
//! > Once you use bevy_saga, you won't be able to register separate event reading systems that use
//! > the events that are in the saga.
//!
//! We drain the events from the [Events](bevy::prelude::Events) buffer. This is a method of
//! manually managing the lifetime of events. We do this to prevent an event of triggering a system
//! chain more than once. Because we manually drain the events from the buffer, if you register
//! another system that expects that event, it cannot be guaranteed that the event will trigger
//! that system.
//!
//! The way bevy_saga is built always allows you to add such a system to the saga.
//!
//! # Example
//!
//! ```
//! use bevy::app::App;
//! use bevy::prelude::{Component, Entity, Query, Update};
//! use bevy_saga_impl::{SagaRegistry, prelude::{OkStage, ErrStage}};
//! use bevy_saga_macros::saga_event;
//!
//! #[derive(Component)]
//! struct Weapon(u8);
//!
//! #[derive(Component)]
//! struct Armor(u8);
//!
//! #[derive(Component)]
//! struct Health(u8);
//!
//! #[saga_event]
//! struct AttackTrigger {
//!     by: Entity,
//!     to: Entity,
//! }
//!
//! #[saga_event]
//! struct Offense {
//!     by: Entity,
//!     attack: u8,
//!     to: Entity,
//! }
//!
//! #[saga_event]
//! struct Attack {
//!     by: Entity,
//!     attack: u8,
//!     to: Entity,
//!     defense: u8,
//! }
//!
//! #[saga_event]
//! struct Damage {
//!     to: Entity,
//!     damage: u8,
//! }
//!
//! #[saga_event]
//! struct AttackDone;
//!
//! #[saga_event]
//! struct Error(String);
//!
//! fn calculate_offense(attack: AttackTrigger, query: Query<&Weapon>, /* other queries or resources */) -> Result<Offense, Error> {
//!     let AttackTrigger { by, to } = attack;
//!     if let Ok(Weapon(attack_strength)) = query.get(by) {
//!         Ok(Offense {
//!             by,
//!             attack: *attack_strength,
//!             to,
//!         })
//!     } else {
//!         Err(Error("Attacker has no weapon.".to_string()))
//!     }
//! }
//!
//! fn calculate_defense(offense: Offense, query: Query<&Armor>, /* other queries or resources */) -> Result<Attack, Error> {
//!     let Offense { by, attack, to } = offense;
//!     if let Ok(Armor(defense_strength)) = query.get(to) {
//!         Ok(Attack {
//!             by,
//!             attack,
//!             to,
//!             defense: *defense_strength,
//!         })
//!     } else {
//!         Err(Error("Defender has no armor.".to_string()))
//!     }
//! }
//!
//! fn perform_attack(attack: Attack, /* other queries or resources */) -> Option<Damage> {
//!     let Attack {
//!         by, attack, to, defense,
//!     } = attack;
//!     if attack > defense {
//!         Some(Damage {
//!             to,
//!             damage: attack - defense,
//!         })
//!     } else {
//!         None
//!     }  
//! }
//!
//! fn take_damage(damage: Damage, mut query: Query<&mut Health>, /* other queries or resources */) -> Result<AttackDone, Error> {
//!     let Damage { to, damage } = damage;
//!     if let Ok(mut health) = query.get_mut(to) {
//!         println!("Taking {} damage.", damage);
//!         health.0 -= damage;
//!         Ok(AttackDone)
//!     } else {
//!         Err(Error("Defender doesn't have any health.".to_string()))
//!     }
//! }
//!
//! fn finalize_attack(attack_done: AttackDone, /* other queries or resources */) {
//!     println!("Attack saga finished!")
//! }
//!
//! fn send_network_event(damage: Damage, /* other queries or resources */) {
//!     // -- snip --
//! }
//!
//! fn handle_error(error: Error, /* other queries or resources */) {
//!     panic!("{}", error.0)
//! }
//!
//! let mut app = App::new();
//! app.add_saga(Update, calculate_offense
//!     .ok(
//!         calculate_defense
//!             .ok((perform_attack,
//!                 (take_damage, send_network_event)
//!                 .ok(finalize_attack)
//!                 .err(handle_error)
//!             ))
//!             .err(handle_error)
//!     )
//!     .err(handle_error));
//! let attacker = app.world_mut().spawn((Weapon(5), Armor(5), Health(10))).id();
//! let defender = app.world_mut().spawn((Weapon(0), Armor(3), Health(10))).id();
//! app.world_mut().send_event(AttackTrigger { by: attacker, to: defender });
//! app.update();
//! if let Ok(Health(health)) = app.world_mut().query::<&Health>().get(app.world(), defender) {
//!     assert_eq!(8, *health)
//! }
//! ```
//!
//! # Bevy Compatibility Matrix
//!
//! | bevy_saga | bevy |
//! |-----------|------|
//! | 0.1       | 0.16 |

pub mod prelude;

pub use bevy_saga_impl::*;
pub use bevy_saga_macros::*;