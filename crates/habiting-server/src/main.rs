use habiting_server::prelude::*;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    let addr = Config::try_get().await?.endpoint.get();

    info!("Serving at: {addr}");
    Server::builder()
        .add_service(TagServiceServer::new(HabitingTagService::default()))
        .add_service(SessionServiceServer::new(HabitingSessionService::default()))
        .serve(addr)
        .await?;

    Ok(())
}
