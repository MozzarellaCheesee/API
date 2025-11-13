use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use thiserror::Error;
use crate::create_json_error_str;

#[derive(Debug, Error)]
pub enum UserCreationErrorWrapper {
    #[error("Account with this email and username already exists")]
    EmailAndUsernameAlreadyExists,
    #[error("Account with this email already exists")]
    EmailAlreadyExists,
    #[error("Account with this username already exists")]
    UsernameAlreadyExists,
    #[error("Empty fields")]
    EmptyFields,
    #[error("Wrong password")]
    WrongPassword,
    #[error("Wrong username")]
    WrongUsername,
    #[error("Wrong email")]
    WrongEmail,
}
impl ResponseError for UserCreationErrorWrapper {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            UserCreationErrorWrapper::EmailAndUsernameAlreadyExists => {
                HttpResponse::Conflict()
                    .content_type("application/json")
                    .body(create_json_error_str!("User with this email and username already exists"))
                    .map_into_boxed_body()
            }
            UserCreationErrorWrapper::EmailAlreadyExists => {
                HttpResponse::Conflict()
                    .content_type("application/json")
                    .body(create_json_error_str!("User with this email already exists"))
                    .map_into_boxed_body()
            }
            UserCreationErrorWrapper::UsernameAlreadyExists => {
                HttpResponse::Conflict()
                    .content_type("application/json")
                    .body(create_json_error_str!("User with this username already exists"))
                    .map_into_boxed_body()
            }
            UserCreationErrorWrapper::WrongPassword => {
                HttpResponse::BadRequest()
                    .content_type("application/json")
                    .body(create_json_error_str!("The password must contain at least 8 characters, one or more uppercase letters and a special character."))
                    .map_into_boxed_body()
            }
            UserCreationErrorWrapper::WrongUsername => {
                HttpResponse::BadRequest()
                    .content_type("application/json")
                    .body(create_json_error_str!("The username can only contain letters and the `_` sign."))
                    .map_into_boxed_body()
            }
            UserCreationErrorWrapper::WrongEmail => {
                HttpResponse::BadRequest()
                    .content_type("application/json")
                    .body(create_json_error_str!("The email can only contain letters, nums and the `@` `.` sign."))
                    .map_into_boxed_body()
            }
            UserCreationErrorWrapper::EmptyFields => {
                HttpResponse::BadRequest()
                    .content_type("application/json")
                    .body(create_json_error_str!("Empty fields in request"))
                    .map_into_boxed_body()
            }
        }
    }
}