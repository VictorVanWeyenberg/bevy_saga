use bevy::ecs::schedule::ScheduleLabel;
use crate::{
    plugin::BevySagaUtil,
    SagaEvent,
};
use bevy::prelude::{App, SystemParamFunction};

pub trait EventHandler<M> {
    type In: SagaEvent;

    fn register_handler<Label>(self, label: Label, app: &mut App)
where
    Label: ScheduleLabel + Clone;
}

impl<SPF, M, In> EventHandler<(M,)> for SPF
where
    In: SagaEvent,
    SPF: SystemParamFunction<M, In = In, Out = ()>,
    M: 'static,
{
    type In = In;

    fn register_handler<Label>(self, _label: Label, app: &mut App)
where
    Label: ScheduleLabel + Clone {
        app.add_event_handler::<In, _>(self);
    }
}

impl<SPF1, SPF2, M1, M2, In> EventHandler<(M1, M2)> for (SPF1, SPF2)
where
    In: SagaEvent,
    SPF1: SystemParamFunction<M1, In = In, Out = ()>,
    SPF2: SystemParamFunction<M2, In = In, Out = ()>,
    M1: 'static,
    M2: 'static,
{
    type In = In;

    fn register_handler<Label>(self, _label: Label, app: &mut App)
where
    Label: ScheduleLabel + Clone {
        let (spf1, spf2) = self;
        app.add_event_handler::<In, _>(spf1);
        app.add_event_handler::<In, _>(spf2);
    }
}

impl<SPF1, SPF2, SPF3, M1, M2, M3, In> EventHandler<(M1, M2, M3)> for (SPF1, SPF2, SPF3)
where
    In: SagaEvent,
    SPF1: SystemParamFunction<M1, In = In, Out = ()>,
    SPF2: SystemParamFunction<M2, In = In, Out = ()>,
    SPF3: SystemParamFunction<M3, In = In, Out = ()>,
    M1: 'static,
    M2: 'static,
    M3: 'static,
{
    type In = In;

    fn register_handler<Label>(self, _label: Label, app: &mut App)
where
    Label: ScheduleLabel + Clone {
        let (spf1, spf2, spf3) = self;
        app.add_event_handler::<In, _>(spf1);
        app.add_event_handler::<In, _>(spf2);
        app.add_event_handler::<In, _>(spf3);
    }
}

impl<SPF1, SPF2, SPF3, SPF4, M1, M2, M3, M4, In> EventHandler<(M1, M2, M3, M4)> for (SPF1, SPF2, SPF3, SPF4)
where
    In: SagaEvent,
    SPF1: SystemParamFunction<M1, In = In, Out = ()>,
    SPF2: SystemParamFunction<M2, In = In, Out = ()>,
    SPF3: SystemParamFunction<M3, In = In, Out = ()>,
    SPF4: SystemParamFunction<M4, In = In, Out = ()>,
    M1: 'static,
    M2: 'static,
    M3: 'static,
    M4: 'static,
{
    type In = In;

    fn register_handler<Label>(self, _label: Label, app: &mut App)
where
    Label: ScheduleLabel + Clone {
        let (spf1, spf2, spf3, spf4) = self;
        app.add_event_handler::<In, _>(spf1);
        app.add_event_handler::<In, _>(spf2);
        app.add_event_handler::<In, _>(spf3);
        app.add_event_handler::<In, _>(spf4);
    }
}

