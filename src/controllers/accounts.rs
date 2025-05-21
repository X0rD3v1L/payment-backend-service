use super::Response;
use super::{ErrorResponse, SuccessResponse};
use crate::{
    auth::AuthenticatedUser,
    entities::{account, prelude::*},
};
use rocket::{
    State,
    http::Status,
    serde::{Serialize, json::Json},
};
use sea_orm::*;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BalanceResponse {
    status: String,
    balance: f32,
    currency_code: String,
}

#[get("/balance")]
pub async fn balance(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
) -> Response<Json<BalanceResponse>> {
    let db = db as &DatabaseConnection;
    let account = Account::find()
        .filter(account::Column::UserId.eq(user.id))
        .one(db)
        .await
        .map_err(|_| {
            ErrorResponse((
                Status::InternalServerError,
                "Failed to retrieve account information.".to_string(),
            ))
        })?;

    match account {
        Some(acc) => Ok(SuccessResponse((
            Status::Ok,
            Json(BalanceResponse {
                status: "success".to_string(),
                balance: acc.balance,
                currency_code: acc.currency_code,
            }),
        ))),
        None => Err(ErrorResponse((
            Status::NotFound,
            "Account not found for the given user ID.".to_string(),
        ))),
    }
}
