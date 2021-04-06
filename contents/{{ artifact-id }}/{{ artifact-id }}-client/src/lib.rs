use async_trait::async_trait;

use {{ artifact_id }}_api::{{ArtifactId}};
use {{ artifact_id }}_api::models::{{PrefixName}};

#[derive(Debug)]
pub struct {{ ArtifactId }}Client {
}

#[async_trait]
impl {{ ArtifactId }} for {{ ArtifactId }}Client {
    async fn get_{{prefix_name | pluralize }}(&self) -> Vec<{{PrefixName}}> {
        unimplemented!()
    }
}

