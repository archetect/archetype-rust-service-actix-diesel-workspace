use tracing::{debug};

use {{ artifact_id }}_core::{{ ArtifactId }}Core;
use {{ artifact_id }}_server::{{'{'}}{{ ArtifactId }}Server, settings};

mod cli;
mod logging;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::app().get_matches();
    let config = settings::Settings::new(&matches)?;
    logging::init(&matches);

    debug!("Initializing...");

    let service_core = {{ ArtifactId }}Core::new();

    {{ ArtifactId }}Server::new(service_core)
        .with_host("0.0.0.0")
        .with_server_settings(config.server())
        .with_management_settings(config.management())
        .with_cors_permissive(cli::is_cors_permissive(&matches))
        .build()?
        .run()
        .await?;

    Ok(())
}
