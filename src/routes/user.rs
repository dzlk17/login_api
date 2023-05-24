use argon2::Config;
use serde::Deserialize;

///Struct to change json into struct stores user data.
#[derive(Deserialize)]
pub struct User{
    pub email: String,
    pub password: String,
}

///The function returns hashed password.
/// 
/// ## Arguments
/// *password - function get reference to password
/// 
/// The function hashed password using argon2 alghoritm.
pub fn hash_password(password: &str) -> String{
    let salt = b"randomsalt";
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap()
}

///The function verifies password.
/// 
/// ## Arguments
/// *password - function get reference to password
/// *hash - hashed password saved in database
/// 
/// The function returns true or false depending on whether the passwords are identical. 
pub fn verify_password(password: &str, hash: String) -> bool{
    argon2::verify_encoded(&hash, password.as_bytes()).unwrap()
}