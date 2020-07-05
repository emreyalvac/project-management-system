use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::de::DeserializeOwned;

static SECRET_KEY: &str = "d41d8cd98f00b204e9800998ecf8427e";

pub fn token_decoder<T>(header: String) -> Result<T, bool> where T: DeserializeOwned  {
    let key = SECRET_KEY.as_bytes();
    let split: Vec<&str> = header.split("Bearer").collect();
    let token = split[1].trim();
    let decoded = decode::<T>(token, &DecodingKey::from_secret(key), &Validation::new(Algorithm::HS256));
    match decoded {
        Ok(data) => {
            Ok(data.claims)
        }
        Err(_) => {
            Err(false)
        }
    }
}
