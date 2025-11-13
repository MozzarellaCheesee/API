use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct EnvConfig {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    pub refresh_token_expiry_days: i64,
    pub email_verification_expiry_minutes: i64,
    pub _resend_api_key: String,
    pub _app_url: String,
    pub email_template: String,
    pub email_from: String,
    pub email_subject: String,
    pub confirm_link: String,
}
impl EnvConfig {
    pub fn load_config() -> Self {
        dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "5002".to_string())
                .parse()
                .expect("SERVER_PORT must be a number"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            jwt_expiry_hours: env::var("JWT_EXPIRY_HOURS")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .unwrap_or(1),
            refresh_token_expiry_days: env::var("REFRESH_TOKEN_EXPIRY_DAYS")
                .unwrap_or_else(|_| "7".to_string())
                .parse()
                .unwrap_or(7),
            email_verification_expiry_minutes: env::var("EMAIL_VERIFICATION_EXPIRY_MINUTES")
                .unwrap_or_else(|_| "15".to_string())
                .parse()
                .unwrap_or(24),
            _resend_api_key: env::var("RESEND_API_KEY")
                .expect("RESEND_API_KEY must be set"),
            _app_url: env::var("APP_URL")
                .unwrap_or_else(|_| "https://api.perechitka.ru".to_string()),
            email_template: include_str!("../templates/email_template.html").to_string(),
            email_from: "Perechitka <noreply@perechitka.ru>".to_string(),
            email_subject: "Подтверди email в perechitka.ru".to_string(),
            confirm_link: "https://api.perechitka.ru/v2/auth/confirm?token={email_token}".to_string(),
        }
    }
}
