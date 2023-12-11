use rocket::State;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{entities::credential, error::Error};

#[post("/login")]
pub async fn login(db: &State<DatabaseConnection>) -> Result<(), Error> {
    let db = db.inner();
    let credential = credential::Entity::find()
        .one(db)
        .await?
        .ok_or(Error::WrongCredentials)?;

    Ok(())
}
