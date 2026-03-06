use crate::prelude::habiting_proto::TagDeleteRequest;

use super::*;

impl TagService {
    pub(super) async fn tag_delete(names: Vec<String>) -> anyhow::Result<()> {
        let request = TagDeleteRequest { names };
        let res = TagService::connect()
            .await?
            .tag_delete(request)
            .await
            .map_err(ServerError)?
            .into_inner()
            .rows_affected;

        println!("{res} tags sucessfully deleted!");
        Ok(())
    }
}
