use actix_web::{HttpResponse};
use chrono::Utc;
use diesel_async::AsyncPgConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::CustomError;
use crate::models::{EmailConfirmation};
use crate::schema::email_confirmation;
use diesel_async::{RunQueryDsl};
use diesel::{QueryDsl, ExpressionMethods};
use crate::schema::users;

#[derive(Deserialize, Serialize)]
pub struct EmailConfirm {
    pub token: Uuid
}

impl EmailConfirm {

    async fn is_valid(&self, conn: &mut AsyncPgConnection) -> Result<EmailConfirmation, CustomError> {

        let confirmation = match email_confirmation::table
            .filter(email_confirmation::token.eq(&self.token))
            .first::<EmailConfirmation>(conn)  // ← .first()!
            .await
        {
            Ok(c) => Some(c),
            Err(diesel::result::Error::NotFound) => None,
            Err(e) => return Err(CustomError::from(e)),
        };

        let confirmation = confirmation.ok_or(CustomError::EmailTokenNotFound("Страница с подтверждением не найдена".to_string()))?;

        let now = Utc::now().naive_utc();
        if confirmation.expires_at <= now {
            return Err(CustomError::EmailExpired("Ссылка просрочена".to_string()));
        }
        if confirmation.confirmed {
            return Err(CustomError::EmailUsed("E-mail уже подтвержден".to_string()));
        }

        Ok(confirmation)
    }

    pub async fn confirm_email(&self, conn: &mut AsyncPgConnection) -> Result<HttpResponse, CustomError> {
        let confirmation = self.is_valid(conn).await?;
        diesel::update(email_confirmation::table.find(confirmation.user_id))
            .set(email_confirmation::confirmed.eq(true))
            .execute(conn)
            .await
            .map_err(|_| CustomError::EmailUsed("E-mail уже подтвержден".to_string()))?;

        diesel::update(users::table.find(confirmation.user_id))
            .set(users::e_mail_confirmed.eq(true))
            .execute(conn)
            .await
            .map_err(|_| CustomError::EmailUsed("E-mail уже подтвержден".to_string()))?;

        Ok(HttpResponse::Ok().into())
    }
}