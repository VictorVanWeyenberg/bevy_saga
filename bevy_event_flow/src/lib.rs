use bevy::ecs::schedule::ScheduleLabel;
use bevy::ecs::system::SystemId;
use bevy::prelude::{App, Commands, EventReader, EventWriter, In, IntoScheduleConfigs, IntoSystem, Res, Resource, SystemInput};

pub trait Request: bevy::prelude::Event + Clone {
    type Response: bevy::prelude::Event;
}

pub trait EventFlow {
    fn add_event_flow<R, M>(
        &mut self,
        label: impl ScheduleLabel,
        handler: impl IntoSystem<R, R::Response, M> + 'static,
    ) -> &mut Self
    where
        R: Request + SystemInput<Inner<'static> =R>;

    fn add_event_flow_after<R, P, M>(
        &mut self,
        label: impl ScheduleLabel,
        handler: impl IntoSystem<R, R::Response, M> + 'static,
    ) -> &mut Self
    where
        R: Request + SystemInput<Inner<'static> =R>,
        P: Request<Response =R> + SystemInput<Inner<'static> =P>;
}

impl EventFlow for App {
    fn add_event_flow<R, M>(
        &mut self,
        label: impl ScheduleLabel,
        handler: impl IntoSystem<R, R::Response, M> + 'static,
    ) -> &mut Self
    where
        R: Request + SystemInput<Inner<'static> =R>,
    {
        self.add_event::<R>();
        self.add_event::<R::Response>();
        let id = self.register_system(handler.pipe(send_response::<R>));
        self.insert_resource(EventHandlerId { id });
        self.add_systems(label, process_event::<R>);
        self
    }

    fn add_event_flow_after<R, P, M>(
        &mut self,
        label: impl ScheduleLabel,
        handler: impl IntoSystem<R, R::Response, M> + 'static,
    ) -> &mut Self
    where
        R: Request + SystemInput<Inner<'static> =R>,
        P: Request<Response =R> + SystemInput<Inner<'static> =P>,
    {
        self.add_event::<R>();
        self.add_event::<R::Response>();
        let id = self.register_system(handler.pipe(send_response::<R>));
        self.insert_resource(EventHandlerId { id });
        self.add_systems(label, process_event::<R>.after(process_event::<P>));
        self
    }
}

#[derive(Resource)]
struct EventHandlerId<R>
where
    R: Request + SystemInput,
{
    id: SystemId<R, ()>,
}

fn process_event<R>(
    mut reader: EventReader<R>,
    handler: Res<EventHandlerId<R>>,
    mut commands: Commands,
) where
    R: Request + SystemInput<Inner<'static> = R>,
{
    for event in reader.read().cloned() {
        commands.run_system_with(handler.id, event)
    }
}

fn send_response<R>(In(response): In<R::Response>, mut writer: EventWriter<R::Response>)
where
    R: Request,
{
    writer.write(response);
}
