use crate::{
    error::Error,
    prisma::{session, user, PrismaClient},
};
use sha3::{Digest, Sha3_256};

use super::jwt::Claims;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPair {
    access_token: String,
    refresh_token: String,
}

pub fn sha3_digest(data: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub async fn regenerate_token_pair(
    prisma: &PrismaClient,
    token: String,
) -> Result<TokenPair, Error> {
    let old_session = prisma
        .session()
        .find_unique(session::token::equals(sha3_digest(&token)))
        .exec()
        .await?
        .ok_or(Error::NotFound)?;

    let user = prisma
        .user()
        .find_unique(user::id::equals(old_session.user_id))
        .exec()
        .await?
        .ok_or(Error::NotFound)?;

    Ok(TokenPair {
        access_token: Claims::new(&user).encode()?,
        refresh_token: "".to_string(),
    })
}
