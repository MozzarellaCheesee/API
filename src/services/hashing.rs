use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use crate::errors::auth::hashing::ArgonHashErrorWrapper;

type Argon2ClientResult<T> = Result<T, ArgonHashErrorWrapper>;

#[derive(Clone)]
pub struct Argon2Service {
    pub argon2: Argon2<'static>,
}

impl Argon2Service {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default()
        }
    }

    pub fn hash_target(&self, target: String) -> Argon2ClientResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        Ok(self.argon2.hash_password(target.as_bytes(), &salt)?.to_string())
    }
}