use actix_web::{post, HttpResponse, Responder};
use crate::errors::app_error::AppError;

#[post("/v2/auth/ping")]
pub async fn ping() -> Result<impl Responder, AppError> {
    Ok(HttpResponse::Ok().body("pong"))
}