mod schema;
mod config;
mod handlers;
mod db;
mod errors;
mod services;
mod models;
mod utils;

use std::sync::Arc;
use actix_web::{App, HttpServer, middleware::Logger};
use actix_web::web::Data;
use chrono::Duration;
use regex::Regex;

use crate::config::config::EnvConfig;
use crate::handlers::ping::ping;
use crate::handlers::register::register;
use crate::services::email::EmailService;
use crate::services::hashing::Argon2Service;
use crate::services::jwt::JwtService;
use crate::services::regex::RegexService;

#[derive(Clone)]
pub struct AppState {
    pool: db::connection::DbPool,
    argon2: Arc<Argon2Service>,
    jwt: Arc<JwtService>,
    regex: Arc<RegexService>,
    email: Arc<EmailService>
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Логирование
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Загрузка данных из .env
    let load_config = EnvConfig::load_config();

    // Данные используемые приложением
    let app_state = AppState {
        pool: db::connection::establish_connection(&load_config.database_url).await,
        argon2: Arc::new(Argon2Service::new()),
        jwt: Arc::new(
            JwtService::new(
                load_config.jwt_secret.clone(),
                Duration::minutes(load_config.email_verification_expiry_minutes.clone()),
                Duration::hours(load_config.jwt_expiry_hours.clone()),
                Duration::days(load_config.refresh_token_expiry_days.clone()),
            )
        ),
        regex: Arc::new(
            RegexService::new(
                Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap(),
                vec![
                    Regex::new(r"^.{8,}$").expect("Invalid regex"),
                    Regex::new(r"[A-Z]").expect("Invalid regex"),
                    Regex::new(r"\d").expect("Invalid regex"),
                    Regex::new(r"[?&%$#@!_]").expect("Invalid regex")
                ]
            )
        ),
        email: Arc::new(
            EmailService::new(
                load_config.email_from.to_string().clone(),
                load_config.email_subject.to_string().clone(),
                load_config.email_template.to_string().clone(),
                load_config.confirm_link.to_string().clone(),
            )
        )
    };

    // Поднятие веб-сервера с определением routes и AppState
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(app_state.clone()))
            .service(ping)
            .service(register)
    })
        .bind((load_config.server_host, load_config.server_port))?
        .run()
        .await
}