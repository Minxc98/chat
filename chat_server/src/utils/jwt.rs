use std::collections::HashSet;
use std::ops::Deref;

use crate::models::user::SignInUser;
use crate::{AppError, KeyPairConfig, User};
use jwt_simple::common::VerificationOptions;
use jwt_simple::prelude::*;
use jwt_simple::prelude::{EdDSAKeyPairLike, EdDSAPublicKeyLike};
use tracing::warn;

const JWT_DURATION: u64 = 7; // 7 days

pub struct EncodingKey(jwt_simple::prelude::Ed25519KeyPair);

pub struct DecodingKey(jwt_simple::prelude::Ed25519PublicKey);

impl EncodingKey {
    pub fn load(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519KeyPair::from_pem(pem)?))
    }

    pub fn sign(&self, user: impl Into<User>) -> Result<String, AppError> {
        let user = user.into();
        let claims = Claims::with_custom_claims(user, Duration::from_days(JWT_DURATION));
        let claims = claims.with_issuer("chat_server").with_audience("chat_web");
        Ok(self.0.sign(claims)?)
    }
}

impl DecodingKey {
    pub fn load(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519PublicKey::from_pem(pem)?))
    }
    pub fn verify(&self, token: &str) -> Result<User, AppError> {
        // 修改方法签名
        let mut options = VerificationOptions::default();
        options.allowed_issuers = Some(HashSet::from_strings(&["chat_server"]));
        // 添加验证选项参数
        let claims = self.0.verify_token::<User>(token, Some(options))?;
        Ok(claims.custom)
    }
}

pub fn generate_jwt_token(user: SignInUser, config: &KeyPairConfig) -> Result<String, AppError> {
    let key_pair = Ed25519KeyPair::from_pem(&config.private_key)?;
    let claims = Claims::with_custom_claims(user, Duration::from_days(JWT_DURATION));
    Ok(key_pair.sign(claims)?)
}

// ... 文件原有代码保持不变 ...

#[cfg(test)]
mod tests {
    use super::*;
    use jwt_simple::prelude::Ed25519KeyPair;

    #[test]
    fn test_keypair_loading() {
        // 生成测试用密钥对
        let private_key = include_str!("../../fixtures/encoding.pem");
        let public_key = include_str!("../../fixtures/decoding.pem");

        // 测试EncodingKey加载
        let encoding_key = EncodingKey::load(private_key);
        assert!(encoding_key.is_ok(), "Failed to load encoding key from PEM");

        // 测试DecodingKey加载
        let decoding_key = DecodingKey::load(&public_key);
        assert!(decoding_key.is_ok(), "Failed to load decoding key from PEM");
    }
}
