use std::cell::RefCell;
use tracing::info;
use url::Url;

thread_local! {
    pub static TEMP_DATABASES: RefCell<TempDatabases> = RefCell::new(TempDatabases::new());
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

#[cfg(test)]
mod tests {
    use super::*;

}
