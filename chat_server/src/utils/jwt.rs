use std::ops::Deref;

use jwt_simple::{
    claims::Claims,
    prelude::{Duration, ECDSAP256KeyPairLike, ES256KeyPair},
};

use crate::models::user::SignInUser;
use crate::{AppError, KeyPairConfig};

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

pub fn generate_jwt_token(user: SignInUser, config: &KeyPairConfig) -> Result<String, AppError> {
    let key_pair = ES256KeyPair::from_pem(&config.private_key)?;
    let claims = Claims::with_custom_claims(user, Duration::from_days(JWT_DURATION));
    Ok(key_pair.sign(claims)?)
}
