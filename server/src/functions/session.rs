use crate::{
    error::Error,
    prisma::{identity, service_key, session, PrismaClient},
};
use base64::{
    engine::{GeneralPurpose, GeneralPurposeConfig},
    Engine,
};
use sha3::{Digest, Sha3_256};

use super::jwt::Claims;
use lazy_static::lazy_static;

lazy_static! {
    static ref B64: GeneralPurpose =
        GeneralPurpose::new(&base64::alphabet::URL_SAFE, GeneralPurposeConfig::new());
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn sha3_digest(data: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    B64.encode(&hasher.finalize())
}

pub fn random_base64(num_bytes: usize) -> Result<String, Error> {
    let mut random_vec = vec![0u8; num_bytes];
    getrandom::getrandom(&mut random_vec).map_err(|_| Error::InternalServerError)?;
    Ok(B64.encode(random_vec))
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
        .identity()
        .find_unique(identity::id::equals(old_session.user_id))
        .exec()
        .await?
        .ok_or(Error::NotFound)?;

    // the client gets the raw refresh token, while the server stores the SHA3 digest
    let refresh_token = random_base64(32)?;
    let digest = sha3_digest(&refresh_token);

    let session = prisma
        .session()
        .update(
            session::id::equals(old_session.id),
            vec![session::token::set(digest.clone())],
        )
        .exec()
        .await?;

    let domain = prisma
        .service_key()
        .find_unique(service_key::id::equals(session.service_key_id))
        .exec()
        .await?
        .ok_or(Error::NotFound)?;

    Ok(TokenPair {
        access_token: Claims::new(&user, &domain).encode(domain)?,
        refresh_token,
    })
}

pub async fn create_session(
    prisma: &PrismaClient,
    domain: String,
    identity: identity::Data,
) -> Result<TokenPair, Error> {
    let refresh_token = random_base64(32)?;
    let digest = sha3_digest(&refresh_token);

    prisma
        .session()
        .create(
            digest.clone(),
            identity::id::equals(identity.id.clone()),
            service_key::domain::equals(domain.clone()),
            vec![],
        )
        .exec()
        .await?;

    let domain = prisma
        .service_key()
        .find_unique(service_key::domain::equals(domain))
        .exec()
        .await?
        .ok_or(Error::NotFound)?;

    Ok(TokenPair {
        access_token: Claims::new(&identity, &domain).encode(domain)?,
        refresh_token,
    })
}
