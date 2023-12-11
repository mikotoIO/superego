use rocket::{serde::json::Json, State};

use crate::{
    error::Error,
    prisma::{user, PrismaClient},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    email: String,
    username: String,
    password: String,
    captcha: Option<String>,
}

#[post("/register", data = "<data>")]
pub async fn register(db: &State<PrismaClient>, data: Json<RegisterRequest>) -> Result<(), Error> {
    let db = db.inner();

    db._transaction()
        .run::<Error, _, _, _>(|db| async move {
            let user = db
                .user()
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
                    user::id::equals(user.id.clone()),
                    vec![],
                )
                .exec()
                .await?;

            Ok((user, credential))
        })
        .await?;

    Ok(())
}
