use serde::de::DeserializeOwned;

use crate::errors::AppResult;
use crate::errors::ToAppResult;
use crate::token::Token;
use crate::EnvArgs;

pub struct Jwt {
    encoding_key: jsonwebtoken::EncodingKey,
    decoding_key: jsonwebtoken::DecodingKey,
    algorithm: jsonwebtoken::Algorithm,
}

impl Jwt {
    pub fn new(env_args: &EnvArgs) -> Self {
        let encoding_key = jsonwebtoken::EncodingKey::from_secret(env_args.jwt_secret.as_bytes());
        let decoding_key = jsonwebtoken::DecodingKey::from_secret(env_args.jwt_secret.as_bytes());
        Self {
            encoding_key,
            decoding_key,
            algorithm: jsonwebtoken::Algorithm::HS512,
        }
    }

    pub fn encode<C>(&self, claims: C) -> AppResult<String>
    where
        C: serde::Serialize,
    {
        let header = jsonwebtoken::Header::new(self.algorithm);
        jsonwebtoken::encode(&header, &claims, &self.encoding_key).to_result()
    }

    pub fn decode<C>(&self, Token { token }: &Token) -> AppResult<C>
    where
        C: DeserializeOwned,
    {
        let validation = jsonwebtoken::Validation::new(self.algorithm);
        let data = jsonwebtoken::decode::<C>(token, &self.decoding_key, &validation).to_result()?;
        Ok(data.claims)
    }
}
