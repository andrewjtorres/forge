pub mod pool;

use crate::error::{Error, Result};
use pool::{Pool, PooledConnection};

pub fn connect(pool: &Pool) -> Result<PooledConnection> {
    Ok(pool.get().map_err(|_| Error::UnableToConnectToDatabase)?)
}
