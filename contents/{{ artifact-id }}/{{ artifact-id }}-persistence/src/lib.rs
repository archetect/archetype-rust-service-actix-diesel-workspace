#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, PoolError};
use tracing::debug;
use url::Url;

use crate::settings::DatabaseSettings;

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
        let mut database_url = Url::parse(settings.database().url()).unwrap();

        if let Some(temporary) = settings.tempdb() {
            database_url = tempdb::get_tempdb_url(&database_url);
            let temp_settings = DatabaseSettings::default().with_url(&database_url);
            tempdb::create_database_if_not_exists(&temp_settings)?;
            tempdb::database_migrate(&temp_settings)?;

            if temporary == &settings::TemporaryType::Drop {
                tempdb::TEMP_DATABASES.with(|sm| {
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
