#![allow(unused_variables)]
use crate::prelude::{
    habiting_proto::{
        SessionStartRequest, SessionStartResponse, SessionStopRequest, SessionStopResponse,
        session_service_server::SessionService,
    },
    *,
};

mod session_start;
mod session_stop;

pub struct HabitingSessionService {}

#[tonic::async_trait]
impl SessionService for HabitingSessionService {
    async fn session_start(
        &self,
        req: Request<SessionStartRequest>,
    ) -> Result<Response<SessionStartResponse>, Status> {
        self.handle_start(req).await
    }
    async fn session_stop(
        &self,
        req: Request<SessionStopRequest>,
    ) -> Result<Response<SessionStopResponse>, Status> {
        self.handle_stop(req).await
    }
}
