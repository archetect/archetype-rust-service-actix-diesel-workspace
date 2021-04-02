use tracing::{trace};

use {{ artifact_id }}_persistence::{establish_connection, PgPool};

pub mod metrics;

#[derive(Clone)]
pub struct {{ ArtifactId }} {
    pool: PgPool
}

impl {{ ArtifactId }} {
    pub fn new() -> Self {
        Self { pool: establish_connection() }
    }

    pub fn new_with_pool(pool: PgPool) -> {{ ArtifactId }} {
        Self { pool }
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

