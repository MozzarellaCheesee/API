use actix_web::{post, web, HttpResponse, Responder, ResponseError};
use crate::structs::{AppState, EmailConfirm};
#[post("/v1/auth/confirm")]
pub async fn confirm(
    token: web::Query<EmailConfirm>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    let mut conn = app_state.pool.get().await.expect("Не удалось получить соединение");

    match token.confirm_email(&mut conn).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"message": "Email confirmed"})),
        Err(err) => err.error_response(),
    }
}