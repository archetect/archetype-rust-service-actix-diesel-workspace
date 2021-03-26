use once_cell::sync::Lazy;
use prometheus::{ self, IntCounter };

pub static METRICS_PREFIX: &str = "{{ artifact_id }}";

pub static EXAMPLE_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    prometheus::register_int_counter!(prefixed("example_count"), "Example Counter").unwrap()
});

fn prefixed(suffix: &str) -> String {
    format!("{}_{}", METRICS_PREFIX, suffix)
}