use async_trait::async_trait;

use {{ artifact_id }}_api::{{ArtifactId}};
use {{ artifact_id }}_api::models::{{PrefixName}};
use reqwest::IntoUrl;

#[derive(Debug)]
pub struct {{ ArtifactId }}Client {
    endpoint: reqwest::Url,
    client: reqwest::Client,
}

impl {{ ArtifactId }}Client {
    pub fn new<T: IntoUrl>(endpoint: T) -> Result<{{ ArtifactId }}Client, Box<dyn std::error::Error>> {
        Ok({{ ArtifactId }}Client {
            endpoint: endpoint.into_url()?,
            client: reqwest::Client::new(),
        })
    }
}

#[async_trait]
impl {{ ArtifactId }} for {{ ArtifactId }}Client {
    async fn get_{{ prefix_name | pluralize }}(&self) -> Result<Vec<{{ PrefixName}}>, Box<dyn std::error::Error>> {
        let endpoint = self.endpoint.join("/{{ prefixName | pluralize }}")?;
        let results = self.client
            .get(endpoint)
            .send()
            .await?
            .json::<Vec<{{ PrefixName }}>>()
            .await?;
        Ok(results)
    }
}