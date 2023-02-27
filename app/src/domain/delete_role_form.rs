use validator::{Validate, ValidationError};


fn validate(value: &DeleteRoleForm) -> Result<(), ValidationError> {
    if value._method.eq("delete") {
        if value.row_id.eq("admin") || value.row_id.eq("inactvie") {
            return Err(ValidationError::new("invalid_role"));
        }
    }
    
    Ok(())
}

#[derive(serde::Deserialize, Validate, Clone)]
#[validate(schema(function = "validate"))]
pub struct DeleteRoleForm {
    pub _method: String, 
    pub row_id: String,
}

