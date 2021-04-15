use tracing::debug;

use {{ artifact_id }}_core::{{ArtifactId}}Core;
use {{ artifact_id }}_persistence::{{ArtifactId}}Persistence;
use {{ artifact_id }}_server::{settings, {{ArtifactId}}Server};

mod cli;
mod logging;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::app().get_matches();
    let settings = settings::merge(&args)?;
    logging::init(&args);

    match args.subcommand() {
        ("settings", Some(subargs)) => {
            match subargs.subcommand() {
                ("defaults", _) => println!("{}", settings::Settings::default().to_yaml()?),
                ("merged", _) => println!("{}", &settings.to_yaml()?),
                (_, _) => (), // Unreachable
            }
            return Ok(());
        }
        (_, _) => (), // Unreachable
    }

    debug!("Initializing...");

    let service_core = {{ArtifactId}}Core::new_with_persistence(
        {{ArtifactId}}Persistence::new_with_settings(settings.persistence())?,
    );

    {{ArtifactId}}Server::new(service_core)
        .with_settings(settings.server())
        .with_cors_permissive(cli::is_cors_permissive(&args))
        .build()?
        .run()
        .await?;

    Ok(())
}
