use std::str::FromStr;

use tonic::transport::Endpoint;

use crate::prelude::*;

pub mod prelude {
    pub use super::Config;
}

#[derive(Debug)]
pub struct Config {
    pub endpoint: Endpoint,
}

impl Config {
    pub fn init() -> Result<Self, ClientError> {
        let key = "HABITING_URI";
        let addr = std::env::var(key).map_err(|_| ClientError::UnsetEnvVar(key))?;
        let addr = format!("http://{addr}");

        let endpoint =
            Endpoint::from_str(&addr).map_err(|_| ClientError::InvalidConfig("Endpoint", addr))?;

        Ok(Self { endpoint })
    }
}
