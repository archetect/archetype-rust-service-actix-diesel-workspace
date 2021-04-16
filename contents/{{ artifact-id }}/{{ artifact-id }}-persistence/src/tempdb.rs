use std::cell::RefCell;
use tracing::info;

thread_local! {
    pub static SCHEMA_MANAGER: RefCell<SchemaManager> = RefCell::new(SchemaManager::new());
}

pub struct SchemaManager {
    schemas: Vec<String>,
}

impl SchemaManager {
    pub fn new() -> SchemaManager {
        SchemaManager{
            schemas: vec![],
        }
    }

    pub fn add_schema<T: Into<String>>(&mut self, schema: T) {
        self.schemas.push(schema.into());
    }

    pub fn schemas(&self) -> &[String] {
        self.schemas.as_ref()
    }
}

impl Drop for SchemaManager {
    fn drop(&mut self) {
        for schema in self.schemas() {
            let schema_name_splits: Vec<&str> = schema.rsplit("/").collect();

            if let Some(schema_name) = schema_name_splits.first() {
                info!("Destroying Temp Schema '{}'", schema_name);
            }
        }
    }
}

