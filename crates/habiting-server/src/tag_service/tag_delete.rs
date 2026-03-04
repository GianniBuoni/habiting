use std::sync::Arc;

use super::*;

impl HabitingTagService {
    pub(super) async fn handle_delete(
        &self,
        req: Request<TagDeleteRequest>,
    ) -> Result<Response<TagDeleteResponse>, Status> {
        let req = req.into_inner();
        info!("{req:?}");

        let _conn = Config::try_get().await?.db_conn.get();
        let _args = Arc::<[String]>::from(req.names);

        todo!()
    }
}
