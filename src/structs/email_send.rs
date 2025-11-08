use resend_rs::{Resend, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use dotenv::dotenv;
use resend_rs::types::CreateEmailBaseOptions;

static EMAIL_TEMPLATE: &str = include_str!("../templates/email_template.html");

#[derive(Deserialize, Serialize)]
pub struct EmailSend {
    from: String,
    to: Vec<String>,
    subject: String,
    html: String,
}

pub async fn send_confirm_email(email: &str, token: Uuid, user_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv();
    let from = "Perechitka <noreply@perechitka.ru>";
    let to = vec![email.to_string()];
    let subject = "Подтверди email в Web Library!";
    let resend = Resend::default();
    let confirm_link = format!("https://api.perechitka.ru/v1/auth/confirm?token={}", token);
    let html = EMAIL_TEMPLATE.replace("{user_name}", user_name).replace("{link}", confirm_link.as_str());

    let email_data = CreateEmailBaseOptions::new(from, to, subject)
        .with_html(&html);

    resend.emails.send(email_data).await?;

    Ok(())
}