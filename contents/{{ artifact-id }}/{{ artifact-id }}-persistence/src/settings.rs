use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseSettings {
    url: String,
}

impl DatabaseSettings {
    pub fn url(&self) -> &str {
        self.url.as_str()
    }
}
