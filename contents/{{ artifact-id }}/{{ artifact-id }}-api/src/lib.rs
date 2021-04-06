pub mod models;

use async_trait::async_trait;

#[async_trait]
pub trait {{ArtifactId}} {
    async fn get_{{ prefix_name | pluralize }}(&self) -> Vec<models::{{PrefixName}}>;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
