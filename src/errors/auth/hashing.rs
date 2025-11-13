use actix_web::body::BoxBody;
use actix_web::HttpResponse;
use thiserror::Error;
use crate::create_json_error_str;

#[derive(Debug, Error)]
pub enum ArgonHashErrorWrapper {
    #[error("Argon2 error: {0}")]
    HashingError(argon2::password_hash::Error),
}

impl From<argon2::password_hash::Error> for ArgonHashErrorWrapper {
    fn from(error: argon2::password_hash::Error) -> Self {
        ArgonHashErrorWrapper::HashingError(error)
    }
}

impl ArgonHashErrorWrapper {
    pub fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            ArgonHashErrorWrapper::HashingError(error) => {
                match error {
                    _ => HttpResponse::InternalServerError()
                        .content_type("application/json")
                        .body(create_json_error_str!("An hashing error occurred."))
                        .map_into_boxed_body(),
                }
            }
        }
    }
}

