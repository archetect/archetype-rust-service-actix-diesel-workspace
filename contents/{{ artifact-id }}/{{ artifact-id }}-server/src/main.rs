use actix_web::{App, HttpServer, web};
use actix_web_prom::PrometheusMetrics;
use clap::ArgMatches;
use futures::future;
use log::{debug, warn};

use {{artifact_id}}_core::metrics;

mod cli;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = cli::app().get_matches();
    cli::configure(&matches);

    debug!("Initializing...");

    let server_port = matches.value_of("server-port").unwrap().parse::<u16>().unwrap();
    let management_port = matches.value_of("management-port").unwrap().parse::<u16>().unwrap();
    let separate_management_port = server_port != management_port;

    let server_metrics = if separate_management_port {
        server_metrics()
    } else {
        combined_metrics()
    };

    let cors_permissive = is_cors_permissive(&matches);
    if cors_permissive {
        warn!("Enabling permissive Cors configuration!");
    }

    let server = HttpServer::new(move || {
        let cors = if cors_permissive {
            actix_cors::Cors::permissive()
        } else {
            actix_cors::Cors::default()
        };
        let mut app = App::new()
            .wrap(server_metrics.clone())
            .wrap(cors)
            .service(web::resource("/").to(routes::server_root))
            .route("/", web::get().to(routes::greet))
            .route("/greet", web::get().to(routes::greet))
            .route("/greet/{name}", web::get().to(routes::greet));
        if !separate_management_port {
            app = app
                .service(web::resource("/ping").to(routes::ping))
                .service(web::resource("/health").to(routes::health));
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
                .service(web::resource("/").to(routes::management_root))
                .service(web::resource("/ping").to(routes::ping))
                .service(web::resource("/health").to(routes::health))
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
