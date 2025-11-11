use argon2::{password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};
use rand_core::OsRng;

pub fn hash_password(raw: &str) -> anyhow::Result<String> {
  let salt = SaltString::generate(&mut OsRng);
  let hash = Argon2::default()
      .hash_password(raw.as_bytes(), &salt)
      .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
      .to_string();
  Ok(hash)
}

pub fn verify_password(raw: &str, hash: &str) -> anyhow::Result<bool> {
  let parsed_hash = PasswordHash::new(hash)
      .map_err(|e| anyhow::anyhow!("Failed to parse password hash: {}", e))?;
  Ok(Argon2::default().verify_password(raw.as_bytes(), &parsed_hash).is_ok())
}