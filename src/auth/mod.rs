use jsonwebtoken::{DecodingKey, Validation, decode};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
    serde::{Deserialize, Serialize},
};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};

use crate::{
    AppConfig,
    entities::{prelude::Users, users},
};

pub struct AuthenticatedUser {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: u64,
    pub token_version: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = match req.headers().get_one("token") {
            Some(t) => t,
            None => return Outcome::Error((Status::Unauthorized, "Token absent".to_string())),
        };

        let config = match req.rocket().state::<AppConfig>() {
            Some(cfg) => cfg,
            None => return Outcome::Error((Status::InternalServerError, "Missing config".to_string())),
        };

        let db = match req.rocket().state::<DatabaseConnection>() {
            Some(d) => d,
            None => return Outcome::Error((Status::InternalServerError, "Missing DB connection".to_string())),
        };

        let token_data = match decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        ) {
            Ok(data) => data,
            Err(_) => return Outcome::Error((Status::Unauthorized, "Invalid token".to_string())),
        };

        let claims = token_data.claims;

        let user = match Users::find()
            .filter(users::Column::UserId.eq(&claims.sub))
            .one(db)
            .await
        {
            Ok(Some(u)) => u,
            _ => return Outcome::Error((Status::Unauthorized, "User not found".to_string())),
        };

        if user.token_version != claims.token_version {
            return Outcome::Error((Status::Unauthorized, "Token expired or invalidated".to_string()));
        }

        Outcome::Success(AuthenticatedUser { id: claims.sub })
    }
}
