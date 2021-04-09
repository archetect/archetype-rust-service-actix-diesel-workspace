#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };
use std::env;

pub mod models;
pub mod schema;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PgPool {
    dotenv::dotenv().ok();

    let database_url = env::var("{{ ARTIFACT_ID }}_DATABASE_URL")
        .expect("{{ ARTIFACT_ID }}_DATABASE_URL must be set");
    init_pool(&database_url).expect("Error loading PgPool")
}
