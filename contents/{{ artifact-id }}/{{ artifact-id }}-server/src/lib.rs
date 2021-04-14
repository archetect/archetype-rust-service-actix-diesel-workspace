use std::net::TcpListener;

use actix_web::{App, HttpServer, middleware, web};
use actix_web::dev::Server;
use actix_web::middleware::normalize::TrailingSlash;
use tracing::warn;
use tracing_actix_web::TracingLogger;

use {{ artifact_id }}_core::{{ ArtifactId }}Core;
use crate::settings::ServerSettings;

mod routes;
mod metrics;
pub mod settings;

pub struct {{ ArtifactId }}Server {
    service_port: u16,
    service_server: Server,
    management_port: u16,
    management_server: Option<Server>,
}

impl {{ ArtifactId }}Server {
    pub fn new(service_core: {{ ArtifactId }}Core) -> Builder {
        Builder::new(service_core,  &ServerSettings::default())
    }

    pub fn service_port(&self) -> u16 {
        self.service_port
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
    host: String,
    service_port: u16,
    management_port: Option<u16>,
    service_core: {{ ArtifactId }}Core,
    cors_permissive: bool,
}

impl Builder {
    fn new(core: {{ ArtifactId }}Core, settings: &ServerSettings) -> Self {
        Builder {
            host: settings.host().to_owned(),
            service_port: settings.service().port(),
            management_port: Some(settings.management().port()),
            service_core: core,
            cors_permissive: false,
        }
    }

    pub fn with_host<T: Into<String>>(mut self, host: T) -> Self {
        self.host = host.into();
        self
    }

    pub fn with_settings(self, settings: &settings::ServerSettings) -> Self {
        self.with_host(settings.host())
            .with_service_port(settings.service().port())
            .with_management_port(settings.management().port())
    }

    pub fn with_service_port(mut self, service_port: u16) -> Self {
        self.service_port = service_port;
        self
    }
    
    pub fn with_random_service_port(mut self) -> Self {
        self.service_port = 0;
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

        let host = &self.host;

        let service_listener = TcpListener::bind((host.as_str(), self.service_port))?;
        let service_port = service_listener.local_addr().unwrap().port();

        let management_port = if let Some(management_port) = self.management_port {
            management_port
        } else {
            service_port
        };

        let separate_management_port = (service_port != management_port) && self.service_port != 0;

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
                .bind((host.as_str(), management_port))?
                .run())
        } else {
            None
        };

        Ok({{ ArtifactId }}Server {
            service_port,
            service_server,
            management_port,
            management_server,
        })
    }
}
