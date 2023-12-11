use std::env;

use lazy_static::lazy_static;

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::prisma::identity;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: u64,    // Expires at
    sub: String, // Identity ID
}

impl Claims {
    pub fn new(user: &identity::Data) -> Self {
        Claims {
            // 1 hour expiration
            exp: jsonwebtoken::get_current_timestamp() + 60 * 60,
            sub: user.id.to_string(),
        }
    }

    pub fn encode(&self) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(&HEADER, self, &ENCODING_KEY)
    }

    pub fn decode(token: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        jsonwebtoken::decode(token, &DECODING_KEY, &VALIDATION).map(|data| data.claims)
    }
}

lazy_static! {
    static ref ALGORITHM: Algorithm = Algorithm::HS256;

    static ref HEADER: Header = Header::new(*ALGORITHM);
    // global secret variable
    static ref SECRET: String = env::var("SECRET").expect("SECRET must be set");

    static ref ENCODING_KEY: EncodingKey = EncodingKey::from_secret(SECRET.as_ref());
    static ref DECODING_KEY: DecodingKey = DecodingKey::from_secret(SECRET.as_ref());
    static ref VALIDATION: Validation = Validation::new(*ALGORITHM);
}
