use crate::{extension::BevySagaUtil, SagaEvent};
use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{App, IntoScheduleConfigs, SystemParamFunction};
use variadics_please::all_tuples;

/// The definition of an event handler.
///
/// The event handler is the last item in a chain to create a saga. Since there is no output event,
/// bevy_saga does not expect any following processors. The saga has thus ended.
///
/// An event handler is a collection of functions that all have the same type as input
/// parameter. None of the systems in the collection may have a return type.
///
/// The input type has to be attributed with `saga_event`.
///
/// All systems in the collection are executed concurrently.
///
/// # Example
///
/// ```
/// # use bevy::app::{App, Update};
/// # use bevy::prelude::{Component, Query};
/// # use bevy_saga_impl::SagaRegistry;
/// # use bevy_saga_macros::saga_event;
/// #[saga_event]
/// struct A;
///
/// fn handler1(_: A, /* other queries or resources */) { }
/// fn handler2(_: A, /* other queries or resources */) { }
/// fn handler3(_: A, /* other queries or resources */) { }
///
/// # let mut app = App::new();
///
/// // One event handler function is a valid event handler.
/// // One handler is also a valid saga.
/// app.add_saga(Update, handler1);
///
/// // A handler can also be accompanied by other functions that handle the event.
/// let handler = (handler1, handler2, handler3);
/// app.add_saga(Update, handler);
/// ```
///
/// # Special Handlers
///
/// There are two special types of event handlers.
///
/// ## Result Handlers
///
/// When an event processor returns a result and both the Ok and the Err types are saga events, the
/// processor can be turned into a result handler. The handler ends the saga chain but in addition,
/// two new sagas can be passed in. If the result is Ok, the Ok value will be propagated through the
/// Ok saga. If the result is Err, the Err value will be propagated through the Err saga.
///
/// ```
/// # use bevy_saga_macros::saga_event;
/// #[saga_event]
/// struct N(u8);
///
/// #[saga_event]
/// struct Yes;
///
/// #[saga_event]
/// struct No;
///
/// fn is_even(N(n): N) -> Result<Yes, No> {
///     if n % 2 == 0 {
///         Ok(Yes)
///     } else {
///         Err (No)
///     }
/// }
/// ```
///
/// Given this function, you can define two sagas. One for when N is even and one for when N is
/// odd.
///
/// ```
/// # use bevy_saga_macros::saga_event;
/// # #[saga_event]
/// # struct Yes;
/// # #[saga_event]
/// # struct No;
/// fn if_even(_: Yes) {
///    println!("It's even!")
/// }
///
/// fn if_odd(_: No) {
///    println!("It's odd!")
/// }
/// ```
///
/// Then you can create one handler by using the [OkStage](crate::prelude::OkStage) and
/// [ErrStage](crate::prelude::ErrStage) trait methods that are implemented for those result
/// processors.
///
/// ```
/// # use bevy::app::App;
/// # use bevy::prelude::Update;
/// use bevy_saga_impl::prelude::{OkStage, ErrStage};
/// # use bevy_saga_impl::SagaRegistry;
/// # use bevy_saga_macros::saga_event;
/// # #[saga_event]
/// # struct N(u8);
/// # #[saga_event]
/// # struct Yes;
/// # #[saga_event]
/// # struct No;
/// # fn is_even(N(n): N) -> Result<Yes, No> {
/// #     if n % 2 == 0 {
/// #         Ok(Yes)
/// #     } else {
/// #         Err (No)
/// #     }
/// # }
/// # fn if_even(_: Yes) {
/// #    println!("It's even!")
/// # }
/// # fn if_odd(_: No) {
/// #    println!("It's odd!")
/// # }
/// # let mut app = App::new();
/// app.add_saga(Update, is_even.ok(if_even).err(if_odd));
/// ```
///
/// The Ok saga will only be executed if the Result returned Ok. The Err saga will only be executed
/// if the Result returned Err.
///
/// Since the `is_even` function is actually an [event processor](crate::prelude::EventProcessor),
/// you can also call `ok` on an event processor tuple where the first function returns a result.
///
/// ## Event Routers
///
/// When an event processor returns an enum, you can trigger different sagas depending on which
/// enum value was returned. This works just like a result handler only with more sagas and more
/// macro magic.
///
/// Instead of attributing the enum with `saga_event`, you have to attribute it with `saga_router`.
/// Every variant in the enum must have one unnamed field. The type of that field must be a
/// `saga_event`.
///
/// Event processors that return that enum will get access to methods for you to add your sagas
/// into. If a variant is defined as `Foo(Bar)`, you will be able to call the `.foo(...)` method on
/// your processor. In that `foo` method you can add a saga that has `Bar` as input type.
///
/// ```
/// use bevy::app::{App, Update};
/// use bevy_saga_impl::SagaRegistry;
/// use bevy_saga_macros::{saga_event, saga_router};
///
/// #[saga_event]
/// struct Sock(Color);
///
/// #[saga_router]
/// enum Color {
///     Red(Red),
///     Green(Green),
///     Blue(Blue),
/// }
///
/// #[saga_event]
/// struct Red;
///
/// #[saga_event]
/// struct Green;
///
/// #[saga_event]
/// struct Blue;
///
/// fn sock_color(Sock(color): Sock) -> Color {
///     color
/// }
///
/// fn if_red(_: Red) { println!("It's red!") }
/// fn if_green(_: Green) { println!("It's green!") }
/// fn if_blue(_: Blue) { println!("It's blue!") }
///
/// # let mut app = App::new();
/// app.add_saga(Update, sock_color.red(if_red).green(if_green).blue(if_blue));
/// ```
pub trait EventHandler<M> {
    type In: SagaEvent;

    fn register_handler(
        self,
        app: &mut App,
    ) -> ScheduleConfigs<ScheduleSystem>;
}

impl<SPF, M, In> EventHandler<(M,)> for SPF
where
    In: SagaEvent,
    SPF: SystemParamFunction<M, In = In, Out = ()>,
    M: 'static,
{
    type In = In;

    fn register_handler(
        self,
        app: &mut App,
    ) -> ScheduleConfigs<ScheduleSystem>
    {
        app.add_event_handler::<In, _>(self)
    }
}

macro_rules! impl_event_handler {
    ($(#[$meta:meta])* $(($SPF:ident, $spf:ident, $M:ident)),*) => {
        impl<$($SPF,)* $($M,)* In> EventHandler<($($M,)*)> for ($($SPF,)*)
        where
            In: SagaEvent,
            $($SPF: SystemParamFunction<$M, In = In, Out = ()>,)*
            $($M: 'static,)*
        {
            type In = In;

            fn register_handler(
                self,
                app: &mut App,
            ) -> ScheduleConfigs<ScheduleSystem>
            {
                let ($($spf,)*) = self;
                (
                    $(app.add_event_handler::<In, _>($spf),)*
                )
                    .into_configs()
            }
        }
    }
}

all_tuples!(impl_event_handler, 2, 16, SPF, spf, M);