use clap::Parser;
use habiting_client::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.service {
        Service::Sessions(session_args) => {
            dbg!(session_args);
        }
        Service::Tags(tag_args) => {
            dbg!(tag_args);
        }
    }
    Ok(())
}
