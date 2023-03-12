
use validator::{Validate, ValidationError};

pub mod auth;
pub mod data;



pub fn validate_payload<T: Validate>(payload: &T) -> Result<(), ValidationError> {
    Ok(payload.validate().unwrap())
}