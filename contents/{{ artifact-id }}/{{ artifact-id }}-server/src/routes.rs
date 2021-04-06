use actix_web::{HttpRequest, HttpResponse, Responder, web::{self, ServiceConfig}};

use {{ artifact_id }}_api::{{'{'}}{{ArtifactId}}};
use {{ artifact_id }}_core::{get_greeting, {{ArtifactId}}Core};
use {{ artifact_id }}_core::metrics;


pub async fn {{ prefix_name | pluralize }}(core: web::Data<{{ArtifactId}}Core>) -> HttpResponse {
    HttpResponse::Ok().json(core.get_{{ prefix_name | pluralize }}().await)
}

pub fn server_routes(config: &mut ServiceConfig) {
    config.route("/", web::get().to(server_root));
    config.route("/greet", web::get().to(greet));
    config.route("/greet/{name}", web::get().to(greet));
    config.route("/{{prefixName | pluralize}}", web::get().to({{ prefix_name | pluralize }}));
}

pub fn management_routes(config: &mut ServiceConfig) {
    config.route("/ping", web::get().to(ping));
    config.route("/health", web::get().to(health));
}

pub fn server_root() -> HttpResponse {
    HttpResponse::Ok().body(metrics::METRICS_PREFIX.to_uppercase())
}

pub fn management_root() -> HttpResponse {
    HttpResponse::Ok().body(format!("{}_MANAGEMENT",metrics::METRICS_PREFIX.to_uppercase()))
}

pub fn ping() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub(crate) async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    metrics::EXAMPLE_COUNTER.inc();
    format!("{}, {}! ({})", get_greeting(), &name, metrics::EXAMPLE_COUNTER.get())
}
