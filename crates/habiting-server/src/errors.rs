use tonic::Status;

pub mod prelude {
    pub use super::ServerError;
}

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    /// General configuration issue.
    #[error("Issuse with {0} configuration")]
    Config(&'static str),
    /// Enviromnet variable required for configuration
    /// is missing or unset.
    #[error("Environment variable {0} is missing")]
    EnvVarUnset(&'static str),
}

impl From<ServerError> for Status {
    fn from(value: ServerError) -> Self {
        Status::internal(value.to_string())
    }
}
