
use actix_web::{post, web, HttpResponse, Responder, Result};
use crate::{create_json_registry_response, errors::app_error::AppError, models::dto::requests::register::RegisterRequest, AppState};
use serde_json::json;
use crate::db::email::create_email_verify;
use crate::db::user::{create_user, find_user_by_email_or_username};
use crate::errors::db::user::UserCreationErrorWrapper;
use crate::services::validation::validate_user_fields;
use crate::utils::helpers::{check_empty_register_request, check_user_exists_request};

#[post("/v2/auth/register")]
pub async fn register(
    req: web::Json<RegisterRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {

    // Проверка на пустые поля в запросе
    if check_empty_register_request(&req) {
        return Err(AppError::UserCreationError(UserCreationErrorWrapper::EmptyFields));
    }

    // Получение соединения из пула
    let mut conn = state.pool.get().await?;

    // Проверка существует ли пользователь с таким email и username
    let matches = find_user_by_email_or_username(&mut conn, &req.email, &req.username).await?;
    check_user_exists_request(&req, matches)?;


    // Получение клиента Regex и Argon2 для проверки и хеширования пароля
    let regex = state.regex.clone();
    let argon2 = state.argon2.clone();
    let email = state.email.clone();
    let jwt = state.jwt.clone();

    // Проверка валидности введенных данных пользователем
    validate_user_fields(&req.username, &req.email, &req.password, &regex)?;

    // Создание пользователя
    let user = create_user(
        &mut conn,
        &req,
        argon2.hash_target(req.password.clone())?,
    ).await?;

    // Создание токенов верификации почты
    let token = jwt.create_verify_token()?;
    let token_hash = argon2.hash_target(token.clone())?;

    // Отправка токенов верификации в бд
    create_email_verify(&mut conn, &user, token_hash).await?;

    // Отправка верификации на почту
    email.send_verify(user.username.clone(), user.email.clone(), token).await?;

    Ok(HttpResponse::Ok().json(create_json_registry_response!(true, user.id, user.email, user.first_name, user.last_name, user.is_email_verified)))
}