use std::io::Cursor;

use bcrypt::BcryptError;
use prisma_client_rust::QueryError;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum Error {
    NotFound,
    WrongCredentials,
    CaptchaFailed,
    DatabaseError { message: String },
    InternalServerError,
}

// implement Responder for Error
impl<'r> Responder<'r, 'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        let status = match self {
            Error::NotFound => Status::NotFound,
            Error::WrongCredentials => Status::Unauthorized,
            Error::CaptchaFailed => Status::UnprocessableEntity,
            _ => Status::InternalServerError,
        };

        let string = json!(self).to_string();
        Response::build()
            .header(ContentType::new("application", "json"))
            .sized_body(string.len(), Cursor::new(string))
            .status(status)
            .ok()
    }
}

impl From<QueryError> for Error {
    fn from(err: QueryError) -> Self {
        Error::DatabaseError {
            message: err.to_string(),
        }
    }
}

impl From<BcryptError> for Error {
    fn from(_: BcryptError) -> Self {
        Error::InternalServerError
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        Error::InternalServerError
    }
}
