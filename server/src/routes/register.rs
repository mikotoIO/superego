use rocket::{serde::json::Json, State};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, TransactionTrait};
use uuid::Uuid;

use crate::{
    entities::{credential, user},
    error::Error,
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
pub async fn register(
    db: &State<DatabaseConnection>,
    data: Json<RegisterRequest>,
) -> Result<(), Error> {
    let db = db.inner();
    let txn = db.begin().await?;

    let user = user::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4().to_string()),
        username: ActiveValue::Set(data.username.clone()),
        kind: ActiveValue::set(None),
        display_name: ActiveValue::Set(data.username.clone()),
    }
    .insert(&txn)
    .await?;

    let _ = credential::ActiveModel {
        id: ActiveValue::Set(user.id.clone()),
        email: ActiveValue::Set(data.email.clone()),
        passhash: ActiveValue::Set(bcrypt::hash(&data.password, bcrypt::DEFAULT_COST)?),
    }
    .insert(&txn)
    .await?;

    txn.commit().await?;

    Ok(())
}
