use actix_web::{App, test, web};
use zero2prod::health_check;

#[actix_web::test]
async fn test_health_check() {
    let app =
        test::init_service(App::new().route("/health_check", web::get().to(health_check))).await;
    let req = test::TestRequest::get().uri("/health_check").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    assert!(resp.status().is_success())
}
