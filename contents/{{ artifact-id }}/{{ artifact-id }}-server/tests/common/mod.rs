use reqwest::RequestBuilder;

use {{ artifact_id }}_client::{{ ArtifactId }}Client;
use {{ artifact_id }}_core::{{ ArtifactId }}Core;
use {{ artifact_id }}_server::{{ ArtifactId }}Server;

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
        reqwest: reqwest::Client::new(),
        client: {{ ArtifactId }}Client::new(root_url.clone())?,
        server_endpoint: root_url.clone(),
        management_endpoint: root_url,
    })
}

#[allow(dead_code)]
pub struct ServerTestContext {
    reqwest: reqwest::Client,
    client: {{ ArtifactId }}Client,
    server_endpoint: reqwest::Url,
    management_endpoint: reqwest::Url,
}

#[allow(dead_code)]
impl ServerTestContext {
    pub fn client(&self) -> &{{ ArtifactId }}Client {
        &self.client
    }

    pub fn reqwest(&self) -> &reqwest::Client {
        &self.reqwest
    }

    pub fn get<T: Into<String>>(&self, path: T) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
        Ok(self.reqwest().get(self.server_endpoint.join(path.into().as_str())?))
    }

    pub fn get_management_path<T: Into<String>>(&self, path: T) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
        Ok(self.reqwest().get(self.management_endpoint.join(path.into().as_str())?))
    }
}