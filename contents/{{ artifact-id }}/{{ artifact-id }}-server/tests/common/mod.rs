use {{ artifact_id }}_server::{{ ArtifactId }}Server;
use {{ artifact_id }}_core::{{ ArtifactId }}Core;

pub async fn init_logging() {
    tracing_subscriber::fmt().init();
}

pub async fn start_server() -> Result<reqwest::Url, Box<dyn std::error::Error>> {
    let server = {{ ArtifactId }}Server::new({{ ArtifactId }}Core::new())
        .with_random_server_port()
        .build().unwrap();

    let server_port = server.server_port();

    actix_rt::spawn(async {
        let _ = server.run().await;
    });

    let root_url = format!("http://localhost:{}", server_port);
    let root_url = reqwest::Url::parse(root_url.as_str())?;
    Ok(root_url)
}