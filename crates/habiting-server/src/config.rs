use std::time::Duration;

use sqlx::PgPool;
use tokio::sync::OnceCell;

use crate::prelude::*;

static CONFIG: OnceCell<Config> = OnceCell::const_new();

pub mod prelude {
    pub use super::{Config, DBConn};
}

pub struct Config {
    pub db_conn: DBConn,
}

/// Singleton containing static server configurations information.
impl Config {
    async fn init() -> Result<Self, Status> {
        let config = Config {
            db_conn: DBConn::try_init().await?,
        };
        Ok(config)
    }
    pub async fn try_get() -> Result<&'static Self, Status> {
        CONFIG.get_or_try_init(Config::init).await
    }
}

#[allow(dead_code)]
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
        let key = "DATABASE_URL";
        let url = std::env::var(key).map_err(|_| ServerError::EnvVarUnset(key))?;
        let pool = PgPool::connect(&url)
            .await
            .map_err(|_| DbError::Connection(url.into()))?;

        Ok(DBConn(pool))
    }
}
