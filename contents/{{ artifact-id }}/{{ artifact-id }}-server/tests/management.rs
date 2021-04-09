use actix_web::http::StatusCode;

mod common;

#[actix_rt::test]
// #[cfg_attr(not(feature = "expensive_tests"), ignore)]
async fn management_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    common::init_logging().await;

    let context = common::start_server().await?;

    let res = context.get("")?.send().await?;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await?, "{{ artifact_id | constant_case }}".to_string());

    let res = context.get_management_path("/health")?.send().await?;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.content_length(), Some(0));

    let res = context.get_management_path("/ping")?.send().await?;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.content_length(), Some(0));

    let res = context.get_management_path("/metrics")?.send().await?;
    assert_eq!(res.status(), StatusCode::OK);
    assert_ne!(res.content_length(), Some(0));

    Ok(())
}