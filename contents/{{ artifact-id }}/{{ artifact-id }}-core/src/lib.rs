use async_trait::async_trait;
use tracing::trace;

use {{ artifact_id }}_api::{{ArtifactId}};
use {{ artifact_id }}_api::models::{{PrefixName}};
use {{ artifact_id }}_persistence::{{ArtifactId}}Persistence;
use {{ artifact_id }}_persistence::settings::PersistenceSettings;

pub mod metrics;

#[derive(Clone)]
pub struct {{ ArtifactId }}Core {
    persistence: {{ArtifactId}}Persistence,
}

impl {{ ArtifactId }}Core {
pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
    Ok(Self {
        persistence: {{ArtifactId}}Persistence::new(&PersistenceSettings::default())?,
    })
}

pub fn new_with_persistence(persistence: {{ArtifactId}}Persistence) -> Self {
    Self { persistence }
}
}

#[async_trait]
impl {{ ArtifactId }} for {{ ArtifactId }}Core {
    async fn get_{{prefix_name | pluralize }}(&self) -> Result<Vec<{{PrefixName}}>, Box<dyn std::error::Error>> {
        // let _conn = self.pool.get().unwrap();
        let mut results = vec![];
        results.push({{PrefixName}}::new("Example 1"));
        results.push({{PrefixName}}::new("Example 2"));
        Ok(results)
    }
}

pub fn get_greeting() -> &'static str {
    trace!("Preparing greeting");
    metrics::EXAMPLE_COUNTER.inc();
    "Hello"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_greeting() {
        assert_eq!(get_greeting(), "Hello");
    }
}
