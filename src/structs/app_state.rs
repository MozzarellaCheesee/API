use argon2::{Argon2};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

#[derive(Clone)]
pub struct AppState<'a> {
    pub pool: bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    pub argon2: Argon2<'a>
}

impl<'a> AppState<'a> {
    pub fn new(pool: bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>, argon2: Argon2<'a>) -> Self {
        Self {
            pool,
            argon2
        }
    }
}