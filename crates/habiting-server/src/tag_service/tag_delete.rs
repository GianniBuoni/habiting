use std::sync::Arc;

use sqlx::{PgExecutor, Postgres, QueryBuilder};

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

async fn validate_delete(conn: impl PgExecutor<'_>, args: Arc<[String]>) -> Result<(), Status> {
    let validation = ArgumentsBuilder::new(args)
        .with_table("tags")
        .with_column("name")
        .with_task("tag_delete")
        .try_build()?;

    Ok(validation
        .try_check_empty_args()?
        .try_check_repeated_args()?
        .try_check_entry_exists(conn)
        .await?)
}

async fn tag_delete(conn: impl PgExecutor<'_>, args: Arc<[String]>) -> Result<u64, DbError> {
    let mut q = QueryBuilder::<Postgres>::new("DELETE FROM tags WHERE name IN ");
    q.push_tuples(args.iter().take(PG_BIND_LIMIT), |mut b, name| {
        b.push_bind(name);
    });
    let res = q.build().execute(conn).await?.rows_affected();

    Ok(res)
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::*;

    #[sqlx::test(fixtures("../../fixtures/tags.sql"))]
    async fn test_validations(conn: PgPool) {
        let desc = "Test argument validations";
        let test_cases = [
            (
                vec![],
                Some(ClientError::EmptyArgs("tag_delete".into())),
                "empty",
            ),
            (
                vec!["writing", "writing", "gardening"],
                Some(ClientError::RepeatArgs("writing".into())),
                "repeating args",
            ),
            (
                vec!["writing", "reading", "music", "gardening"],
                Some(ClientError::EntryNotFound(
                    "tags".into(),
                    "music, gardening".into(),
                )),
                "non-existant args",
            ),
            (vec!["reading", "writing"], None, "valid args"),
        ];
        for (args, want, case) in test_cases {
            let args = args.into_iter().map(|f| f.to_string()).collect();
            let got = validate_delete(&conn, args).await;
            let want = match want {
                Some(e) => Status::from(e),
                None => {
                    dbg!(&got);
                    assert!(got.is_ok(), "{desc}: {case}");
                    continue;
                }
            };
            match got {
                Ok(e) => panic!("{EXPECTED_ERROR} {e:?}, {desc}: {case}"),
                Err(e) => assert_eq!(want.to_string(), e.to_string(), "{desc}: {case}"),
            }
        }
    }

    #[sqlx::test(fixtures("../../fixtures/tags.sql"))]
    async fn test_delete(conn: PgPool) -> anyhow::Result<()> {
        let desc = "Basic test that tag_delete function returns expected value";
        let want = 2_u64;
        let args = ["reading".into(), "writing".into()].into();
        let got = tag_delete(&conn, args).await?;

        assert_eq!(want, got, "{desc}");
        anyhow::Ok(())
    }
}
