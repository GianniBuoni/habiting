use std::fmt::Display;

use super::*;

impl SessionService {
    pub(super) async fn handle_start(tag_name: impl Display) -> anyhow::Result<()> {
        let request = habiting_proto::SessionStartRequest {
            tag: tag_name.to_string(),
        };

        let res = SessionService::connect()
            .await?
            .session_start(request)
            .await
            .map_err(ServerError)?
            .into_inner()
            .start_time;

        println!("{tag_name} session sucessfully started at: '{res}'");
        Ok(())
    }
}
