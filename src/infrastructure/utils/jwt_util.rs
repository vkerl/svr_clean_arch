use chrono;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::domain::models::{auth::JwtPayload, user::User};

pub fn decode_jwt_token(
    token: &str,
    secret_key: &str,
) -> jsonwebtoken::errors::Result<jsonwebtoken::TokenData<JwtPayload>> {
    decode::<JwtPayload>(
        token,
        &DecodingKey::from_base64_secret(secret_key).unwrap(),
        &Validation::default(),
    )
}

pub fn encode_jwt_token(
    user: &User,
    secret_key: &str,
    token_expire_seconds: i64,
) -> jsonwebtoken::errors::Result<String> {
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(token_expire_seconds))
        .expect("valid timestamp")
        .timestamp();

    let payload = JwtPayload {
        uid: user.uid,
        exp,
    };

    let header = Header {
        typ: None,
        alg: Algorithm::HS256,
        cty: None,
        jku: None,
        jwk: None,
        kid: None,
        x5u: None,
        x5c: None,
        x5t: None,
        x5t_s256: None,
    };

    encode(
        &header,
        &payload,
        &EncodingKey::from_base64_secret(secret_key).unwrap(),
    )
}