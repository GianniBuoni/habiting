use clap::Parser;
use habiting_client::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.service {
        Service::Sessions(session_args) => SessionService::handle_action(session_args.action).await,
        Service::Tags(tag_args) => TagService::handle_action(tag_args.action).await,
    }
}
