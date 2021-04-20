use serde::{Deserialize, Serialize};
use url::Url;

const DEFAULT_DATABASE_URL: &str = "postgres://postgres:password@localhost/{{ artifact_id }}";

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum TemporaryType {
    #[serde(rename = "drop")]
    Drop,
    #[serde(rename = "retain")]
    Retain,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersistenceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    temporary: Option<TemporaryType>,
    database: DatabaseSettings,
}

impl PersistenceSettings {
    pub fn tempdb(&self) -> Option<&TemporaryType> {
        self.temporary.as_ref()
    }

    pub fn with_tempdb(mut self, tembdb_type: TemporaryType) -> PersistenceSettings {
        self.temporary = Some(tembdb_type);
        self
    }
    
    pub fn database(&self) -> &DatabaseSettings {
        &self.database
    }
}

impl Default for PersistenceSettings {
    fn default() -> Self {
        PersistenceSettings {
            temporary: None,
            database: DatabaseSettings::default(),
        }
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

    pub fn with_url(mut self, url: &Url) -> Self {
        self.url = url.to_string();
        self
    }
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        DatabaseSettings { url: String::from(DEFAULT_DATABASE_URL) }
    }
}
