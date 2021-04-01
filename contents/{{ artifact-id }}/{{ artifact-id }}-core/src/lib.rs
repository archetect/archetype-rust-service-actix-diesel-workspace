use log::{trace};

pub mod metrics;

#[derive(Clone)]
pub struct {{ ArtifactId }} {

}

impl {{ ArtifactId }} {
    pub fn new() -> Self {
        Self {}
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

