use actix_web::{post, web, HttpResponse, Responder, ResponseError};
use crate::error::CustomError;
use crate::structs::{AppState, AuthLogoutInput, Claims};
use actix_web_httpauth::extractors::bearer::BearerAuth;

#[post("/v1/auth/logout")]
pub async fn logout(
    token: web::Json<AuthLogoutInput>,
    app_state: web::Data<AppState<'_>>,
    auth: BearerAuth
) -> impl Responder {
    let mut conn = app_state.pool.get().await.expect("Не удалось получить соединение");

    let data: Claims = match token.transcript_token(&auth.token()) {
        Ok(data) => data,
        Err(err) => return match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                CustomError::ExpiredTokenError("Токен просрочен".to_string()).error_response()
            }
            _ => CustomError::TokenCreationError(err).error_response(),
        }
    };

    match token.set_revoked(&mut conn, &data.jti).await {
        Ok(result) => {
            if result.revoked {
                HttpResponse::Ok().json(&token).into()
            } else {
                CustomError::RevorkTokenError("Токен не был отзован".to_string()).error_response()
            }
        }
        Err(err) => CustomError::DbError(err).error_response(),
    }
}
