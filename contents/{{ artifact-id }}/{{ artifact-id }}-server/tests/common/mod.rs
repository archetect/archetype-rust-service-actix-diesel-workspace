use {{ artifact_id }}_server::{{ ArtifactId }}Server;
use {{ artifact_id }}_core::{{ ArtifactId }}Core;
use reqwest::RequestBuilder;

pub async fn init_logging() {
    tracing_subscriber::fmt().init();
}

pub async fn start_server() -> Result<ServerTestContext, Box<dyn std::error::Error>> {
    let server = {{ ArtifactId }}Server::new({{ ArtifactId }}Core::new())
        .with_random_server_port()
        .build().unwrap();

    let server_port = server.server_port();

    actix_rt::spawn(async {
        let _ = server.run().await;
    });

    let root_url = format!("http://localhost:{}", server_port);
    let root_url = reqwest::Url::parse(root_url.as_str())?;

    Ok(ServerTestContext {
        client: reqwest::Client::new(),
        server_endpoint: root_url.clone(),
        management_endpoint: root_url,
    })
}

pub struct ServerTestContext {
    client: reqwest::Client,
    server_endpoint: reqwest::Url,
    management_endpoint: reqwest::Url,
}

impl ServerTestContext {
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn get<T: Into<String>>(&self, path: T) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
        Ok(self.client().get(self.server_endpoint.join(path.into().as_str())?))
    }

    pub fn get_management_path<T: Into<String>>(&self, path: T) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
        Ok(self.client().get(self.management_endpoint.join(path.into().as_str())?))
    }
}