use tracing::{debug};

use {{ artifact_id }}_core::{{ ArtifactId }}Core;
use {{ artifact_id }}_server::{{'{'}}{{ ArtifactId }}Server, settings};

mod cli;
mod logging;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::app().get_matches();
    let settings = settings::Settings::new(&args)?;
    logging::init(&args);

    match args.subcommand() {
        ("settings", Some(subargs)) => {
            match subargs.subcommand() {
                ("defaults", _) => println!("{}", settings::defaults()),
                ("merged", _) => println!("{}", serde_yaml::to_string(&settings)?),
                (_, _) => (), // Unreachable
            }
            return Ok(());
        },
        (_, _) => (), // Unreachable
    }

    debug!("Initializing...");

    let service_core = {{ ArtifactId }}Core::new();

    {{ ArtifactId }}Server::new(service_core)
        .with_settings(settings.server())
        .with_cors_permissive(cli::is_cors_permissive(&args))
        .build()?
        .run()
        .await?;

    Ok(())
}
