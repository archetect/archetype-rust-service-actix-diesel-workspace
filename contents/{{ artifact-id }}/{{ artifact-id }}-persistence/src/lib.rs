#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use url::Url;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

pub mod models;
pub mod schema;
pub mod settings;
pub mod tempdb;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct {{ArtifactId}}Persistence {
    pool: PgPool,
}

impl {{ArtifactId}}Persistence {
    pub fn new() -> Result<{{ArtifactId}}Persistence, Box<dyn std::error::Error>> {
        {{ArtifactId}}Persistence::new_with_settings(&settings::PersistenceSettings::default())
    }

    pub fn new_with_settings(
        settings: &settings::PersistenceSettings,
    ) -> Result<{{ArtifactId}}Persistence, Box<dyn std::error::Error>> {
        let database_url = Url::parse(settings.database().url()).unwrap();
        tempdb::TEMP_DATABASES.with(|sm| {
            let tempdb = tempdb::get_tempdb_url(&database_url);
            sm.borrow_mut().add_database(tempdb);
        });

        let pool = init_pool(database_url.as_str())?;
        Ok({{ArtifactId}}Persistence { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}
