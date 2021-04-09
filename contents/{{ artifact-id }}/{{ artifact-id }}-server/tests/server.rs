use {{ artifact_id }}_api::{{ ArtifactId }};

mod common;

#[actix_rt::test]
// #[cfg_attr(not(feature = "expensive_tests"), ignore)]
async fn get_{{ prefixName | pluralize }}() -> Result<(), Box<dyn std::error::Error>> {
    common::init_logging().await;

    let context = common::start_server().await?;

    let results = context.client().get_{{ prefixName | pluralize }}().await?;
    println!("{:?}", results);

    assert_eq!(results.len(), 2);

    Ok(())
}