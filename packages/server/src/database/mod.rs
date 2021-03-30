pub mod pool;

use crate::error::{Error, Result};
use pool::{Pool, PooledConnection};

pub fn connect(pool: &Pool) -> Result<PooledConnection> {
    pool.get().map_err(|_| Error::UnableToConnectToDatabase)
}
