use async_trait::async_trait;
use tracing::{trace};

use {{ artifact_id }}_api::{{ArtifactId}};
use {{ artifact_id }}_api::models::{{PrefixName}};
use {{ artifact_id }}_persistence::{establish_connection, PgPool};

pub mod metrics;

#[derive(Clone)]
pub struct {{ ArtifactId }}Core {
    pool: PgPool
}

impl {{ ArtifactId }}Core {
    pub fn new() -> Self {
        Self { pool: establish_connection() }
    }

    pub fn new_with_pool(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl {{ ArtifactId }} for {{ ArtifactId }}Core {
    async fn get_{{prefix_name | pluralize }}(&self) -> Vec<{{PrefixName}}> {
        // let _conn = self.pool.get().unwrap();
        let mut results = vec![];
        results.push({{PrefixName}}::new("Example 1"));
        results.push({{PrefixName}}::new("Example 2"));
        results
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

