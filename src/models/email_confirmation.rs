use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::email_confirmation)]
#[diesel(primary_key(user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EmailConfirmation {
    pub user_id: i64,
    pub token: Uuid,
    pub expires_at: NaiveDateTime,
    pub confirmed: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::email_confirmation)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEmailConfirmation<'a> {
    pub user_id: &'a i64,
    pub token: &'a Uuid,
}