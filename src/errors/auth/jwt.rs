use actix_web::body::BoxBody;
use actix_web::HttpResponse;
use thiserror::Error;
use crate::create_json_error_str;

#[derive(Debug, Error)]
pub enum JwtServiceErrorWrapper {
    #[error("Argon2 error: {0}")]
    CreationError(jsonwebtoken::errors::Error),
}

impl From<jsonwebtoken::errors::Error> for JwtServiceErrorWrapper {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        JwtServiceErrorWrapper::CreationError(error)
    }
}

impl JwtServiceErrorWrapper {
    pub fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            JwtServiceErrorWrapper::CreationError(error) => {
                match error {
                    _ => HttpResponse::InternalServerError()
                        .content_type("application/json")
                        .body(create_json_error_str!("An create jwt error occurred."))
                        .map_into_boxed_body(),
                }
            }
        }
    }
}

