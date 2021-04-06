use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct {{ PrefixName }} {
    title: String,
}

impl {{ PrefixName }} {
    pub fn new(title: String) -> Self {
        Self{ title }
    }
}