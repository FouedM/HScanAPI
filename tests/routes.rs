// tests/routes.rs

use rocket::http::{ContentType, Status};
use rocket::local::Client;
use your_crate_name::rocket; // replace with your crate name

#[tokio::test]
async fn test_verify_product() {
    let rocket = rocket().ignite().await.unwrap();
    let client = Client::untracked(rocket).expect("valid rocket instance");
    let response = client.post("/verifyProduct")
        .header(ContentType::Form)
        .body("barCode=1234567890128") // replace with a valid barcode
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let response_body = response.into_string().await.expect("valid response body");
    assert!(response_body.contains("\"result\":true"));
}