use std::cell::RefCell;
use tracing::info;
use url::Url;

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
    if let Some(database_name) = &mut get_database_name(&url) {
        database_name.push_str("_123");
        url.set_path(&database_name);
    } else {
        url.set_path("123");
    }
    url
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(input, expected,
    case(Url::parse("postgresql://localhost/mydb").unwrap(), Some("mydb".to_owned())),
    case(Url::parse("postgresql://other@localhost/otherdb?connect_timeout=10&application_name=myapp").unwrap(),
    Some("otherdb".to_owned())),
    case(Url::parse("postgresql://user:secret@localhost").unwrap(), None),
    case(Url::parse("postgresql://localhost:5433").unwrap(), None),
    case(Url::parse("postgresql://localhost").unwrap(), None),
    case(Url::parse("postgresql://").unwrap(), None),
    )]
    fn test_get_database_name(input: Url, expected: Option<String>) {
        assert_eq!(get_database_name(&input), expected);
    }

    #[rstest(input, expected,
    case(Url::parse("postgresql://localhost/mydb").unwrap(),
    Url::parse("postgresql://localhost/postgres").unwrap()),
    case(Url::parse("postgresql://localhost").unwrap(),
    Url::parse("postgresql://localhost/postgres").unwrap()),
    case(Url::parse("postgresql://other@localhost/otherdb?connect_timeout=10&application_name=myapp").unwrap(),
    Url::parse("postgresql://other@localhost/postgres?connect_timeout=10&application_name=myapp").unwrap()),
    )]
    fn test_get_admin_url(input: Url, expected: Url) {
        assert_eq!(get_admin_url(&input), expected);
    }

    #[rstest(input, expected,
    case(Url::parse("postgresql://localhost/mydb").unwrap(),
    Url::parse("postgresql://localhost/mydb_123").unwrap()),
    case(Url::parse("postgresql://localhost").unwrap(),
    Url::parse("postgresql://localhost/123").unwrap()),
    case(Url::parse("postgresql://other@localhost/otherdb?connect_timeout=10&application_name=myapp").unwrap(),
    Url::parse("postgresql://other@localhost/otherdb_123?connect_timeout=10&application_name=myapp").unwrap()),
    )]
    fn test_get_tempdb_url(input: Url, expected: Url) {
        assert_eq!(get_tempdb_url(&input), expected);
    }
}

