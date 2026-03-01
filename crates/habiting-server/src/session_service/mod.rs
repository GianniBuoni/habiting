#![allow(unused_variables)]
use crate::prelude::{
    habiting_proto::{
        SessionStartRequest, SessionStartResponse, SessionStopRequest, SessionStopResponse,
        session_service_server::SessionService,
    },
    *,
};

pub struct HabitingSessionService {}

#[tonic::async_trait]
impl SessionService for HabitingSessionService {
    async fn session_start(
        &self,
        req: Request<SessionStartRequest>,
    ) -> Result<Response<SessionStartResponse>, Status> {
        todo!()
    }
    async fn session_stop(
        &self,
        req: Request<SessionStopRequest>,
    ) -> Result<Response<SessionStopResponse>, Status> {
        todo!()
    }
}
