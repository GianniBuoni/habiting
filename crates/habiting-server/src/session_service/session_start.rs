use super::*;

impl HabitingSessionService {
    pub(super) async fn handle_start(
        &self,
        req: Request<SessionStartRequest>,
    ) -> Result<Response<SessionStartResponse>, Status> {
        let req = req.into_inner();
        info!("{req:?}");
        todo!()
    }
}
