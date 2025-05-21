use payment_service::rocket;

#[rocket::main]
async fn main() {
    rocket().await.launch().await.unwrap();
}
