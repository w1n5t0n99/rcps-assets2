use secrecy::{Secret, ExposeSecret};
use validator::{Validate, ValidationError, validate_length, validate_must_match};


fn validate_password_length(value: &PasswordForm) -> Result<(), ValidationError> {
    if validate_length(value.new_password.expose_secret(), Some(8), None, None) == false {
        return Err(ValidationError::new("password_length"));
    }
    
    Ok(())
}

fn validate_password_match(value: &PasswordForm) -> Result<(), ValidationError> {
    if validate_must_match(value.new_password.expose_secret(), value.new_password_check.expose_secret()) == false {
        return Err(ValidationError::new("password_match"));
    }    
    Ok(())
}

#[derive(serde::Deserialize, Validate, Clone)]
#[validate(schema(function = "validate_password_length"))]
#[validate(schema(function = "validate_password_match"))]
pub struct PasswordForm {
    pub current_password: Secret<String>,
    pub new_password: Secret<String>,
    pub new_password_check: Secret<String>,
}