use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use bb8::RunError;
use diesel_async::pooled_connection::PoolError;
use crate::create_json_error_str;

#[derive(Debug, Error)]
pub enum DieselErrorWrapper {
    #[error("Diesel error: {0}")]
    DieselError(diesel::result::Error),
    #[error("Pool error: {0}")]
    GetConnectionError(RunError<PoolError>),
}

impl From<diesel::result::Error> for DieselErrorWrapper {
    fn from(error: diesel::result::Error) -> Self {
        DieselErrorWrapper::DieselError(error)
    }
}

impl From<RunError<PoolError>> for DieselErrorWrapper {
    fn from(error: RunError<PoolError>) -> Self { DieselErrorWrapper::GetConnectionError(error) }
}

impl ResponseError for DieselErrorWrapper {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            DieselErrorWrapper::DieselError(diesel_error) => {
                match diesel_error {
                    diesel::result::Error::NotFound => {
                        HttpResponse::NotFound()
                            .content_type("application/json")
                            .body(create_json_error_str!("The requested resource was not found."))
                            .map_into_boxed_body()
                    }
                    diesel::result::Error::DatabaseError(kind, _info) if *kind == diesel::result::DatabaseErrorKind::UniqueViolation => {
                        HttpResponse::Conflict()
                            .content_type("application/json")
                            .body(create_json_error_str!("A record with this value already exists."))
                            .map_into_boxed_body()
                    }
                    _ => HttpResponse::InternalServerError()
                        .content_type("application/json")
                        .body(create_json_error_str!("An internal database error occurred."))
                        .map_into_boxed_body(),
                    }
            }
            DieselErrorWrapper::GetConnectionError(error) => {
                match error {
                    _ => HttpResponse::InternalServerError()
                        .content_type("application/json")
                        .body(create_json_error_str!("An internal database pool error occurred."))
                        .map_into_boxed_body(),
                }
            }
        }
    }
}


