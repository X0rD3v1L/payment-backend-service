use std::time::SystemTime;

use crate::utils::random::generate_initial_balance;
use crate::utils::validations::format_validation_errors_json;
use crate::{
    AppConfig,
    auth::AuthenticatedUser,
    entities::{account, prelude::*, users},
};

use super::{ErrorResponse, Response, SuccessResponse};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{FixedOffset, Utc};
use garde::Validate;
use jsonwebtoken::{EncodingKey, Header, encode};
use rocket::{
    State,
    http::Status,
    serde::{Deserialize, Serialize, json::Json},
};
use sea_orm_migration::prelude::Expr;
use rand::{rng, Rng};
use sea_orm::*;

#[derive(Debug, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ReqRegister {
    #[garde(email)]
    email: String,
    #[garde(length(min = 12))]
    password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResRegister {
    status: String,
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    sub: String,
    role: String,
    exp: u64,
    token_version: i32,
}

#[post("/login", data = "<req_login>")]
pub async fn login(
    db: &State<DatabaseConnection>,
    config: &State<AppConfig>,
    req_login: Json<ReqRegister>,
) -> Response<Json<ResRegister>> {
    let db = db as &DatabaseConnection;

    if let Err(report) = req_login.validate() {
        return Err(ErrorResponse((
            Status::UnprocessableEntity,
            format_validation_errors_json(report).to_string(),
        )));
    }

    let config = config as &AppConfig;

    let u = match Users::find()
        .filter(users::Column::Email.eq(&req_login.email))
        .one(db)
        .await?
    {
        Some(u) => u,
        None => {
            return Err(ErrorResponse((
                Status::Unauthorized,
                "Invalid Credentials".to_string(),
            )));
        }
    };

    if !verify(&req_login.password, &u.password_hash).unwrap() {
        return Err(ErrorResponse((
            Status::Unauthorized,
            "Invalid Credentials".to_string(),
        )));
    }

    let new_token_version = rng().random_range(0..100_000);

    users::Entity::update_many()
        .col_expr(users::Column::TokenVersion, Expr::value(new_token_version))
        .filter(users::Column::UserId.eq(&u.user_id))
        .exec(db)
        .await?;
    
    let claims = Claims {
        sub: u.user_id,
        role: "users".to_string(),
        exp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 4 * 60 * 60,
        token_version: new_token_version,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .unwrap();

    Ok(SuccessResponse((Status::Ok, Json(ResRegister { status: "success".to_string(), token }))))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignUp {
    #[garde(email)]
    email: String,
    #[garde(length(min = 12))]
    password: String,
    #[garde(skip)]
    profile: JsonValue,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterResponse {
    status: String,
    message: String,
}

#[post("/register", data = "<req_register>")]
pub async fn register(
    db: &State<DatabaseConnection>,
    req_register: Json<ReqSignUp>,
) -> Response<Json<RegisterResponse>> {
    let db = db as &DatabaseConnection;

    if let Err(report) = req_register.validate() {
        return Err(ErrorResponse((
            Status::UnprocessableEntity,
            format_validation_errors_json(report).to_string(),
        )));
    }
    let initial_balance = generate_initial_balance();

    if Users::find()
        .filter(users::Column::Email.eq(&req_register.email))
        .one(db)
        .await?
        .is_some()
    {
        return Err(ErrorResponse((
            Status::UnprocessableEntity,
            "An account already exist with this email address".to_string(),
        )));
    }

    Users::insert(users::ActiveModel {
        email: Set(req_register.email.to_owned()),
        password_hash: Set(hash(&req_register.password, DEFAULT_COST).unwrap()),
        profile_data: Set(req_register.profile.clone()),
        created_at: Set(Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())),
        kyc_status: Set("pending".to_owned()),
        ..Default::default()
    })
    .exec(db)
    .await?;

    let user = Users::find()
        .filter(users::Column::Email.eq(req_register.email.to_owned()))
        .one(db)
        .await?
        .expect("User not found after insertion");

    let user_id = user.user_id;

    Account::insert(account::ActiveModel {
        user_id: Set(user_id),
        currency_code: Set("INR".to_owned()),
        balance: Set(initial_balance),
        locked_balance: Set(0.0),
        updated_at: Set(Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())),
        ..Default::default()
    })
    .exec(db)
    .await?;

    Ok(SuccessResponse((
        Status::Created,
        Json(RegisterResponse {
            status: "success".to_string(),
            message: "Account created".to_string(),
        }),
    )))
}

#[get("/me")]
pub async fn me(_db: &State<DatabaseConnection>, user: AuthenticatedUser) -> Response<String> {
    Ok(SuccessResponse((
        Status::Ok,
        "User ID :".to_string() + user.id.to_string().as_str(),
    )))
}
