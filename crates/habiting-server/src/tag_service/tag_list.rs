use super::*;

impl HabitingTagService {
    pub(super) async fn handle_list(
        &self,
        req: Request<TagListRequest>,
    ) -> Result<Response<TagListResponse>, Status> {
        let req = req.into_inner();
        info!("{req:?}");

        let _conn = Config::try_get().await?.db_conn.get();

        todo!()
    }
}
