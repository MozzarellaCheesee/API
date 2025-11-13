use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::errors::db::diesel_error::DieselErrorWrapper;
use crate::models::dto::requests::register::RegisterRequest;
use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::users;
use crate::schema::users::{email, username};

type DieselResult<T> = Result<T, DieselErrorWrapper>;

pub async fn find_user_by_email_or_username(
    conn: &mut AsyncPgConnection,
    search_email: &str,
    search_username: &str,
) -> DieselResult<Vec<(String, String)>> {
    let result = users
        .filter(email.eq(search_email).or(username.eq(search_username)))
        .select((email, username))
        .load::<(String, String)>(conn)
        .await?;
    Ok(result)
}

pub async fn create_user(
    conn: &mut AsyncPgConnection,
    req: &RegisterRequest,
    password_hash: String,
) -> DieselResult<User> {
    let new_user = NewUser {
        email: req.email.clone(),
        username: req.username.clone(),
        password_hash: password_hash.to_string(),
        first_name: req.first_name.clone(),
        last_name: req.last_name.clone(),
    };

    let added_user = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn)
        .await?;

    Ok(added_user)
}
