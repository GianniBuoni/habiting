use tonic::transport::Channel;

use crate::prelude::{habiting_proto::tag_service_client::TagServiceClient, *};

pub mod prelude {
    pub use super::TagService;
}

mod tag_create;
mod tag_delete;
mod tag_list;
mod tag_update;

pub struct TagService {}

impl TagService {
    async fn connect() -> Result<TagServiceClient<Channel>, ClientError> {
        let dst = Config::init()?.endpoint;
        TagServiceClient::connect(dst)
            .await
            .map_err(ClientError::Connection)
    }
    pub async fn handle_action(action: TagActions) -> anyhow::Result<()> {
        match action {
            TagActions::Create(target_args) => TagService::tag_create(target_args.targets).await,
            TagActions::List => TagService::tag_list().await,
            TagActions::Update(update_args) => TagService::tag_update(update_args).await,
            TagActions::Delete(target_args) => TagService::tag_delete(target_args.targets).await,
        }
    }
}
