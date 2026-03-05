use thiserror::Error;

pub mod prelude {
    pub use super::ClientError;
}

#[derive(Debug, Error)]
pub enum ClientError {
    /// For operations that use env variables
    /// for configuration.
    #[error("Environmet variable {0} unset")]
    UnsetEnvVar(&'static str),
    #[error("Can't parse {1}, as type {0}")]
    InvalidConfig(&'static str, String),
}
