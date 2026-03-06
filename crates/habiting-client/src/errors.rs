use std::fmt::Display;

use thiserror::Error;

use crate::cli::UpdateArgs;

pub mod prelude {
    pub use super::{ClientError, ServerError};
}

#[derive(Debug, Error)]
pub enum ClientError {
    /// For operations that use env variables
    /// for configuration.
    #[error("Environmet variable {0} unset")]
    UnsetEnvVar(&'static str),
    #[error("Can't parse {1}, as type {0}")]
    InvalidConfig(&'static str, String),
    #[error("Issue connecting to endpoint")]
    Connection(#[from] tonic::transport::Error),
    #[error("Targets and new names must be of equal length:\n\nTargets:    {}\nNew names:  {}", .0.targets.join(", "), .0.new_names.join(", "))]
    UnequalArgs(UpdateArgs),
}

#[derive(Debug, Error)]
/// Errors returned from the server
pub struct ServerError(pub tonic::Status);

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n\n{}.\n{}.", self.0.code(), self.0.message())
    }
}
