use prisma::PrismaClient;

use dotenv::dotenv;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

pub mod error;
pub mod functions;
#[allow(warnings, unused)]
pub mod prisma;
pub mod routes;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let db = PrismaClient::_builder().build().await.unwrap();

    rocket::build().manage(db).mount(
        "/",
        routes![
            routes::index,
            routes::register::register,
            routes::login::login
        ],
    )
}
