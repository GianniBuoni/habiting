pub(crate) mod habiting_proto {
    tonic::include_proto!("habiting");
}
// internal prelude exports
pub use super::cli::prelude::*;
pub(crate) use super::config::prelude::*;
pub(crate) use super::errors::prelude::*;
pub use super::session_service::prelude::*;
