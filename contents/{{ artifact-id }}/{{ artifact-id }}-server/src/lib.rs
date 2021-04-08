use std::net::TcpListener;

use actix_web::{App, HttpServer, middleware, web};
use actix_web::dev::Server;
use actix_web::middleware::normalize::TrailingSlash;
use tracing::warn;
use tracing_actix_web::TracingLogger;

use {{ artifact_id }}_core::{{ ArtifactId }}Core;

mod routes;
mod metrics;

pub struct {{ ArtifactId }}Server {
    server_port: u16,
    service_server: Server,
    management_port: u16,
    management_server: Option<Server>,
}

impl {{ ArtifactId }}Server {
    pub fn new(server_port: u16, service_core: {{ ArtifactId }}Core) -> Builder {
        Builder::new(server_port, service_core )
    }

    pub fn server_port(&self) -> u16 {
        self.server_port
    }

    pub fn management_port(&self) -> u16 {
        self.management_port
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        if let Some(management_server) = self.management_server {
            futures::future::try_join(self.service_server, management_server).await.map(|_| ())
        } else {
            self.service_server.await
        }
    }
}

pub struct Builder {
    server_port: u16,
    management_port: Option<u16>,
    service_core: {{ ArtifactId }}Core,
    cors_permissive: bool,
}

impl Builder {
    pub fn new(server_port: u16, service_core: {{ ArtifactId }}Core) -> Self {
        Builder {
            server_port,
            management_port: None,
            service_core,
            cors_permissive: false,
        }
    }

    pub fn with_server_port(mut self, server_port: u16) -> Self {
        self.server_port = server_port;
        self
    }

    pub fn with_management_port(mut self, management_port: u16) -> Self {
        self.management_port = Some(management_port);
        self
    }

    pub fn with_cors_permissive(mut self, cors_permissive: bool) -> Self {
        self.cors_permissive = cors_permissive;
        self
    }

    pub fn build(self) -> Result<{{ ArtifactId }}Server, std::io::Error> {
        let service_core = self.service_core.clone();

        let service_listener = TcpListener::bind(("0.0.0.0", self.server_port))?;
        let server_port = service_listener.local_addr().unwrap().port();

        let management_port = if let Some(management_port) = self.management_port {
            management_port
        } else {
            server_port
        };

        let separate_management_port = (server_port != management_port) && self.server_port != 0;

        let server_metrics = if separate_management_port {
            metrics::server_metrics()
        } else {
            metrics::combined_metrics()
        };

        let cors_permissive = self.cors_permissive;
        if cors_permissive {
            warn!("Enabling permissive Cors configuration!");
        }

        let service_server = HttpServer::new(move || {
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
                .data(service_core.clone())
                .configure(routes::server_routes)
                ;
            if !separate_management_port {
                app = app
                    .configure(routes::management_routes)
                ;
            }
            app
        }).listen(service_listener)?.run();

        let management_server = if separate_management_port {
            let management_metrics = metrics::management_metrics();
            Some(HttpServer::new(move || {
                App::new()
                    .wrap(TracingLogger)
                    .wrap(management_metrics.clone())
                    .wrap(middleware::NormalizePath::new(TrailingSlash::Trim))
                    .service(web::resource("/").to(routes::management_root))
                    .configure(routes::management_routes)
            })
                .bind(("0.0.0.0", management_port))?
                .run())
        } else {
            None
        };

        Ok({{ ArtifactId }}Server {
            server_port,
            service_server,
            management_port,
            management_server,
        })
    }
}