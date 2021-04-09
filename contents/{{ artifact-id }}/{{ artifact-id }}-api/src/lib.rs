pub mod models;

use async_trait::async_trait;

#[async_trait]
pub trait {{ArtifactId}} {
    async fn get_{{ prefix_name | pluralize }}(&self) -> Result<Vec<models::{{PrefixName}}>, Box<dyn std::error::Error>>;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
