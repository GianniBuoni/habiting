use crate::prelude::*;

use habiting_proto::session_service_client::SessionServiceClient;
use tonic::transport::Channel;

pub mod prelude {
    pub use super::SessionService;
}

mod session_start;
mod session_stop;

pub struct SessionService {}

impl SessionService {
    async fn connect() -> Result<SessionServiceClient<Channel>, ClientError> {
        let dst = Config::init()?.endpoint;
        SessionServiceClient::connect(dst)
            .await
            .map_err(ClientError::Connection)
    }
    pub async fn handle_action(action: SessionActions) -> anyhow::Result<()> {
        match action {
            SessionActions::List => todo!(),
            SessionActions::Start { tag_name } => SessionService::handle_start(tag_name).await,
            SessionActions::Stop { tag_name } => SessionService::handle_stop(tag_name).await,
        }
    }
}
