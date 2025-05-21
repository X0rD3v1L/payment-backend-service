use payment_service::rocket;
use rocket::local::asynchronous::Client;
use rocket::http::Status;
use rocket::uri;

#[rocket::async_test]
async fn hello_rocket() {
    let rocket_instance = rocket().await;
    let client = Client::tracked(rocket_instance).await.expect("valid rocket instance");

    let response = client.get(uri!(payment_service::index)).dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await.unwrap(), "Hello, Rocket!");
}