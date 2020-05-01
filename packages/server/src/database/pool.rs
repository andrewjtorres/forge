use diesel::{pg::PgConnection, r2d2};

type ConnectionManager = r2d2::ConnectionManager<PgConnection>;

pub type Pool = r2d2::Pool<ConnectionManager>;

pub type PooledConnection = r2d2::PooledConnection<ConnectionManager>;

pub fn connect(database_url: &str) -> Pool {
    let manager = ConnectionManager::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Unable to establish a pooled connection")
}
