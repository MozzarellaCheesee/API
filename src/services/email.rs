use serde::{Deserialize, Serialize};
use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::Resend;
use crate::errors::auth::email::EmailSendErrorWrapper;

type EmailSendServiceResult<T> = Result<T, EmailSendErrorWrapper>;

#[derive(Deserialize, Serialize)]
pub struct EmailService {
    from: String,
    subject: String,
    html: String,
    confirm_link: String,
}

impl EmailService {
    pub fn new(from: String, subject: String, html: String, confirm_link: String) -> Self {
        Self {
            from,
            subject,
            html,
            confirm_link
        }
    }

    pub async  fn send_verify(&self, user_name: String, to: String, token: String) -> EmailSendServiceResult<()> {
        let to = vec![to];
        let resend = Resend::default();
        let link = self.confirm_link.replace("{email_token}", &token);
        let html = self.html.replace("{user_name}", &user_name).replace("{link}", &link);


        let email_data = CreateEmailBaseOptions::new(&self.from, to, &self.subject)
            .with_html(&html);

        resend.emails.send(email_data).await?;

        Ok(())
    }
}