use actix_web::{App, test};
use zero2prod::routes::subscribe;

#[actix_web::test]
async fn test_subscriptions() {
    let app = test::init_service(App::new().service(subscribe)).await;
    let req = test::TestRequest::get().uri("/subscribe").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    assert!(resp.status().is_success())
}
