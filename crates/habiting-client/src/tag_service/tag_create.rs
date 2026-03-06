use super::*;

impl TagService {
    pub(super) async fn tag_create(tag_names: Vec<String>) -> anyhow::Result<()> {
        let request = habiting_proto::TagCreateRequest { names: tag_names };
        let res = TagService::connect()
            .await?
            .tag_create(request)
            .await
            .map_err(ServerError)?
            .into_inner()
            .tags;

        let names = res
            .into_iter()
            .map(|f| f.name)
            .collect::<Vec<String>>()
            .join(", ");

        println!("Successfully created tags: {names}");
        Ok(())
    }
}
