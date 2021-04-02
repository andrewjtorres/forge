use diesel::{pg::PgConnection, r2d2};

type ConnectionManager = r2d2::ConnectionManager<PgConnection>;

pub type Pool = r2d2::Pool<ConnectionManager>;

pub type PooledConnection = r2d2::PooledConnection<ConnectionManager>;

pub fn create(database_url: &str) -> Pool {
    Pool::builder()
        .build(ConnectionManager::new(database_url))
        .expect("Unable to create a connection pool")
}
