use actix_web::{HttpRequest, Responder, HttpResponse};

use {{ artifact_id }}_core::get_greeting;
use {{ artifact_id }}_core::metrics;

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
