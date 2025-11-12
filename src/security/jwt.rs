use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct JwtManager {
  secret: String,
  expiration: Duration,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Claims {
    sub: String,
    exp: usize,
}


impl JwtManager {
  pub fn new(secret: String, expiration: Duration) -> Self {
      Self { secret, expiration }
  }

  pub fn generate(&self, user_id: Uuid) -> anyhow::Result<String> {
      let expires_at = SystemTime::now()
          .checked_add(self.expiration)
          .expect("exp overflow")
          .duration_since(UNIX_EPOCH)?
          .as_secs() as usize;

      let claims = Claims {
          sub: user_id.to_string(),
          exp: expires_at,
      };

      Ok(encode(
          &Header::default(),
          &claims,
          &EncodingKey::from_secret(self.secret.as_bytes()),
      )?)
  }

  pub fn validate(&self, token: &str) -> anyhow::Result<Uuid> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(self.secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(Uuid::parse_str(&data.claims.sub)?)
  }
}


