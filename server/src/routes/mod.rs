use rocket::serde::json::Json;

pub mod login;
pub mod register;

#[derive(Serialize)]
pub struct AppInfo {
    name: String,
    version: String,
}

#[get("/")]
pub fn index() -> Json<AppInfo> {
    Json(AppInfo {
        name: "superego".to_string(),
        version: "0.0.0".to_string(),
    })
}
