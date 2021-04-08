use tracing::{debug};

use {{ artifact_id }}_core::{{ ArtifactId }}Core;
use {{ artifact_id }}_server::{{ ArtifactId }}Server;

mod cli;
mod logging;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = cli::app().get_matches();
    logging::init(&matches);

    debug!("Initializing...");

    let service_core = {{ ArtifactId }}Core::new();
    let server_port = matches.value_of("server-port").unwrap().parse::<u16>().unwrap();
    let management_port = matches.value_of("management-port").unwrap().parse::<u16>().unwrap();

    {{ ArtifactId }}Server::new(server_port, service_core)
        .with_management_port(management_port)
        .with_cors_permissive(cli::is_cors_permissive(&matches))
        .build()?
        .run()
        .await?;

    Ok(())
}
