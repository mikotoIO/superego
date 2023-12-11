use rocket::{serde::json::Json, State};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entities::{credential, user},
    error::Error,
    functions::jwt,
};

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

    let user = user::Entity::find_by_id(credential.id)
        .one(db)
        .await?
        .ok_or(Error::NotFound)?;

    Ok(Json(LoginResponse {
        access_token: jwt::Claims::new(&user).encode()?,
        refresh_token: "".to_string(), // TODO: implement refresh token
    }))
}
