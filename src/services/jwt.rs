use std::ops::Add;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::errors::auth::jwt::JwtServiceErrorWrapper;

type JwtServiceResult<T> = Result<T, JwtServiceErrorWrapper>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyClaims {
    pub sn: Uuid,
    exp: i64,
}

// pub struct AuthTokens{
//     pub access: String,
//     pub refresh: String
// }

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    _decoding_key: DecodingKey,
    header: Header,
    _validation: Validation,
    verify_ttl_m: Duration,
    pub _access_token_ttl_h: Duration,
    pub _refresh_token_ttl_d: Duration,
}

impl JwtService {
    pub fn new(
        secret: String,
        verify_ttl_m: Duration,
        _access_token_ttl_h: Duration,
        _refresh_token_ttl_d: Duration
    ) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            _decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            header: Header::new(Algorithm::HS256),
            _validation: Validation::new(Algorithm::HS256),
            verify_ttl_m,
            _access_token_ttl_h,
            _refresh_token_ttl_d,
        }
    }

    pub fn create_verify_token(&self) -> JwtServiceResult<String> {
        let exp = Utc::now().add(self.verify_ttl_m).timestamp();
        let sn = Uuid::new_v4();
        let claims = VerifyClaims {
            sn,
            exp
        };

        let token = jsonwebtoken::encode(&self.header, &claims, &self.encoding_key)?;

        Ok(token)
    }
}
