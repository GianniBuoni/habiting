use crate::{
    cli::UpdateArgs,
    prelude::habiting_proto::{TagUpdate, TagUpdateRequest},
};

use super::*;

impl TagService {
    pub(super) async fn tag_update(args: UpdateArgs) -> anyhow::Result<()> {
        if args.targets.len() != args.new_names.len() {
            return Err(ClientError::UnequalArgs(args).into());
        }

        let edit_reqs = args
            .targets
            .into_iter()
            .zip(args.new_names)
            .map(|(target, new_name)| TagUpdate { target, new_name })
            .collect::<Vec<TagUpdate>>();
        let request = TagUpdateRequest { edit_reqs };

        let res = TagService::connect()
            .await?
            .tag_update(request)
            .await
            .map_err(ServerError)?
            .into_inner()
            .tags;

        println!("{} tags sucessfully updated!", res.len());
        Ok(())
    }
}
