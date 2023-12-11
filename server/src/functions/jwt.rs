use lazy_static::lazy_static;

use crate::entities::user;

use jsonwebtoken::{Algorithm, EncodingKey, Header};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: u64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)

              // aud: String, // Audience
              // iss: String, // Optional. Issuer
}

impl Claims {
    pub fn new(user: &user::Model) -> Self {
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
    static ref ENCODING_KEY: EncodingKey = EncodingKey::from_secret("secret".as_ref());
}
