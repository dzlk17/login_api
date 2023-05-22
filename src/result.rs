///New Result type.
/// 
///Result type created to make error handling easier. Allows to return a result of a generic type `T` and any error that can be converted to a Box<Error>.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;