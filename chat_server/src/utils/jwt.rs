use std::collections::HashSet;
use std::ops::Deref;

use jwt_simple::prelude::*;
use jwt_simple::common::VerificationOptions;
use jwt_simple::prelude::{EdDSAKeyPairLike, EdDSAPublicKeyLike};
use crate::models::user::SignInUser;
use crate::{AppError, KeyPairConfig, User};

const JWT_DURATION: u64 = 7; // 7 days

pub struct EncodingKey(jwt_simple::prelude::Ed25519KeyPair);
pub struct DecodingKey(jwt_simple::prelude::Ed25519PublicKey);

impl Deref for EncodingKey {
    type Target = jwt_simple::prelude::Ed25519KeyPair;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for DecodingKey {
    type Target = jwt_simple::prelude::Ed25519PublicKey;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl EncodingKey {
    pub fn load(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519KeyPair::from_pem(pem)?))
    }
    
    pub fn sign(user: User,key: &EncodingKey) -> Result<String, AppError> {
        let claims = Claims::with_custom_claims(user, Duration::from_days(JWT_DURATION));
        let claims = claims.with_issuer("chat_server").with_audience("chat_web");
        Ok(key.sign(claims)?)
    }
}

impl DecodingKey {
    pub fn load(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519PublicKey::from_pem(pem)?))
    }
    pub fn verify(token: &str, key: &DecodingKey) -> Result<User, AppError> {
        let mut options = VerificationOptions::default();
        options.allowed_issuers = Some(HashSet::from_strings(&["chat_server"]));
        let claims = key.verify_token::<User>(token, None)?;
        Ok(claims.custom)
    }
}

pub fn generate_jwt_token(user: SignInUser, config: &KeyPairConfig) -> Result<String, AppError> {
    let key_pair = ES256KeyPair::from_pem(&config.private_key)?;
    let claims = Claims::with_custom_claims(user, Duration::from_days(JWT_DURATION));
    Ok(key_pair.sign(claims)?)
}
