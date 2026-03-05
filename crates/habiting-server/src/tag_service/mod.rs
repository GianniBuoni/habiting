use crate::prelude::{
    habiting_proto::{
        TagCreateRequest, TagCreateResponse, TagDeleteRequest, TagDeleteResponse, TagListRequest,
        TagListResponse, TagUpdateRequest, TagUpdateResponse, tag_service_server::TagService,
    },
    *,
};

mod tag_create;
mod tag_delete;
mod tag_list;
mod tag_row;
mod tag_update;

pub mod prelude {
    pub use super::HabitingTagService;
    pub use super::habiting_proto::tag_service_server::TagServiceServer;
}

#[derive(Default)]
pub struct HabitingTagService {}

#[tonic::async_trait]
impl TagService for HabitingTagService {
    async fn tag_create(
        &self,
        req: Request<TagCreateRequest>,
    ) -> Result<Response<TagCreateResponse>, Status> {
        self.handle_create(req).await
    }
    async fn tag_delete(
        &self,
        req: Request<TagDeleteRequest>,
    ) -> Result<Response<TagDeleteResponse>, Status> {
        self.handle_delete(req).await
    }
    async fn tag_list(
        &self,
        req: Request<TagListRequest>,
    ) -> Result<Response<TagListResponse>, Status> {
        self.handle_list(req).await
    }
    async fn tag_update(
        &self,
        req: Request<TagUpdateRequest>,
    ) -> Result<Response<TagUpdateResponse>, Status> {
        self.handle_update(req).await
    }
}
