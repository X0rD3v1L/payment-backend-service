#[macro_use]
extern crate rocket;
use controllers::{Response, SuccessResponse};
use fairings::cors::{CORS, options};
use migrator::Migrator;
use rocket::{Build, Rocket, http::Status};
use sea_orm_migration::MigratorTrait;

mod auth;
mod controllers;
mod db;
mod entities;
mod fairings;
mod migrator;
mod utils;
mod kafka;

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
    jwt_secret: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_host: std::env::var("PAYMENTS_DB_HOST").unwrap_or("localhost".to_string()),
            db_port: std::env::var("PAYMENTS_DB_PORT").unwrap_or("5432".to_string()),
            db_username: std::env::var("PAYMENTS_DB_USERNAME").unwrap_or("postgres".to_string()),
            db_password: std::env::var("PAYMENTS_DB_PASSWORD").unwrap_or("root".to_string()),
            db_database: std::env::var("PAYMENTS_DB_DATABASE").unwrap_or("payments".to_string()),
            jwt_secret: std::env::var("PAYMENTS_JWT_SECRET")
                .expect("Please set the PAYMENTS_JWT_SECRET env variable."),
        }
    }
}

#[get("/")]
pub fn index() -> Response<String> {
    Ok(SuccessResponse((Status::Ok, "Hello, Rocket!".to_string())))
}

#[launch]
pub async fn rocket() -> Rocket<Build> {
    dotenvy::dotenv().ok();
    let config = AppConfig::default();
    let db = db::connect(&config).await.unwrap();

    Migrator::up(&db, None).await.unwrap();

    // Spawn Kafka consumer task
    let db_clone = db.clone();
    tokio::spawn(async move {
        kafka::consumer::start(db_clone).await;
    });
    
    rocket::build()
        .attach(CORS)
        .manage(db)
        .manage(config)
        .mount("/", routes![options])
        .mount("/", routes![index])
        .mount("/", routes![
            controllers::profile::get_profile,
            controllers::profile::update_profile
        ])
        .mount("/auth", routes![
            controllers::auth::register,
            controllers::auth::login,
            controllers::auth::me
        ])
        .mount("/accounts", routes![controllers::accounts::balance])
        .mount("/transactions", routes![
            controllers::transactions::create_transaction,
            controllers::transactions::get_transaction_status,
            controllers::transactions::list_transactions
        ])
}

