use clap::Parser;
use habiting_client::prelude::*;

fn main() {
    let cli = Cli::parse();

    match cli.service {
        Service::Sessions(session_args) => {
            dbg!(session_args);
        }
        Service::Tags(tag_args) => {
            dbg!(tag_args);
        }
    }
}
