use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};

/// Used to store any information that will be store in the token. - User email and expiration date;
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub exp: u64,
}
/// Used to store keys for encoding and decoding the JWT token.
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

/// Implements method for the Keys struct.
impl Keys {

    /// Function is a constructor method for the Keys struct.
    /// 
    /// ## Arguments
    /// secret - Function takes user secret key in bytes.
    pub fn new(secret: &[u8]) -> Self{
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

/// Function give timestamp 
/// 
/// Function set tocken expirtion date to one hour from the time of invocation.
pub fn get_timestamp() -> u64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    now.as_secs() + 3600
}