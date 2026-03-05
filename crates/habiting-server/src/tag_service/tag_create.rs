use std::sync::Arc;

use sqlx::{PgExecutor, Postgres, QueryBuilder};

use super::{tag_row::TagPgRow, *};

impl HabitingTagService {
    pub(super) async fn handle_create(
        &self,
        req: Request<TagCreateRequest>,
    ) -> Result<Response<TagCreateResponse>, Status> {
        let req = req.into_inner();
        info!("{req:?}");

        let conn = Config::try_get().await?.db_conn.get();
        let args = Arc::<[String]>::from(req.names);

        let tags = tokio::time::timeout(DBConn::context(), async {
            let mut tx = conn.begin().await.map_err(DbError::Sql)?;
            validate_create(&mut *tx, args.clone()).await?;
            let tags = tag_create(&mut *tx, args).await?;
            tx.commit().await.map_err(DbError::Sql)?;

            Ok::<Vec<TagPgRow>, Status>(tags)
        })
        .await
        .map_err(|_| DbError::Context("tag_create"))??
        .into_iter()
        .map(habiting_proto::Tag::from)
        .collect();

        Ok(Response::new(TagCreateResponse { tags }))
    }
}

async fn validate_create(conn: impl PgExecutor<'_>, args: Arc<[String]>) -> Result<(), Status> {
    let validation = Arguments::builder(args)
        .with_table("tags")
        .with_column("name")
        .with_task("task_create")
        .try_build()?;

    Ok(validation
        .try_check_empty_args()?
        .try_check_repeated_args()?
        .try_check_unique_constraint(conn)
        .await?)
}

async fn tag_create(
    conn: impl PgExecutor<'_>,
    args: Arc<[String]>,
) -> Result<Vec<TagPgRow>, DbError> {
    let mut q = QueryBuilder::<Postgres>::new("INSERT INTO tags (name) ");
    q.push_values(args.iter().take(PG_BIND_LIMIT), |mut b, name| {
        b.push_bind(name);
    });
    q.push(" RETURNING *");

    let rows = q.build_query_as::<TagPgRow>().fetch_all(conn).await?;
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::*;

    #[sqlx::test(fixtures("../../fixtures/tags.sql"))]
    async fn test_validation(conn: PgPool) {
        let desc = "Test validation of";
        let test_cases = [
            (
                vec!["reading", "reading", "writing"],
                Some(ClientError::RepeatArgs("reading".into())),
                "repeat args",
            ),
            (
                vec!["reading", "writing", "music"],
                Some(ClientError::UniqueConstraint(
                    "tags".into(),
                    "reading, writing".into(),
                )),
                "non-unique args",
            ),
            (vec!["music", "meetings"], None, "valid args"),
        ];
        for (args, err, case) in test_cases {
            let args = args.iter().map(|f| f.to_string()).collect();
            let got = validate_create(&conn, args).await;
            let want = match err {
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

    #[sqlx::test]
    async fn test_create(conn: PgPool) {
        let desc = "Test ceating various lengths of tag args";
        let test_cases = [
            (vec!["meetings"], 1),
            (vec!["music", "gardening"], 2),
            (vec!["cooking", "cleaning", "tutoring"], 3),
        ];
        for (args, want) in test_cases {
            let args = args.iter().map(|f| f.to_string()).collect();
            let got = tag_create(&conn, args).await;

            match got {
                Ok(rows) => assert_eq!(want, rows.len(), "{desc}: {want}"),
                Err(e) => panic!("{desc}: {want}. {e}"),
            }
        }
    }
}
