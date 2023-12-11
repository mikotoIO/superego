use rocket::{serde::json::Json, State};

use crate::{
    error::Error,
    functions::session::{create_session, TokenPair},
    prisma::{identity, PrismaClient},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub captcha: Option<String>,
}

#[post("/register", data = "<data>")]
pub async fn register(
    db: &State<PrismaClient>,
    data: Json<RegisterRequest>,
) -> Result<Json<TokenPair>, Error> {
    let db = db.inner();

    let (identity, _) = db
        ._transaction()
        .run::<Error, _, _, _>(|db| async move {
            let identity = db
                .identity()
                .create(
                    data.username.clone().to_lowercase(),
                    data.username.clone(),
                    vec![],
                )
                .exec()
                .await?;
            let credential = db
                .credential()
                .create(
                    data.email.clone().trim().to_lowercase(),
                    bcrypt::hash(&data.password, bcrypt::DEFAULT_COST)?,
                    identity::id::equals(identity.id.clone()),
                    vec![],
                )
                .exec()
                .await?;

            Ok((identity, credential))
        })
        .await?;

    Ok(Json(create_session(db, identity).await?))
}
