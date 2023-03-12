use serde::{Deserialize, Serialize};
use validator::{ValidationError, Validate};

#[derive(Deserialize, Serialize)]
pub struct SingIn {
    pub email: String,
    pub passwd: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct SingUp {
    #[validate(length(min=2), custom = "validate_unique_name")]
    pub name: String,
    #[validate(email)]
    pub email: String,
    // The password you entered is not valid Please note that the password must respect the following rules:
    // It must contain between 6 and 32 characters.
    // TODO Use only characters from the following set: ! # $ % & ( ) * + , - . / 0123456789 : ; < = > ? @ ABCDEFGHIJKLMNOPQRSTUVWXYZ [ \ ] _ ` abcdefghijklmnopqrstuvwxyz { | }
    // TODO It must contain at least 1 letter(s) (ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz).
    // TODO It must contain at least 1 numeric character(s) (0123456789).
    // TODO must not contain more than 2 identical consecutive characters (AAA, iiii, $$$$$ ...).

    #[validate(length(min = 6))]
    #[validate(length(max = 32))]
    pub passwd: String,
}

fn validate_unique_name(name: &str) -> Result<(), ValidationError> {
    if name == "xx" {
        return Err(ValidationError::new("terrible_username"))
    }
    Ok(())
}
