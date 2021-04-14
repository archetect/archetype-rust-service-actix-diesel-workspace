use serde::{Deserialize, Serialize};

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
        DatabaseSettings { url: String::from("postgres://postgres:password@localhost/{{ artifact_id }}") }
    }
}
