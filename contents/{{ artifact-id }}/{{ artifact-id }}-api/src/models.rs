use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct {{ PrefixName }} {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u64>,
    title: String,
}

impl {{ PrefixName }} {
    pub fn new<T: Into<String>>(title: T) -> Self {
        Self{ id: None, title: title.into() }
    }

    pub fn with_id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }
}