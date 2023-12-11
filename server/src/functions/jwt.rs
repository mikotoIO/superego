use std::env;

use lazy_static::lazy_static;

use jsonwebtoken::{Algorithm, EncodingKey, Header};

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
}

lazy_static! {
    static ref HEADER: Header = Header::new(Algorithm::HS256);
    // global secret variable
    static ref SECRET: String = env::var("SECRET").expect("SECRET must be set");
    static ref ENCODING_KEY: EncodingKey = EncodingKey::from_secret(SECRET.as_ref());
}
