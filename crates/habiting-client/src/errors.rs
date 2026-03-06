use std::fmt::Display;

use thiserror::Error;

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
}

#[derive(Debug, Error)]
/// Errors returned from the server
pub struct ServerError(pub tonic::Status);

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n\n{}.\n{}.", self.0.code(), self.0.message())
    }
}
