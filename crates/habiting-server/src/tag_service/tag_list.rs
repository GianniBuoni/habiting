use sqlx::PgExecutor;

use super::{tag_row::TagPgRow, *};

impl HabitingTagService {
    pub(super) async fn handle_list(
        &self,
        req: Request<TagListRequest>,
    ) -> Result<Response<TagListResponse>, Status> {
        let req = req.into_inner();
        info!("{req:?}");

        let conn = Config::try_get().await?.db_conn.get();

        let tags = tokio::time::timeout(DBConn::context(), tag_list(conn))
            .await
            .map_err(|_| DbError::Context("tag_list"))??
            .into_iter()
            .map(habiting_proto::Tag::from)
            .collect();

        Ok(Response::new(TagListResponse { tags }))
    }
}

async fn tag_list(conn: impl PgExecutor<'_>) -> Result<Vec<TagPgRow>, DbError> {
    Ok(sqlx::query_as!(TagPgRow, "SELECT * FROM tags")
        .fetch_all(conn)
        .await?)
}

#[cfg(test)]
mod test {
    use sqlx::PgPool;

    use super::*;

    #[sqlx::test(fixtures("../../fixtures/tags.sql"))]
    async fn test_list(conn: PgPool) -> anyhow::Result<()> {
        let desc = "Basic test, listing all items in tags table";
        let want = 2;
        let got = tag_list(&conn).await?;

        assert_eq!(want, got.len(), "{desc}");
        anyhow::Ok(())
    }
}
