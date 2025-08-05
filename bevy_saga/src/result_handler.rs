use crate::handler::EventHandler;
use crate::plugin::BevySagaUtil;
use crate::{Saga, SagaEvent};
use bevy::app::App;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::SystemParamFunction;

pub struct ResultHandler<ResultSource, OkSaga, ErrSaga> {
    result_source: ResultSource,
    ok_saga: OkSaga,
    err_saga: ErrSaga,
}

pub struct ResultHandlerM<T>(T);

impl<ResultSource, OkSaga, ErrSaga, MRS, MOP, MEP> EventHandler<ResultHandlerM<(MRS, MOP, MEP)>>
    for ResultHandler<ResultSource, OkSaga, ErrSaga>
where
    ResultSource: SystemParamFunction<MRS, Out=Result<OkSaga::In, ErrSaga::In>>,
    ResultSource::In: SagaEvent,
    OkSaga: Saga<MOP>,
    ErrSaga: Saga<MEP>,
    OkSaga::In: SagaEvent,
    ErrSaga::In: SagaEvent,
    MRS: 'static,
    MOP: 'static,
    MEP: 'static,
{
    type In = ResultSource::In;

    fn register_handler<Label>(self, label: Label, app: &mut App)
    where
        Label: ScheduleLabel + Clone,
    {
        let ResultHandler {
            result_source,
            ok_saga,
            err_saga,
        } = self;
        app.add_result_handler(result_source);
        ok_saga.register(label.clone(), app);
        err_saga.register(label.clone(), app);
    }
}

pub trait OkStage<RS, MRS, Ok, Err>
where
    RS: SystemParamFunction<MRS, Out=Result<Ok, Err>>,
    RS::In: SagaEvent,
{
    fn ok<OkSaga, MOP>(self, ok_saga: OkSaga) -> impl ErrStage<RS, MRS, MOP, Ok, Err>
    where
        MOP: 'static,
        OkSaga: Saga<MOP, In=Ok>;
}

pub trait ErrStage<RS, MRS, MOP, Ok, Err> {
    fn err<ErrSaga, MEP>(
        self,
        err_saga: ErrSaga,
    ) -> impl EventHandler<ResultHandlerM<(MRS, MOP, MEP)>>
    where
        MEP: 'static,
        ErrSaga: Saga<MEP, In=Err>;
}

struct OkBuilderStage<ResultSource, OkSaga> {
    result_source: ResultSource,
    ok_saga: OkSaga,
}

impl<RS, MRS, Ok, Err> OkStage<RS, MRS, Ok, Err> for RS
where
    RS: SystemParamFunction<MRS, Out=Result<Ok, Err>>,
    RS::In: SagaEvent,
    Ok: SagaEvent,
    Err: SagaEvent,
    MRS: 'static,
{
    fn ok<OkSaga, MOP>(self, ok_saga: OkSaga) -> impl ErrStage<RS, MRS, MOP, Ok, Err>
    where
        MOP: 'static,
        OkSaga: Saga<MOP, In=Ok>,
    {
        OkBuilderStage {
            result_source: self,
            ok_saga,
        }
    }
}

impl<RS, MRS, MOP, OkSaga, Ok, Err> ErrStage<RS, MRS, MOP, Ok, Err> for OkBuilderStage<RS, OkSaga>
where
    RS: SystemParamFunction<MRS, Out=Result<Ok, Err>>,
    RS::In: SagaEvent,
    OkSaga: Saga<MOP, In=Ok>,
    Ok: SagaEvent,
    Err: SagaEvent,
    MRS: 'static,
    MOP: 'static,
{
    fn err<ErrSaga, MEP>(
        self,
        err_saga: ErrSaga,
    ) -> impl EventHandler<ResultHandlerM<(MRS, MOP, MEP)>>
    where
        MEP: 'static,
        ErrSaga: Saga<MEP, In=Err>,
    {
        let OkBuilderStage {
            result_source,
            ok_saga,
        } = self;
        ResultHandler {
            result_source,
            ok_saga,
            err_saga,
        }
    }
}
