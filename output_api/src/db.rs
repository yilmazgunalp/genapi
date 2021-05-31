use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use r2d2;
use r2d2_diesel;
use std;

pub type ConnectionPool = r2d2::Pool<r2d2_diesel::ConnectionManager<diesel::pg::PgConnection>>;

pub fn establish_connection_pool() -> ConnectionPool {
    let connection_manager = r2d2_diesel::ConnectionManager::<diesel::pg::PgConnection>::new(
        std::env::var("DATABASE_URL").unwrap(),
    );
    r2d2::Pool::builder()
        .max_size(10)
        .build(connection_manager)
        .unwrap()
}

pub fn establish_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    PgConnection::establish(&database_url).unwrap()
}

