use crate::handler::EventHandler;
use crate::result_processor::ResultProcessor;
use crate::{Saga, SagaEvent};
use bevy::app::App;
use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::IntoScheduleConfigs;

pub struct ResultHandler<ResultSource, OkSaga, ErrSaga> {
    result_source: ResultSource,
    ok_saga: OkSaga,
    err_saga: ErrSaga,
}

pub struct ResultHandlerM<T>(T);

impl<ResultSource, OkSaga, ErrSaga, MRS, MOP, MEP> EventHandler<ResultHandlerM<(MRS, MOP, MEP)>>
    for ResultHandler<ResultSource, OkSaga, ErrSaga>
where
    ResultSource: ResultProcessor<MRS, Ok = OkSaga::In, Err = ErrSaga::In>,
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

    fn register_handler(self, app: &mut App) -> ScheduleConfigs<ScheduleSystem> {
        let ResultHandler {
            result_source,
            ok_saga,
            err_saga,
        } = self;

        (
            result_source.register_result_processor(app),
            (ok_saga.register(app), err_saga.register(app)).into_configs(),
        )
            .chain()
    }
}

pub trait OkStage<RS, MRS, Ok, Err>
where
    RS: ResultProcessor<MRS, Ok = Ok, Err = Err>,
    RS::In: SagaEvent,
{
    fn ok<OkSaga, MOP>(self, ok_saga: OkSaga) -> impl ErrStage<RS, MRS, MOP, Ok, Err>
    where
        MOP: 'static,
        OkSaga: Saga<MOP, In = Ok>;
}

pub trait ErrStage<RS, MRS, MOP, Ok, Err> {
    fn err<ErrSaga, MEP>(
        self,
        err_saga: ErrSaga,
    ) -> impl EventHandler<ResultHandlerM<(MRS, MOP, MEP)>>
    where
        MEP: 'static,
        ErrSaga: Saga<MEP, In = Err>;
}

struct OkBuilderStage<ResultSource, OkSaga> {
    result_source: ResultSource,
    ok_saga: OkSaga,
}

impl<RS, MRS, Ok, Err> OkStage<RS, MRS, Ok, Err> for RS
where
    RS: ResultProcessor<MRS, Ok = Ok, Err = Err>,
    RS::In: SagaEvent,
    Ok: SagaEvent,
    Err: SagaEvent,
    MRS: 'static,
{
    fn ok<OkSaga, MOP>(self, ok_saga: OkSaga) -> impl ErrStage<RS, MRS, MOP, Ok, Err>
    where
        MOP: 'static,
        OkSaga: Saga<MOP, In = Ok>,
    {
        OkBuilderStage {
            result_source: self,
            ok_saga,
        }
    }
}

impl<RS, MRS, MOP, OkSaga, Ok, Err> ErrStage<RS, MRS, MOP, Ok, Err> for OkBuilderStage<RS, OkSaga>
where
    RS: ResultProcessor<MRS, Ok = Ok, Err = Err>,
    RS::In: SagaEvent,
    OkSaga: Saga<MOP, In = Ok>,
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
        ErrSaga: Saga<MEP, In = Err>,
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
