use std::sync::Arc;

use sqlx::{PgExecutor, Postgres, QueryBuilder};

use crate::prelude::habiting_proto::TagUpdate;

use super::{tag_row::TagPgRow, *};

const TABLE: &str = "tags";
const COLUMN: &str = "name";
const TASK: &str = "user_create";

impl HabitingTagService {
    pub(super) async fn handle_update(
        &self,
        req: Request<TagUpdateRequest>,
    ) -> Result<Response<TagUpdateResponse>, Status> {
        let req = req.into_inner();
        info!("{req:?}");

        let conn = Config::try_get().await?.db_conn.get();
        let args = Arc::<[TagUpdate]>::from(req.edit_reqs);

        let tags = tokio::time::timeout(DBConn::context(), async {
            // start tx
            let mut tx = conn.begin().await.map_err(DbError::Sql)?;
            // run validations
            let targets = args.iter().cloned().map(|f| f.target).collect();
            validate_targets(&mut *tx, targets).await?;
            let new_names = args.iter().cloned().map(|f| f.new_name).collect();
            validate_new_names(&mut *tx, new_names).await?;
            // sql statement
            let res = tag_update(&mut *tx, args).await?;
            // commit tx
            tx.commit().await.map_err(DbError::Sql)?;
            Ok::<Vec<TagPgRow>, Status>(res)
        })
        .await
        .map_err(|_| DbError::Context(TASK))??
        .into_iter()
        .map(habiting_proto::Tag::from)
        .collect();

        Ok(Response::new(TagUpdateResponse { tags }))
    }
}

async fn validate_targets(conn: impl PgExecutor<'_>, args: Arc<[String]>) -> Result<(), Status> {
    let validate_targets = ArgumentsBuilder::new(args)
        .with_column(COLUMN)
        .with_table(TABLE)
        .with_task(TASK)
        .try_build()?;

    validate_targets
        .try_check_empty_args()?
        .try_check_repeated_args()?
        .try_check_entry_exists(conn)
        .await?;

    Ok(())
}

async fn validate_new_names(conn: impl PgExecutor<'_>, args: Arc<[String]>) -> Result<(), Status> {
    let validate_new_names = ArgumentsBuilder::new(args)
        .with_table(TABLE)
        .with_column(COLUMN)
        .with_task(TASK)
        .try_build()?;

    validate_new_names
        .try_check_empty_args()?
        .try_check_repeated_args()?
        .try_check_unique_constraint(conn)
        .await?;

    Ok(())
}

async fn tag_update(
    conn: impl PgExecutor<'_>,
    args: Arc<[TagUpdate]>,
) -> Result<Vec<TagPgRow>, DbError> {
    let mut q = QueryBuilder::<Postgres>::new(
        "UPDATE tags SET name = data.new_name, updated_at = CURRENT_TIMESTAMP FROM (",
    );
    q.push_values(args.iter().take(PG_BIND_LIMIT / 2), |mut b, req| {
        b.push_bind(&req.target);
        b.push_bind(&req.new_name);
    });
    q.push(") AS data(target, new_name) WHERE tags.name = data.target RETURNING *");

    Ok(q.build_query_as::<TagPgRow>().fetch_all(conn).await?)
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::*;

    const REPEATING: [&str; 2] = ["writing", "writing"];
    const IN_DB: [&str; 2] = ["reading", "writing"];
    const NOT_IN_DB: [&str; 2] = ["weaving", "smithing"];

    #[sqlx::test(fixtures("../../fixtures/tags.sql"))]
    async fn test_target_validations(conn: PgPool) {
        let desc = "Test target validation arguments";
        let test_cases = [
            (
                REPEATING,
                Some(ClientError::RepeatArgs("writing".into())),
                "repeating",
            ),
            (
                NOT_IN_DB,
                Some(ClientError::EntryNotFound(
                    TABLE.into(),
                    "weaving, smithing".into(),
                )),
                "not in db",
            ),
            (IN_DB, None, "valid targets"),
        ];
        for (args, want, case) in test_cases {
            let args = args.iter().map(|f| f.to_string()).collect();
            let got = validate_targets(&conn, args).await;
            let want = match want {
                Some(e) => Status::from(e),
                None => {
                    assert!(got.is_ok(), "{desc}: {case}");
                    continue;
                }
            };
            match got {
                Ok(e) => panic!("{EXPECTED_ERROR} {e:?}, {desc}: {case}"),
                Err(e) => assert_eq!(want.to_string(), e.to_string(), "{desc}: {case}"),
            };
        }
    }

    #[sqlx::test(fixtures("../../fixtures/tags.sql"))]
    async fn test_new_name_validations(conn: PgPool) {
        let desc = "Test target validation arguments";
        let test_cases = [
            (
                REPEATING,
                Some(ClientError::RepeatArgs("writing".into())),
                "repeating",
            ),
            (NOT_IN_DB, None, "valid new names"),
            (
                IN_DB,
                Some(ClientError::UniqueConstraint(
                    TABLE.into(),
                    "reading, writing".into(),
                )),
                "new names in db",
            ),
        ];
        for (args, want, case) in test_cases {
            let args = args.iter().map(|f| f.to_string()).collect();
            let got = validate_new_names(&conn, args).await;
            let want = match want {
                Some(e) => Status::from(e),
                None => {
                    assert!(got.is_ok(), "{desc}: {case}");
                    continue;
                }
            };
            match got {
                Ok(e) => panic!("{EXPECTED_ERROR} {e:?}, {desc}: {case}"),
                Err(e) => assert_eq!(want.to_string(), e.to_string(), "{desc}: {case}"),
            };
        }
    }

    #[sqlx::test(fixtures("../../fixtures/tags.sql"))]
    async fn test_update(conn: PgPool) -> anyhow::Result<()> {
        let args = IN_DB
            .iter()
            .zip(NOT_IN_DB)
            .map(|(target, new_name)| TagUpdate {
                target: target.to_string(),
                new_name: new_name.to_string(),
            })
            .collect();

        let got = tag_update(&conn, args)
            .await?
            .into_iter()
            .map(|f| f.name)
            .collect::<Vec<String>>();

        got.into_iter().zip(NOT_IN_DB).for_each(|(got, want)| {
            assert_eq!(want, &got);
        });
        anyhow::Ok(())
    }
}
