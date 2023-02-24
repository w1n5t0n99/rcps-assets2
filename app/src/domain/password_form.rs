use secrecy::{Secret, ExposeSecret};
use validator::{Validate, ValidationError, validate_length, validate_must_match};


fn validate(value: &PasswordForm) -> Result<(), ValidationError> {
    if validate_must_match(value.new_password.expose_secret(), value.new_password_check.expose_secret()) == false {
        return Err(ValidationError::new("new passwords do not match"));
    }

    if validate_length(value.new_password.expose_secret(), Some(8), None, None) == false {
        let mut e = ValidationError::new("new passwords not long enough");
        e.add_param(std::borrow::Cow::Borrowed("new_password"), &"error".to_string());
        return Err(e);

    }
    
    Ok(())
}

#[derive(serde::Deserialize, Validate, Clone)]
#[validate(schema(function = "validate"))]
pub struct PasswordForm {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}