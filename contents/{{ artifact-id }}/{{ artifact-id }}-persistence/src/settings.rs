use serde::{Deserialize, Serialize};

const DEFAULT_DATABASE_URL: &str = "postgres://postgres:password@localhost/{{ artifact_id }}";

#[derive(Debug, Deserialize, Serialize)]
pub struct PersistenceSettings {
    database: DatabaseSettings,
}

impl PersistenceSettings {
    pub fn database(&self) -> &DatabaseSettings {
        &self.database
    }
}

impl Default for PersistenceSettings {
    fn default() -> Self {
        PersistenceSettings { database: DatabaseSettings::default() }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseSettings {
    url: String,
}

impl DatabaseSettings {
    pub fn url(&self) -> &str {
        self.url.as_str()
    }
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        DatabaseSettings { url: String::from(DEFAULT_DATABASE_URL) }
    }
}
