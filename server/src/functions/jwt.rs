use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::prisma::{identity, service_key};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: u64,    // Expires at
    sub: String, // Identity ID
    aud: String, // Service Key Domain
}

lazy_static! {
    static ref ALGORITHM: Algorithm = Algorithm::HS256;
    static ref HEADER: Header = Header::new(*ALGORITHM);
    static ref VALIDATION: Validation = Validation::new(*ALGORITHM);
}

impl Claims {
    pub fn new(user: &identity::Data, domain: &service_key::Data) -> Self {
        Claims {
            // 1 hour expiration
            exp: jsonwebtoken::get_current_timestamp() + 60 * 60,
            sub: user.id.to_string(),
            aud: domain.domain.clone(),
        }
    }

    pub fn encode(&self, domain: service_key::Data) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(
            &HEADER,
            self,
            &EncodingKey::from_secret(domain.signing_key.as_ref()),
        )
    }

    pub fn decode(
        token: &str,
        domain: service_key::Data,
    ) -> Result<Self, jsonwebtoken::errors::Error> {
        jsonwebtoken::decode(
            token,
            &DecodingKey::from_secret(domain.signing_key.as_ref()),
            &VALIDATION,
        )
        .map(|data| data.claims)
    }
}
