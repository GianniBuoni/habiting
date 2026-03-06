use clap::{Args, Parser, Subcommand};

pub mod prelude {
    pub use super::{Cli, Service, SessionArgs, TagArgs};
    pub(crate) use super::{SessionActions, TagActions};
}

#[derive(Debug, Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub service: Service,
}

#[derive(Debug, Subcommand)]
pub enum Service {
    /// Manage current sessions
    #[command(alias = "s")]
    Sessions(SessionArgs),
    /// Manage available session tags
    #[command(alias = "t")]
    Tags(TagArgs),
}

#[derive(Debug, Args)]
pub struct SessionArgs {
    #[command(subcommand)]
    pub action: SessionActions,
}

#[derive(Debug, Subcommand)]
pub enum SessionActions {
    /// Lists currently active sessions
    #[command(alias = "ls")]
    List,
    /// Starts a new session
    #[command(alias = "add")]
    Start { tag_name: String },
    /// Stops a target session
    #[command(alias = "end")]
    Stop { tag_name: String },
}

#[derive(Debug, Args)]
pub struct TagArgs {
    #[command(subcommand)]
    pub action: TagActions,
}

#[derive(Debug, Subcommand)]
pub enum TagActions {
    /// Creates a new session tag
    #[command(alias = "add")]
    Create(TargetArgs),
    /// Lists all available session tags
    #[command(alias = "ls")]
    List,
    /// Change the name of a session tag
    #[command(alias = "edit")]
    Update(UpdateArgs),
    /// Deletes a session tag and associated sessions
    #[command(alias = "rm")]
    Delete(TargetArgs),
}

#[derive(Debug, Args)]
pub struct TargetArgs {
    /// List of sesion tag names
    #[arg(long, short, value_delimiter = ',')]
    pub targets: Vec<String>,
}

#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Original names of session tags
    #[arg(long, short, value_delimiter = ',')]
    pub targets: Vec<String>,
    /// New name for target session tags
    #[arg(long, short, value_delimiter = ',')]
    pub new_names: Vec<String>,
}
