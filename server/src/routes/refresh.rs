use rocket::{serde::json::Json, State};

use crate::{
    error::Error,
    functions::session::{regenerate_token_pair, TokenPair},
    prisma::PrismaClient,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[post("/refresh", data = "<data>")]
pub async fn refresh(
    db: &State<PrismaClient>,
    data: Json<RefreshRequest>,
) -> Result<Json<TokenPair>, Error> {
    let db = db.inner();

    Ok(Json(
        regenerate_token_pair(db, data.refresh_token.clone()).await?,
    ))
}
