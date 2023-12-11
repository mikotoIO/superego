use std::env;

use sea_orm::Database;

use dotenv::dotenv;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

pub mod entities;
pub mod error;
pub mod functions;
pub mod routes;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let db = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(db)
        .await
        .expect("Failed to connect to database");
    rocket::build()
        .manage(db)
        .mount("/", routes![routes::index])
}
