use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::errors::db::diesel_error::DieselErrorWrapper;
use crate::models::email::{Email, NewEmail};
use crate::models::user::User;
use crate::schema::email_verification_tokens::dsl::email_verification_tokens;


type DieselResult<T> = Result<T, DieselErrorWrapper>;

pub async fn create_email_verify(
    conn: &mut AsyncPgConnection,
    user: &User,
    token_hash: String,
) -> DieselResult<()> {
    let new_email = NewEmail {
        user_id: user.id,
        token_hash,
    };

    diesel::insert_into(email_verification_tokens)
        .values(&new_email)
        .get_result::<Email>(conn)
        .await?;

    Ok(())
}