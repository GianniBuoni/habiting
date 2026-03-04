use std::ops::Sub;

use sqlx::{
    PgExecutor,
    types::{
        Uuid,
        chrono::{DateTime, Local},
    },
};

use super::*;

impl HabitingSessionService {
    pub(super) async fn handle_stop(
        &self,
        req: Request<SessionStopRequest>,
    ) -> Result<Response<SessionStopResponse>, Status> {
        let req = req.into_inner();
        info!("{req:?}");

        let conn = Config::try_get().await?.db_conn.get();
        let tag = req.tag;

        let (start_time, end_time) = tokio::time::timeout(DBConn::context(), async {
            // begin transcation
            let mut tx = conn.begin().await.map_err(DbError::Sql)?;
            // run validations
            validate_stop(&mut *tx, &tag).await?;
            let session = validate_active_session(&mut *tx, &tag).await?;
            // make updates
            let date_times = session_stop(&mut *tx, session).await?;
            // commit transaction
            tx.commit().await.map_err(DbError::Sql)?;
            Ok::<(DateTime<Local>, DateTime<Local>), Status>(date_times)
        })
        .await
        .map_err(|_| DbError::Context("session_stop"))??;

        let duration = end_time.sub(start_time).num_minutes().unsigned_abs();
        let end_time = end_time.to_string();

        Ok(Response::new(SessionStopResponse { end_time, duration }))
    }
}

/// Use Argument valodator to make sure target tag exists.
async fn validate_stop(conn: impl PgExecutor<'_>, tag: &str) -> Result<(), Status> {
    let validate = ArgumentsBuilder::new([tag.into()].into())
        .with_table("tags")
        .with_column("name")
        .with_task("session_stop")
        .try_build()?;

    Ok(validate
        .try_check_empty_args()?
        .try_check_entry_exists(conn)
        .await?)
}

/// Table specific validation that ensures that there is only one session
/// in the database for a given tag has a NULL ended_at column.
/// Returns the Uuid of the current valid active session.
async fn validate_active_session(conn: impl PgExecutor<'_>, tag: &str) -> Result<Uuid, Status> {
    let active_session = sqlx::query_scalar!("SELECT uuid FROM sessions WHERE tag_id IN (SELECT uuid FROM tags WHERE name = $1) AND ended_at IS NULL", tag).fetch_all(conn).await.map_err(DbError::Sql)?;

    if active_session.len() == 1 {
        return Ok(*active_session.first().unwrap());
    }
    match active_session.is_empty() {
        true => Err(ClientError::EntryNotFound("sessions".into(), tag.into()).into()),
        false => Err(ClientError::UniqueConstraint("sessions".into(), tag.into()).into()),
    }
}

/// Updates ended_at column for a valid session.
async fn session_stop(
    conn: impl PgExecutor<'_>,
    session: Uuid,
) -> Result<(DateTime<Local>, DateTime<Local>), DbError> {
    let now = Local::now();
    let start_time = sqlx::query_scalar!(
        "UPDATE sessions SET ended_at = $1 WHERE uuid = $2 RETURNING created_at",
        now,
        session
    )
    .fetch_one(conn)
    .await?;

    Ok((start_time.with_timezone(&Local), now))
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::*;

    /// get session uuids using two statements and w/o validations
    async fn get_session(conn: &PgPool, tag: &str) -> Result<Uuid, DbError> {
        let tag_id = sqlx::query_scalar!("SELECT uuid FROM tags WHERE name = $1", tag)
            .fetch_one(conn)
            .await?;
        let session_id = sqlx::query_scalar!("SELECT uuid FROM sessions WHERE tag_id = $1", tag_id)
            .fetch_one(conn)
            .await?;

        Ok(session_id)
    }

    #[sqlx::test(fixtures("../../fixtures/basic.sql"))]
    async fn test_validate_stop(conn: PgPool) {
        let desc = "Test session_stop validation function";
        let test_cases = [("reading", true), ("music", false), ("writing", true)];
        for (tag, should_ok) in test_cases {
            let got = validate_stop(&conn, tag).await;
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
    async fn test_validate_session(conn: PgPool) -> anyhow::Result<()> {
        let desc = "Test session validation functions return expected results";
        let test_cases = [("reading", true), ("writing", true), ("grading", false)];

        for (tag, should_ok) in test_cases {
            let got = validate_active_session(&conn, tag).await;
            if should_ok {
                let want = get_session(&conn, tag).await?;
                assert_eq!(want, got?, "{tag}: {desc}");
                continue;
            }
            match got {
                Ok(e) => panic!("{EXPECTED_ERROR} {e}, {tag}: {desc}"),
                Err(e) => {
                    let want =
                        Status::from(ClientError::EntryNotFound("sessions".into(), tag.into()))
                            .to_string();
                    assert_eq!(want.to_string(), e.to_string());
                }
            }
        }
        anyhow::Ok(())
    }

    #[sqlx::test(fixtures("../../fixtures/basic.sql"))]
    async fn test_session_stop(conn: PgPool) -> anyhow::Result<()> {
        let desc = "Test session_stop function returns ok or error.";
        let test_cases = [
            ("reading", None),
            ("writing", None),
            ("grading", Some(Uuid::nil())),
        ];
        for (tag, should_fail) in test_cases {
            let session = match should_fail {
                Some(uuid) => uuid,
                None => get_session(&conn, tag).await?,
            };
            let got = session_stop(&conn, session).await;

            if should_fail.is_none() {
                dbg!(&got);
                assert!(got.is_ok(), "{tag}: {desc}");
                continue;
            }
            match got {
                Ok(e) => panic!("{EXPECTED_ERROR} {e:?}, {desc}"),
                Err(e) => continue,
            }
        }
        anyhow::Ok(())
    }
}
