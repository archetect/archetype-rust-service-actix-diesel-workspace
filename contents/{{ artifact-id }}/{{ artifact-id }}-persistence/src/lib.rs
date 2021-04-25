#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, PoolError};
use tracing::debug;

use crate::settings::DatabaseSettings;

pub mod models;
pub mod schema;
pub mod settings;
pub mod database;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct {{ArtifactId}}Persistence {
    pool: PgPool,
}

impl {{ArtifactId}}Persistence {
    pub fn new(
        settings: &settings::PersistenceSettings,
    ) -> Result<{{ArtifactId}}Persistence, Box<dyn std::error::Error>> {
        let mut database_url = settings.database().url().clone();

        if let Some(temporary) = settings.temporary() {
            database_url = database::get_tempdb_url(&database_url);
            let temp_settings = DatabaseSettings::default().with_url(&database_url);
            database::init(&temp_settings)?;
            database::migrate(&temp_settings)?;

            if temporary == &settings::TemporaryType::Drop {
                database::TEMP_DATABASES.with(|sm| {
                    debug!("Registering database for drop");
                    sm.borrow_mut().add_database(database_url.clone());
                });
            }
        }

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
