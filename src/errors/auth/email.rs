use actix_web::body::BoxBody;
use actix_web::HttpResponse;
use thiserror::Error;
use crate::create_json_error_str;

#[derive(Debug, Error)]
pub enum EmailSendErrorWrapper {
    #[error("Argon2 error: {0}")]
    EmailSendError(resend_rs::Error),
}

impl From<resend_rs::Error> for EmailSendErrorWrapper {
    fn from(error: resend_rs::Error) -> Self {
        EmailSendErrorWrapper::EmailSendError(error)
    }
}

impl EmailSendErrorWrapper {
    pub fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            EmailSendErrorWrapper::EmailSendError(error) => {
                match error {
                    _ => HttpResponse::InternalServerError()
                        .content_type("application/json")
                        .body(create_json_error_str!("There was an error sending the letter."))
                        .map_into_boxed_body(),
                }
            }
        }
    }
}

