use sqlx::{
    PgExecutor,
    types::chrono::{DateTime, Local},
};

use super::*;

impl HabitingSessionService {
    pub(super) async fn handle_start(
        &self,
        req: Request<SessionStartRequest>,
    ) -> Result<Response<SessionStartResponse>, Status> {
        let req = req.into_inner();
        info!("{req:?}");

        let conn = Config::try_get().await?.db_conn.get();
        let tag = req.tag;

        let start_time = tokio::time::timeout(DBConn::context(), async {
            // begin transaction
            let mut tx = conn.begin().await.map_err(DbError::Sql)?;
            // run validations
            validate_start(&mut *tx, &tag).await?;
            validate_active_session(&mut *tx, &tag).await?;
            // make insertion
            let date_time = session_start(&mut *tx, &tag).await?;
            // commit transaction
            tx.commit().await.map_err(DbError::Sql)?;
            Ok::<DateTime<Local>, Status>(date_time)
        })
        .await
        .map_err(|_| DbError::Context("session_start"))??;

        let start_time = start_time.to_string();
        Ok(Response::new(SessionStartResponse { start_time }))
    }
}

/// Uses the Arguments validator to ensure that argument is:
/// - not empty
/// - that the target tag already exists
async fn validate_start(conn: impl PgExecutor<'_>, tag: &str) -> Result<(), Status> {
    let validate = ArgumentsBuilder::new([tag.into()].into())
        .with_table("tags")
        .with_column("name")
        .with_task("session_start")
        .try_build()?;

    Ok(validate
        .try_check_empty_args()?
        .try_check_entry_exists(conn)
        .await?)
}

/// Table specific validation to ensure that no other session has a NULL
/// ended_at column.
/// There should only be one active session available at a time.
async fn validate_active_session(conn: impl PgExecutor<'_>, tag: &str) -> Result<(), Status> {
    let validate = sqlx::query_scalar!("SELECT ended_at FROM sessions WHERE tag_id IN (SELECT uuid FROM tags WHERE name = $1) AND ended_at IS NULL", tag).fetch_all(conn).await.map_err(DbError::Sql)?;

    match validate.is_empty() {
        true => Ok(()),
        false => Err(ClientError::UniqueConstraint("sessions".into(), tag.into()).into()),
    }
}

/// Starts the session by inserting a new row.
/// Should be called after all the validation functions.
async fn session_start(conn: impl PgExecutor<'_>, tag: &str) -> Result<DateTime<Local>, DbError> {
    let start_time = sqlx::query_scalar!(
        "INSERT INTO sessions (tag_id) SELECT uuid AS tag_id FROM tags WHERE name = $1 RETURNING created_at",
        tag
    ).fetch_one(conn).await?;

    Ok(start_time.with_timezone(&Local))
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::*;

    #[sqlx::test(fixtures("../../fixtures/tags.sql"))]
    async fn test_session_start(conn: PgPool) {
        let desc = "Test basic session start request returns timestamp";
        let test_cases = [("reading", true), ("writing", true), ("music", false)];
        for (tag, should_ok) in test_cases {
            let got = session_start(&conn, tag).await;

            if should_ok {
                // threre's not a great way to compare want values
                // since time values are are handled by the database
                assert!(got.is_ok(), "{tag}: {desc}");
                continue;
            }
            match got {
                Ok(e) => panic!("{EXPECTED_ERROR} {e}, {tag}: {desc}"),
                Err(_) => assert!(got.is_err(), "{tag}: {desc}"),
            }
        }
    }

    #[sqlx::test(fixtures("../../fixtures/basic.sql"))]
    async fn test_tag_deletion_cascade(conn: PgPool) -> anyhow::Result<()> {
        let desc = "Test if deleting referenced tags will delete the session info as well";
        let err = "DB count failed somehow";
        // assert that test fixture has expected amount of sessions
        let count = sqlx::query_scalar!("SELECT COUNT(*) AS count FROM sessions")
            .fetch_one(&conn)
            .await?
            .ok_or(anyhow::Error::msg(err))?;
        assert_eq!(2, count, "Control assertion: {desc}");
        // test cases
        let test_cases = [("reading", 1), ("writing", 0)];
        for (tag, want) in test_cases {
            sqlx::query!("DELETE FROM tags WHERE name=$1", tag)
                .execute(&conn)
                .await?;
            let got = sqlx::query_scalar!("SELECT COUNT(*) AS count FROM sessions")
                .fetch_one(&conn)
                .await?
                .ok_or(anyhow::Error::msg(err))?;
            assert_eq!(want, got, "{tag}: {desc}");
        }
        anyhow::Ok(())
    }

    #[sqlx::test(fixtures("../../fixtures/tags.sql"))]
    async fn test_tag_not_found(conn: PgPool) {
        let desc = "Test validation issue returns correct ClientError.";
        let test_cases = [("reading", true), ("music", false), ("writing", true)];
        for (tag, should_ok) in test_cases {
            let got = validate_start(&conn, tag).await;
            if should_ok {
                assert!(got.is_ok(), "{tag}: {desc}");
                continue;
            }
            match got {
                Ok(e) => panic!("{EXPECTED_ERROR} {e:?}, {tag}: {desc}"),
                Err(e) => {
                    let want = Status::from(ClientError::EntryNotFound("tags".into(), tag.into()))
                        .to_string();
                    assert_eq!(want, e.to_string(), "{tag}: {desc}");
                }
            }
        }
    }

    #[sqlx::test(fixtures("../../fixtures/basic.sql"))]
    async fn test_session_collision(conn: PgPool) {
        let desc = "Test if active sessions will result in validation errors.";
        let test_cases = [("reading", false), ("writing", false), ("grading", true)];
        for (tag, should_ok) in test_cases {
            let got = validate_active_session(&conn, tag).await;
            if should_ok {
                assert!(got.is_ok(), "{tag}: {desc}");
                continue;
            }
            match got {
                Ok(e) => panic!("{EXPECTED_ERROR} {e:?}, {tag}: {desc}"),
                Err(e) => {
                    let want =
                        Status::from(ClientError::UniqueConstraint("sessions".into(), tag.into()))
                            .to_string();
                    assert_eq!(want, e.to_string(), "{tag}: {desc}");
                }
            }
        }
    }
}
