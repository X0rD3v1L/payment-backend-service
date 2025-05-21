use super::Response;
use super::{ErrorResponse, SuccessResponse};
use crate::utils::validations::format_validation_errors_json;
use crate::utils::validations::{ProfileUpdateContext, validate_optional_name};
use crate::{
    auth::AuthenticatedUser,
    entities::{prelude::*, users},
};
use garde::Validate;
use rocket::{
    State,
    http::Status,
    serde::{Deserialize, Serialize, json::Json},
};
use sea_orm::*;
use serde_json::json;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ProfileResponse {
    status: String,
    email: String,
    first_name: String,
    last_name: String,
}

#[get("/profile")]
pub async fn get_profile(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
) -> Response<Json<ProfileResponse>> {
    let db = db as &DatabaseConnection;

    let user_row = Users::find_by_id(user.id).one(db).await.map_err(|_| {
        ErrorResponse((
            Status::InternalServerError,
            "Failed to fetch user profile.".to_string(),
        ))
    })?;

    match user_row {
        Some(user_model) => {
            let profile_data = user_model.profile_data.clone();
            let first_name = profile_data
                .get("first_name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let last_name = profile_data
                .get("last_name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            Ok(SuccessResponse((
                Status::Ok,
                Json(ProfileResponse {
                    status: "success".to_string(),
                    email: user_model.email,
                    first_name,
                    last_name,
                }),
            )))
        }
        None => Err(ErrorResponse((
            Status::NotFound,
            "User not found.".to_string(),
        ))),
    }
}

#[derive(Debug, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
#[garde(context(ProfileUpdateContext))]
pub struct ProfileUpdateRequest {
    #[garde(custom(validate_optional_name))]
    first_name: Option<String>,
    #[garde(custom(validate_optional_name))]
    last_name: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ProfileUpdateResponse {
    status: String,
    message: &'static str,
}

#[put("/profile", data = "<data>")]
pub async fn update_profile(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    data: Json<ProfileUpdateRequest>,
) -> Response<Json<ProfileUpdateResponse>> {
    let db = db as &DatabaseConnection;

    if let Err(report) = data.validate() {
        return Err(ErrorResponse((
            Status::UnprocessableEntity,
            format_validation_errors_json(report).to_string(),
        )));
    }
    let user_model = Users::find_by_id(user.id)
        .one(db)
        .await
        .map_err(|_| {
            ErrorResponse((
                Status::InternalServerError,
                "Failed to retrieve user.".to_string(),
            ))
        })?
        .ok_or_else(|| ErrorResponse((Status::NotFound, "User not found.".to_string())))?;

    let mut new_profile = user_model.profile_data.clone();

    if let Some(fname) = &data.first_name {
        new_profile["first_name"] = json!(fname);
    }

    if let Some(lname) = &data.last_name {
        new_profile["last_name"] = json!(lname);
    }

    let mut active_model: users::ActiveModel = user_model.into();
    active_model.profile_data = Set(new_profile);

    active_model.update(db).await.map_err(|_| {
        ErrorResponse((
            Status::InternalServerError,
            "Failed to update profile.".to_string(),
        ))
    })?;

    Ok(SuccessResponse((
        Status::Ok,
        Json(ProfileUpdateResponse {
            status: "success".to_string(),
            message: "Profile updated successfully.",
        }),
    )))
}
