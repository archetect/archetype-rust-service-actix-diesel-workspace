use clap::{ArgMatches, value_t};
use tracing::Level;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::util::SubscriberInitExt;

use crate::cli::LogFormat;

pub fn init(config: &ArgMatches) {
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
        LogFormat::Standard => { register_standard_subscriber(filter, FmtSpan::ENTER | FmtSpan::CLOSE) }
        LogFormat::Json => { register_json_subscriber(filter, span_events) }
        LogFormat::Pretty => { register_pretty_subscriber(filter, span_events) }
        LogFormat::Bunyan => { register_bunyan_subscriber(filter) }
        LogFormat::None => ()
    }
}

fn register_standard_subscriber(filter: EnvFilter, span_events: FmtSpan) {
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(span_events)
        .with_ansi(atty::is(atty::Stream::Stdout))
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
        .init()
}

fn register_bunyan_subscriber(filter: EnvFilter) {
    tracing_subscriber::registry()
        .with(filter)
        .with(JsonStorageLayer)
        .with(BunyanFormattingLayer::new("{{ artifact-id }}".into(), std::io::stdout))
        .init();
}