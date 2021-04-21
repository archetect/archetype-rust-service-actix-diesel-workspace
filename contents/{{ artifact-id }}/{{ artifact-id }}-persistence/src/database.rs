use std::cell::RefCell;

use diesel::*;
use diesel::backend::Backend;
use diesel::query_builder::{AstPass, QueryFragment, QueryId};
use tracing::info;
use url::Url;

use crate::settings::DatabaseSettings;

embed_migrations!();

thread_local! {
    pub static TEMP_DATABASES: RefCell<TempDatabases> = RefCell::new(TempDatabases::new());
}

table! {
    pg_database (datname) {
        datname -> Text,
        datistemplate -> Bool,
    }
}

pub fn init(database_settings: &DatabaseSettings) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(database_name) = get_database_name(database_settings.url()) {
        let admin_url = get_admin_url(database_settings.url());
        let conn = PgConnection::establish(admin_url.as_str())?;
        if !database_exists(&conn, &database_name)? {
            CreateDatabase::with_name(&database_name).execute(&conn)?;
            info!("'{}' database created", database_name);
        } else {
            info!("'{}' database already exists", database_name);
        }
    }
    Ok(())
}

pub fn migrate(database_settings: &DatabaseSettings) -> Result<(), Box<dyn std::error::Error>> {
    let conn = PgConnection::establish(database_settings.url().as_str())?;
    embedded_migrations::run(&conn)?;
    Ok(())
}

fn database_exists(conn: &PgConnection, database_name: &str) -> QueryResult<bool> {
    use self::pg_database::dsl::*;

    pg_database
        .select(datname)
        .filter(datname.eq(database_name))
        .filter(datistemplate.eq(false))
        .get_result::<String>(conn)
        .optional()
        .map(|x| x.is_some())
}

pub struct TempDatabases {
    database_urls: Vec<Url>,
}

impl TempDatabases {
    pub fn new() -> TempDatabases {
        TempDatabases {
            database_urls: vec![],
        }
    }

    pub fn add_database(&mut self, database_url: Url) {
        self.database_urls.push(database_url);
    }

    pub fn database_urls(&self) -> &[Url] {
        self.database_urls.as_ref()
    }
}

impl Drop for TempDatabases {
    fn drop(&mut self) {
        for database_url in self.database_urls() {
            if let Some(database_name) = get_database_name(database_url) {
                let admin_url = get_admin_url(database_url);
                match PgConnection::establish(admin_url.as_str()) {
                    Ok(conn) => {
                        if let Err(err) = DropDatabase::with_name(&database_name).execute(&conn) {
                            eprintln!("Failed to drop temp database '{}': {}", database_name, err);
                        }
                    }
                    Err(error) => {
                        eprintln!("Error establishing a connection while attempting to drop database '{}': {}",
                                  database_name, error);
                    }
                }
            }
        }
    }
}

pub fn get_database_name(url: &Url) -> Option<String> {
    url.path_segments()?.last().map(|v| v.into())
}

pub fn get_admin_url(url: &Url) -> Url {
    let mut url = url.clone();
    url.set_path("postgres");
    url
}

pub fn get_tempdb_url(url: &Url) -> Url {
    let mut url = url.clone();
    let id = tempdb_id();
    if let Some(database_name) = &mut get_database_name(&url) {
        database_name.push_str("_");
        database_name.push_str(&id);
        url.set_path(&database_name);
    } else {
        url.set_path(&id);
    }
    url
}

fn tempdb_id() -> String {
    let id = chrono::Utc::now().timestamp_nanos() as u64;
    harsh::Harsh::default().encode(&[id])
}


struct CreateDatabase {
    database_name: String,
}

impl CreateDatabase {
    pub fn with_name<T: Into<String>>(database_name: T) -> Self {
        CreateDatabase {
            database_name: database_name.into(),
        }
    }
}

impl<DB: Backend> QueryFragment<DB> for CreateDatabase {
    fn walk_ast(&self, mut sql: AstPass<DB>) -> QueryResult<()> {
        sql.push_sql("create database ");
        sql.push_identifier(self.database_name.as_str())?;
        Ok(())
    }
}

impl<Conn> RunQueryDsl<Conn> for CreateDatabase {}

impl QueryId for CreateDatabase {
    type QueryId = ();
    const HAS_STATIC_QUERY_ID: bool = false;
}

#[derive(Debug, Clone)]
struct DropDatabase {
    database_name: String,
}

impl DropDatabase {
    pub fn with_name<T: Into<String>>(database_name: T) -> Self {
        DropDatabase {
            database_name: database_name.into(),
        }
    }
}

impl<DB: Backend> QueryFragment<DB> for DropDatabase {
    fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
        out.push_sql("drop database ");
        out.push_identifier(&self.database_name)?;
        Ok(())
    }
}

impl<Conn> RunQueryDsl<Conn> for DropDatabase {}

impl QueryId for DropDatabase {
    type QueryId = ();

    const HAS_STATIC_QUERY_ID: bool = false;
}

#[cfg(test)]
mod tests {
    use super::*;

}
