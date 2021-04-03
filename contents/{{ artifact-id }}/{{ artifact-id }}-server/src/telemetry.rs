use actix_web_prom::PrometheusMetrics;
use clap::{ArgMatches, value_t};
use tracing::Level;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::util::SubscriberInitExt;

use {{ artifact_id }}_core::metrics;

use crate::cli::LogFormat;

pub fn init_tracing(config: &ArgMatches) {
    let mut filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    match config.occurrences_of("verbosity") {
        0 => (),
        1 => { filter = filter.add_directive(Level::DEBUG.into()); }
        _ => {
            filter = filter.add_directive("{{ artifact_id }}=trace".parse().unwrap());
        }
    };
    let format = value_t!(config, "log-format", crate::cli::LogFormat).unwrap_or_else(|e| e.exit());
    let span_events = FmtSpan::FULL;
    match format {
        LogFormat::Standard => { register_standard_subscriber(filter, span_events) }
        LogFormat::Json => { register_json_subscriber(filter, span_events) }
        LogFormat::Pretty => { register_pretty_subscriber(filter, span_events) }
        LogFormat::Bunyan => { register_bunyan_subscriber(filter) }
    }
}

fn register_standard_subscriber(filter: EnvFilter, span_events: FmtSpan) {
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(span_events)
        .init()
}

fn register_json_subscriber(filter: EnvFilter, span_events: FmtSpan) {
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(span_events)
        .json()
        .with_current_span(false)
        .init()
}

fn register_pretty_subscriber(filter: EnvFilter, span_events: FmtSpan) {
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(span_events)
        .pretty()
        .with_level(true)
        .with_thread_names(true)
        .init()
}

fn register_bunyan_subscriber(filter: EnvFilter) {
    tracing_subscriber::registry()
        .with(filter)
        .with(JsonStorageLayer)
        .with(BunyanFormattingLayer::new("{{ artifact-id }}".into(), std::io::stdout))
        .init();
}

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
