use actix_web_prom::PrometheusMetrics;

use {{ artifact_id }}_core::metrics;

pub fn server_metrics() -> PrometheusMetrics {
    PrometheusMetrics::new_with_registry(
        prometheus::default_registry().clone(),
        metrics::METRICS_PREFIX,
        None,
        None)
        .unwrap()
}

pub fn management_metrics() -> PrometheusMetrics {
    PrometheusMetrics::new_with_registry(
        prometheus::default_registry().clone(),
        format!("{}_{}", metrics::METRICS_PREFIX, "management").as_str(),
        Some("/metrics"),
        None)
        .unwrap()
}

pub fn combined_metrics() -> PrometheusMetrics {
    PrometheusMetrics::new_with_registry(
        prometheus::default_registry().clone(),
        metrics::METRICS_PREFIX,
        Some("/metrics"),
        None)
        .unwrap()
}
