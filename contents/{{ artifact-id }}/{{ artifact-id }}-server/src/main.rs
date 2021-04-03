use actix_web::{App, HttpServer, middleware, web};
use actix_web::middleware::normalize::TrailingSlash;
use actix_web_prom::PrometheusMetrics;
use clap::ArgMatches;
use futures::future;
use tracing::{debug, warn};
use tracing_actix_web::TracingLogger;

use {{artifact_id}}_core::{ {{ ArtifactId }}, metrics};

use crate::telemetry::{init_tracing};

mod cli;
mod routes;
mod telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = cli::app().get_matches();
    cli::configure(&matches);

    init_tracing(&matches);

    debug!("Initializing...");

    let server_port = matches.value_of("server-port").unwrap().parse::<u16>().unwrap();
    let management_port = matches.value_of("management-port").unwrap().parse::<u16>().unwrap();
    let separate_management_port = (server_port != management_port) && server_port != 0;

    let server_metrics = if separate_management_port {
        server_metrics()
    } else {
        combined_metrics()
    };

    let cors_permissive = is_cors_permissive(&matches);
    if cors_permissive {
        warn!("Enabling permissive Cors configuration!");
    }

    let {{ suffix_name }} = {{ ArtifactId }}::new();

    let server = HttpServer::new(move || {
        let cors = if cors_permissive {
            actix_cors::Cors::permissive()
        } else {
            actix_cors::Cors::default()
        };
        let mut app = App::new()
            .wrap(TracingLogger)
            .wrap(server_metrics.clone())
            .wrap(cors)
            .wrap(middleware::NormalizePath::new(TrailingSlash::Trim))
            .app_data(web::JsonConfig::default())
            .data({{ suffix_name }}.clone())
            .configure(routes::server_routes)
            ;
        if !separate_management_port {
            app = app
                .configure(routes::management_routes)
            ;
        }
        app
    })
        .bind(("127.0.0.1", server_port))?
        .run();

    if separate_management_port {
        let management_metrics = management_metrics();
        let management = HttpServer::new(move || {
            App::new()
                .wrap(management_metrics.clone())
                .wrap(middleware::NormalizePath::new(TrailingSlash::Trim))
                .service(web::resource("/").to(routes::management_root))
                .configure(routes::management_routes)
        })
            .bind(("127.0.0.1", management_port))?
            .run();
        future::try_join(server, management).await?;
    } else {
        server.await?;
    }

    Ok(())
}

fn server_metrics() -> PrometheusMetrics {
    PrometheusMetrics::new_with_registry(
        prometheus::default_registry().clone(),
        metrics::METRICS_PREFIX,
        None,
        None)
        .unwrap()
}

fn management_metrics() -> PrometheusMetrics {
    PrometheusMetrics::new_with_registry(
        prometheus::default_registry().clone(),
        format!("{}_{}", metrics::METRICS_PREFIX, "management").as_str(),
        Some("/metrics"),
        None)
        .unwrap()
}

fn combined_metrics() -> PrometheusMetrics {
    PrometheusMetrics::new_with_registry(
        prometheus::default_registry().clone(),
        metrics::METRICS_PREFIX,
        Some("/metrics"),
        None)
        .unwrap()
}

fn is_cors_permissive(matches: &ArgMatches) -> bool {
    // The cors-permissive flag takes precedence
    if matches.is_present("cors-permissive") {
        return true;
    }
    // If CORS_PERMISSIVE environment variable has been set to anything other than false
    matches.value_of("cors-permissive-env")
        .map_or(false, |value| {
            if let Ok(value) = value.parse::<bool>() {
                value
            } else {
                true
            }
        }, )
}
