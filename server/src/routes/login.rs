use rocket::{serde::json::Json, State};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{entities::credential, error::Error};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    email: String,
    password: String,
    captcha: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    access_token: String,
    refresh_token: String,
}

#[post("/login", data = "<data>")]
pub async fn login(
    db: &State<DatabaseConnection>,
    data: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Error> {
    let db = db.inner();
    let credential = credential::Entity::find()
        .filter(credential::Column::Email.eq(data.email.trim().to_lowercase()))
        .one(db)
        .await?
        .ok_or(Error::WrongCredentials)?;

    let verify = bcrypt::verify(&data.password, &credential.passhash)?;
    if !verify {
        return Err(Error::WrongCredentials);
    }

    todo!("implement the access/refresh token generation");

    Ok(Json(LoginResponse {
        access_token: "".to_string(),
        refresh_token: "".to_string(),
    }))
}
