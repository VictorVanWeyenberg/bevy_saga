use bevy::ecs::schedule::ScheduleLabel;
use bevy::ecs::system::SystemId;
use bevy::prelude::{App, Commands, Event, EventReader, EventWriter, In, IntoScheduleConfigs, IntoSystem, Res, Resource};

pub trait EventFlow {
    fn add_event_flow<Request, M>(
        &mut self,
        label: impl ScheduleLabel,
        handler: impl IntoSystem<In<Request>, Request::Response, M> + 'static,
    ) -> &mut Self
    where
        Request: crate::Request;

    fn add_event_flow_after<Request, Prior, M>(
        &mut self,
        label: impl ScheduleLabel,
        handler: impl IntoSystem<In<Request>, Request::Response, M> + 'static,
    ) -> &mut Self
    where
        Request: crate::Request,
        Prior: crate::Request<Response = Request>;
}

impl EventFlow for App {
    fn add_event_flow<Request, M>(
        &mut self,
        label: impl ScheduleLabel,
        handler: impl IntoSystem<In<Request>, Request::Response, M> + 'static,
    ) -> &mut Self
    where
        Request: crate::Request,
    {
        self.add_event::<Request>();
        self.add_event::<Request::Response>();
        let id = self.register_system(handler.pipe(send_response::<Request>));
        self.insert_resource(EventHandlerId { id });
        self.add_systems(label, process_event::<Request>);
        self
    }

    fn add_event_flow_after<Request, Prior, M>(
        &mut self,
        label: impl ScheduleLabel,
        handler: impl IntoSystem<In<Request>, Request::Response, M> + 'static,
    ) -> &mut Self
    where
        Request: crate::Request,
        Prior: crate::Request<Response = Request>,
    {
        self.add_event::<Request>();
        self.add_event::<Request::Response>();
        let id = self.register_system(handler.pipe(send_response::<Request>));
        self.insert_resource(EventHandlerId { id });
        self.add_systems(label, process_event::<Request>.after(process_event::<Prior>));
        self
    }
}

#[derive(Resource)]
struct EventHandlerId<R>
where
    R: Request,
{
    id: SystemId<In<R>, ()>,
}

pub trait Request: Event + Clone {
    type Response: Event;
}

fn process_event<R>(
    mut reader: EventReader<R>,
    handler: Res<EventHandlerId<R>>,
    mut commands: Commands,
) where
    R: Request,
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
