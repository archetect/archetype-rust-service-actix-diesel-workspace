use actix_web::http::StatusCode;

mod common;

#[actix_rt::test]
// #[cfg_attr(not(feature = "expensive_tests"), ignore)]
async fn management_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    common::init_logging().await;

    let client = reqwest::Client::new();
    let root_url = common::start_server().await?;

    let res = client.get(root_url.join("health")?).send().await?;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.content_length(), Some(0));

    let res = client.get(root_url.join("ping")?).send().await?;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.content_length(), Some(0));

    let res = client.get(root_url.join("metrics")?).send().await?;
    assert_eq!(res.status(), StatusCode::OK);
    assert_ne!(res.content_length(), Some(0));

    Ok(())
}