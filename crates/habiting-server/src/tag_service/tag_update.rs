use std::sync::Arc;

use crate::prelude::habiting_proto::TagUpdate;

use super::*;

impl HabitingTagService {
    pub(super) async fn handle_update(
        &self,
        req: Request<TagUpdateRequest>,
    ) -> Result<Response<TagUpdateResponse>, Status> {
        let req = req.into_inner();
        info!("{req:?}");

        let _conn = Config::try_get().await?.db_conn.get();
        let _args = Arc::<[TagUpdate]>::from(req.edit_reqs);
        todo!()
    }
}
