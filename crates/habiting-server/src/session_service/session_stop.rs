use super::*;

impl HabitingSessionService {
    pub(super) async fn handle_stop(
        &self,
        req: Request<SessionStopRequest>,
    ) -> Result<Response<SessionStopResponse>, Status> {
        let req = req.into_inner();
        info!("{req:?}");
        todo!()
    }
}
