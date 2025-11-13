use bb8::RunError;
use diesel_async::pooled_connection::PoolError;
use thiserror::Error;
use crate::define_app_error;
use crate::errors::auth::email::EmailSendErrorWrapper;
use crate::errors::auth::hashing::ArgonHashErrorWrapper;
use crate::errors::auth::jwt::JwtServiceErrorWrapper;
use crate::errors::db::diesel_error::{DieselErrorWrapper};
use crate::errors::db::user::UserCreationErrorWrapper;

define_app_error! {

    #[derive(Debug, Error)]
    pub enum AppError {
        /// Database Errors
        #[error(transparent)]
        DieselError(#[from] DieselErrorWrapper),
        /// User Errors
        #[error(transparent)]
        UserCreationError(#[from] UserCreationErrorWrapper),
        /// Hashing Errors
        #[error(transparent)]
        HashingError(#[from] ArgonHashErrorWrapper),
        /// Jwt Error
        #[error(transparent)]
        JwtError(#[from] JwtServiceErrorWrapper),
        /// EmailSend Error
        #[error(transparent)]
        EmailError(#[from] EmailSendErrorWrapper)
    }

    response: {
        delegates: [
            DieselError,
            UserCreationError,
            HashingError,
            JwtError,
            EmailError
        ],
        custom: {}
    }

}

impl From<RunError<PoolError>> for AppError {
    fn from(error: RunError<PoolError>) -> Self {
        AppError::DieselError(DieselErrorWrapper::from(error))
    }
}