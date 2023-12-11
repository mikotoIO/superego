use rocket::{serde::json::Json, State};

use crate::{
    error::Error,
    functions::session::{create_session, TokenPair},
    prisma::{credential, identity, PrismaClient},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub captcha: Option<String>,
}

#[post("/login", data = "<data>")]
pub async fn login(
    db: &State<PrismaClient>,
    data: Json<LoginRequest>,
) -> Result<Json<TokenPair>, Error> {
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

    let identity = db
        .identity()
        .find_unique(identity::id::equals(credential.id))
        .exec()
        .await?
        .ok_or(Error::NotFound)?;

    Ok(Json(create_session(db, identity).await?))
}
