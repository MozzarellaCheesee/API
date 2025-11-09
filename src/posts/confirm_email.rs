use actix_web::{post, web, HttpResponse, Responder, ResponseError};
use crate::structs::EmailConfirm;
use crate::ConnPool;

#[post("/v1/auth/confirm")]
pub async fn confirm(
    token: web::Query<EmailConfirm>,
    pool: web::Data<ConnPool>,
) -> impl Responder {
    let mut conn = pool.get().await.expect("Не удалось получить соединение");

    match token.confirm_email(&mut conn).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"message": "Email confirmed"})),
        Err(err) => err.error_response(),
    }
}