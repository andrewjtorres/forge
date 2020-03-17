use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};

pub fn connect(database_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Cannot directly establish a pooled connection")
}
