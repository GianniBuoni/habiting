use super::*;

impl TagService {
    pub(super) async fn tag_list() -> anyhow::Result<()> {
        let request = habiting_proto::TagListRequest {};
        let res = TagService::connect()
            .await?
            .tag_list(request)
            .await
            .map_err(ServerError)?
            .into_inner()
            .tags;

        println!("Available tags:\n");
        res.iter().for_each(|f| println!("{}", f.name));

        Ok(())
    }
}
