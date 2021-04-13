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

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseSettings {
    url: String,
}

impl DatabaseSettings {
    pub fn url(&self) -> &str {
        self.url.as_str()
    }
}
