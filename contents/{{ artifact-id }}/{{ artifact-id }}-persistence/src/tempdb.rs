use std::cell::RefCell;
use diesel::*;
use diesel::query_builder::{QueryFragment, AstPass, QueryId};
use diesel::backend::Backend;
use crate::settings::{DatabaseSettings};
use tracing::info;
use url::Url;

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

pub fn create_database_if_not_exists(database_settings: &DatabaseSettings) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(database_settings.url())?;
    if let Some(database_name) = get_database_name(&url) {
        let admin_url = get_admin_url(&url);
        let conn = PgConnection::establish(admin_url.as_str())?;
        if !database_exists(&conn, &database_name)? {
            CreateDatabase::with_name(&database_name).execute(&conn)?;
            eprintln!("'{}' database created", database_name);
        } else {
            eprintln!("'{}' database already exists", database_name);
        }
    }
    Ok(())
}

pub fn database_migrate(database_settings: &DatabaseSettings) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(database_settings.url())?;
    let conn = PgConnection::establish(url.as_str())?;
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
                info!("Dropping Temp Database '{}'", database_name);
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


pub struct CreateDatabase {
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

#[cfg(test)]
mod tests {
    use super::*;

}
