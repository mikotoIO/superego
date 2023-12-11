use rocket::{serde::json::Json, State};

use crate::{
    error::Error,
    functions::jwt,
    prisma::{credential, user, PrismaClient},
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
    db: &State<PrismaClient>,
    data: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Error> {
    let db = db.inner();

    let credential = db
        .credential()
        .find_unique(credential::email::equals(data.email.trim().to_lowercase()))
        .exec()
        .await?
        .ok_or(Error::WrongCredentials)?;

    let verify = bcrypt::verify(&data.password, &credential.passhash)?;
    if !verify {
        return Err(Error::WrongCredentials);
    }

    let user = db
        .user()
        .find_unique(user::id::equals(credential.id))
        .exec()
        .await?
        .ok_or(Error::NotFound)?;

    Ok(Json(LoginResponse {
        access_token: jwt::Claims::new(&user).encode()?,
        refresh_token: "".to_string(), // TODO: implement refresh token
    }))
}
