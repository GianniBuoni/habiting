use std::fmt::Display;

use super::*;

impl SessionService {
    pub(super) async fn handle_stop(tag_name: impl Display) -> anyhow::Result<()> {
        let request = habiting_proto::SessionStopRequest {
            tag: tag_name.to_string(),
        };

        let res = SessionService::connect()
            .await?
            .session_stop(request)
            .await
            .map_err(ServerError)?
            .into_inner();

        println!(
            "{tag_name} session ended at: '{}' and lasted for {} minutes",
            res.end_time, res.duration
        );
        Ok(())
    }
}
