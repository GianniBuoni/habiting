use std::{net::SocketAddr, time::Duration};

use sqlx::PgPool;
use tokio::sync::OnceCell;

use crate::prelude::*;

static CONFIG: OnceCell<Config> = OnceCell::const_new();

pub mod prelude {
    pub use super::{Config, DBConn};
}

pub struct Config {
    pub db_conn: DBConn,
    pub endpoint: Endpoint,
}

/// Singleton containing static server configurations information.
impl Config {
    async fn init() -> Result<Self, Status> {
        info!("Inializing server configuration");
        let config = Config {
            db_conn: DBConn::try_init().await?,
            endpoint: Endpoint::try_init()?,
        };
        info!("Server sucessfully configured");
        Ok(config)
    }
    pub async fn try_get() -> Result<&'static Self, Status> {
        CONFIG.get_or_try_init(Config::init).await
    }
}

/// Newtype wrapper around a database connection pool.
/// Use the Config struct to get the interior connection.
pub struct DBConn(PgPool);

impl DBConn {
    /// Context deadline for sql transactions
    pub fn context() -> Duration {
        Duration::from_secs(10)
    }
    pub fn get(&'static self) -> &'static PgPool {
        &self.0
    }
    /// Itinializes the database connections
    async fn try_init() -> Result<Self, Status> {
        info!("Initializing database connection");

        let key = "DATABASE_URL";

        debug!("Checking for vaiable {key}");
        let url = std::env::var(key).map_err(|_| ServerError::EnvVarUnset(key))?;

        debug!("Establishing inital database connection");
        let pool = PgPool::connect(&url)
            .await
            .map_err(|_| DbError::Connection(url.into()))?;

        info!("Database initialization sucessfull");
        Ok(DBConn(pool))
    }
}

pub struct Endpoint(SocketAddr);

impl Endpoint {
    pub fn get(&self) -> SocketAddr {
        self.0
    }
    fn try_init() -> Result<Self, Status> {
        info!("Initializing server endppoint");

        let key = "HABITING_URI";

        debug!("Checking for variable {key}");
        let uri = std::env::var(key).map_err(|_| ServerError::EnvVarUnset(key))?;

        debug!("Attempting to parse URI string as socket address");
        let uri = uri.parse().map_err(|_| ServerError::Config("endpoint"))?;

        info!("Server endpoint configured");
        Ok(Self(uri))
    }
}
